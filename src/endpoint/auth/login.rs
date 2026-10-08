//! auth 端点：登录获取 token。
//!
//! 对应 `POST /api/auth/login`；请求体为用户名、明文密码（服务端会做静态盐
//! SHA-256 哈希，`examples/alist/server/handles/auth.go:47` 的 `model.StaticHash`）
//! 与可选两步验证码，成功返回 token 等登录信息。
//! 哈希登录变体见 [`super::login_hash`]。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::LoginResponse;

/// 登录请求构建器。
///
/// 通过 [`Auth::login`](super::Auth::login) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/login", model = LoginResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 用户名（必选）。
    username: String,
    /// 明文密码（必选；服务端做静态盐 SHA-256 哈希）。
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

impl<'a> super::Auth<'a> {
    /// 登录获取临时 token。
    ///
    /// 对应 AList `POST /api/auth/login`；密码以明文发送，由服务端做静态盐
    /// SHA-256 哈希（数据来源：`docs/api/alistv3.openapi.yaml` 的 `auth/login`
    /// 与 `examples/alist/server/handles/auth.go` 的 `Login`，实现于 auth.go:41-49）。
    /// 成功时返回 [`LoginResponse`]（含 token；新版本服务端还返回 `device_key`）。
    /// 该端点是 [`Client`](crate::Client) 内部自动刷新 token 所用登录的公开入口。
    /// 预哈希密码的变体见 [`login_hash`](super::Auth::login_hash)。
    ///
    /// # Arguments
    ///
    /// * `username` - 用户名。
    /// * `password` - 明文密码。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`LoginResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如用户名或密码错误返回 400、两步验证码缺失或错误返回 402）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let login = client.auth().login("admin", "password")
    ///     .otp_code("123456") // 可选：两步验证码
    ///     .await?;
    /// println!("token: {}", login.token);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn login(&self, username: impl Into<String>, password: impl Into<String>) -> Request<'a> {
        Request::new(self.client, username, password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体（含可选 otp_code）断言。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "admin", "password")
            .otp_code("123456")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/auth/login"), "URL 应包含路径: {url}");
        assert!(
            !url.contains("username="),
            "请求体字段不应出现在查询串: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"username\":\"admin\""), "{body}");
        assert!(body.contains("\"password\":\"password\""), "{body}");
        assert!(body.contains("\"otp_code\":\"123456\""), "{body}");
    }

    /// 请求形状：可选参数缺省时请求体不含 `otp_code` 键。
    #[test]
    fn build_request_skips_absent_otp_code() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "admin", "password")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(!body.contains("otp_code"), "{body}");
    }

    /// 收发路径：mock 服务器 + 记录请求原文，解码登录响应。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_login_response() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"token":"abcd","device_key":"key-1"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let login = client
            .auth()
            .login("admin", "password")
            .otp_code("123456")
            .send()
            .await
            .unwrap();
        assert_eq!(login.token, "abcd");
        assert_eq!(login.device_key.as_deref(), Some("key-1"));

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/auth/login "),
            "{}",
            recorded[0]
        );
        assert!(
            !recorded[0].contains("/api/auth/login/hash"),
            "不应误用哈希登录路径: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"username\":\"admin\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"password\":\"password\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"otp_code\":\"123456\""),
            "{}",
            recorded[0]
        );
    }
}
