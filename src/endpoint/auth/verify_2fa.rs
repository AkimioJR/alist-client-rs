//! auth 端点：验证并启用两步验证。
//!
//! 对应 `POST /api/auth/2fa/verify`；请求体为 TOTP 验证码与
//! [`generate_2fa`](super::generate_2fa) 返回的密钥（`examples/alist/server/handles/auth.go`
//! 的 `Verify2FAReq`，auth.go:245-248），校验通过后服务端把密钥绑定到当前账号，
//! 成功时响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 验证两步验证码请求构建器。
///
/// 通过 [`Auth::verify_2fa`](super::Auth::verify_2fa) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/2fa/verify", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 当前 TOTP 验证码（必选）。
    code: String,
    /// 待启用的 2FA 密钥（必选）。
    secret: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(
        client: &'a crate::Client,
        code: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        Self {
            client,
            code: code.into(),
            secret: secret.into(),
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 验证两步验证码并启用 2FA。
    ///
    /// 对应 AList `POST /api/auth/2fa/verify`（数据来源：
    /// `docs/api/alistv3.openapi.yaml` 的 `auth/2fa/verify` 与
    /// `examples/alist/server/handles/auth.go` 的 `Verify2FA`，
    /// 实现于 auth.go:250-271）。校验通过后服务端将密钥写入当前账号，
    /// 成功时响应 `data` 为 `null`；验证码错误返回响应 400。
    ///
    /// # Arguments
    ///
    /// * `code` - 用户从认证器 App 中读取的当前 TOTP 验证码。
    /// * `secret` - [`generate_2fa`](super::Auth::generate_2fa) 返回的 2FA 密钥。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如验证码错误返回 400、游客账号调用返回 403）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.auth().verify_2fa("123456", "RPQZG4MDS3").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn verify_2fa(&self, code: impl Into<String>, secret: impl Into<String>) -> Request<'a> {
        Request::new(self.client, code, secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "123456", "RPQZG4MDS3")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/auth/2fa/verify"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"code\":\"123456\""), "{body}");
        assert!(body.contains("\"secret\":\"RPQZG4MDS3\""), "{body}");
    }

    /// 收发路径：mock 服务器返回 `data: null`，以 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        client
            .auth()
            .verify_2fa("123456", "RPQZG4MDS3")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/auth/2fa/verify "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"code\":\"123456\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"secret\":\"RPQZG4MDS3\""),
            "{}",
            recorded[0]
        );
    }
}
