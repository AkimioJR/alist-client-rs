//! auth 端点：哈希登录获取 token。
//!
//! 对应 `POST /api/auth/login/hash`；与 [`super::login`] 的唯一差别是请求体中的
//! `password` 必须是预哈希值：先拼接 `-https://github.com/alist-org/alist` 后缀，
//! 再取 SHA-256 十六进制字符串（盐常量见 `examples/alist/internal/model/user.go:23`
//! 的 `StaticHashSalt`，哈希方式见 user.go:183-185 的 `StaticHash`）。
//! 服务端对两种登录的处理逻辑一致（auth.go:41-59）。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::LoginResponse;

/// 哈希登录请求构建器。
///
/// 通过 [`Auth::login_hash`](super::Auth::login_hash) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/login/hash", model = LoginResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 用户名（必选）。
    username: String,
    /// 预哈希密码（必选）：`sha256(密码-https://github.com/alist-org/alist)`。
    password: String,
    /// 两步验证码（可选）；账号启用 2FA 时必填。
    otp_code: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(
        client: &'a crate::Client,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            client,
            username: username.into(),
            password: password.into(),
            otp_code: None,
        }
    }
}

#[cfg(feature = "auth")]
impl<'a> super::Auth<'a> {
    /// 以预哈希密码登录获取临时 token。
    ///
    /// 对应 AList `POST /api/auth/login/hash`；请求体中的 `password` 必须为
    /// `sha256(密码-https://github.com/alist-org/alist)` 的十六进制字符串
    /// （数据来源：`docs/api/alistv3.openapi.yaml` 的 `auth/login/hash` 与
    /// `examples/alist/server/handles/auth.go` 的 `LoginHash`，实现于 auth.go:52-59，
    /// 盐常量为 `examples/alist/internal/model/user.go:23` 的 `StaticHashSalt`）。
    /// 成功时返回 [`LoginResponse`]。
    /// 明文密码的变体见 [`login`](super::Auth::login)。
    ///
    /// # Arguments
    ///
    /// * `username` - 用户名。
    /// * `password` - 预哈希密码：先拼接 `-https://github.com/alist-org/alist`
    ///   后缀，再取 SHA-256 十六进制字符串。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`LoginResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如凭据错误返回 400、两步验证码缺失或错误返回 402）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// // password_hash = sha256("password-https://github.com/alist-org/alist")（十六进制）
    /// let password_hash = "…";
    /// let login = client.auth().login_hash("admin", password_hash)
    ///     .otp_code("123456") // 可选：两步验证码
    ///     .await?;
    /// println!("token: {}", login.token);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn login_hash(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, username, password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言（哈希登录路径与请求体键名）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "admin", "hashed-password")
            .otp_code("123456")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/auth/login/hash"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"username\":\"admin\""), "{body}");
        assert!(body.contains("\"password\":\"hashed-password\""), "{body}");
        assert!(body.contains("\"otp_code\":\"123456\""), "{body}");
    }

    /// 收发路径：mock 服务器 + 记录请求原文。
    #[tokio::test]
    async fn send_posts_expected_request() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"token":"abcd"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let login = Request::new(&client, "admin", "hashed-password")
            .send()
            .await
            .unwrap();
        assert_eq!(login.token, "abcd");
        assert_eq!(login.device_key, None, "老服务器响应不含 device_key");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/auth/login/hash "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"username\":\"admin\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"password\":\"hashed-password\""),
            "{}",
            recorded[0]
        );
        assert!(!recorded[0].contains("otp_code"), "{}", recorded[0]);
    }
}
