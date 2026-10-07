//! auth 端点：生成两步验证密钥。
//!
//! 对应 `POST /api/auth/2fa/generate`；无需请求体，返回二维码 data URL 与
//! TOTP 密钥（`examples/alist/server/handles/auth.go:216-243` 的 `Generate2FA`）。
//! 生成的密钥须经 [`super::verify_2fa`] 校验后才正式启用。游客账号调用返回 403。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::Generate2FaResp;

/// 生成两步验证密钥请求构建器。
///
/// 通过 [`Auth::generate_2fa`](super::Auth::generate_2fa) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/2fa/generate", model = Generate2FaResp)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Auth<'a> {
    /// 生成两步验证密钥与二维码。
    ///
    /// 对应 AList `POST /api/auth/2fa/generate`（数据来源：
    /// `docs/api/alistv3.openapi.yaml` 的 `auth/2fa/generate` 与
    /// `examples/alist/server/handles/auth.go` 的 `Generate2FA`，
    /// 实现于 auth.go:216-243）。成功时返回 [`Generate2FaResp`]：
    /// `qr` 为 PNG 二维码的 data URL，`secret` 为 TOTP 密钥。
    /// 密钥需经 [`verify_2fa`](super::Auth::verify_2fa) 校验后才会绑定到账号。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Generate2FaResp`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 如游客账号调用返回 403）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let generated = client.auth().generate_2fa().await?;
    /// println!("secret: {}, qr: {}", generated.secret, generated.qr);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn generate_2fa(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL 断言；无请求体字段时不应携带 JSON body。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/auth/2fa/generate"),
            "URL 应包含路径: {url}"
        );
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器 + 记录请求原文，解码生成密钥响应。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_generated_2fa() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"qr":"data:image/png;base64,iVBORw0KGgoAAAANSUhE","secret":"RPQZG4MDS3"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let generated = client.auth().generate_2fa().send().await.unwrap();
        assert_eq!(generated.secret, "RPQZG4MDS3");
        assert!(generated.qr.starts_with("data:image/png;base64,"));

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/auth/2fa/generate "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: token-1"),
            "应注入认证头: {}",
            recorded[0]
        );
    }
}
