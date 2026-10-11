//! auth 端点：为当前用户添加 SSH 公钥。
//!
//! 对应 `POST /api/me/sshkey/add`；请求体为标题与公钥文本，成功时响应
//! `data: null`，以 `()` 作为端点模型。该端点不在 AList OpenAPI 规范中，
//! 请求形状以 AList 服务端路由定义与 sshkey 处理模块为准。
//! 公钥格式非法或标题为空时返回响应 400。
//!
//! # Version
//!
//! - 自 AList `v3.42.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 添加当前用户 SSH 公钥请求构建器。
///
/// 通过 [`Auth::me_ssh_key_add`](super::Auth::me_ssh_key_add) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/me/sshkey/add", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 密钥标题 / 备注名（必选参数）。
    title: String,
    /// SSH 公钥文本（必选参数）。
    ///
    /// 须为服务端可解析的 OpenSSH 授权密钥格式（如 `ssh-ed25519 AAAA...`），
    /// 服务端解析后仅保留指纹；格式非法时返回响应 400。
    key: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(
        client: &'a crate::Client,
        title: impl Into<String>,
        key: impl Into<String>,
    ) -> Self {
        Self {
            client,
            title: title.into(),
            key: key.into(),
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 为当前用户添加 SSH 公钥。
    ///
    /// 对应 AList `POST /api/me/sshkey/add`（数据来源：AList 服务端路由定义与
    /// AList 服务端 sshkey 处理模块的 `AddMyPublicKey`；该端点不在 AList OpenAPI
    /// 规范中）。请求体为标题与公钥文本，服务端解析公钥后存储指纹；
    /// 成功时响应 `data` 为 `null`。公钥格式非法或标题为空时返回响应 400。
    ///
    /// # Arguments
    ///
    /// * `title` - 密钥标题 / 备注名。
    /// * `key` - SSH 公钥文本（OpenSSH 授权密钥格式）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如请求体缺失字段或标题为空返回 400、公钥格式非法返回 400）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.42.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client
    ///     .auth()
    ///     .me_ssh_key_add("laptop", "ssh-ed25519 AAAA... user@host")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn me_ssh_key_add(&self, title: impl Into<String>, key: impl Into<String>) -> Request<'a> {
        Request::new(self.client, title, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "laptop", "ssh-ed25519 AAAA test")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/me/sshkey/add"), "URL 应包含路径: {url}");
        assert!(!url.contains("title="), "请求体字段不应出现在查询串: {url}");

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"title\":\"laptop\""), "{body}");
        assert!(body.contains("\"key\":\"ssh-ed25519 AAAA test\""), "{body}");
    }

    /// 收发路径：mock 服务器返回 `data: null`，断言请求原文与 `()` 解码。
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
            .with_authentication(crate::Authentication::Token("test-token".to_owned()));

        client
            .auth()
            .me_ssh_key_add("laptop", "ssh-ed25519 AAAA test")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/me/sshkey/add "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: test-token"),
            "应注入认证头: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"title\":\"laptop\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"key\":\"ssh-ed25519 AAAA test\""),
            "{}",
            recorded[0]
        );
    }

    /// 收发路径：公钥格式非法时服务端返回 HTTP 200 + 信封 `code` 400，
    /// 断言客户端将其映射为 API 错误。
    #[tokio::test]
    async fn send_maps_invalid_key_to_api_error() {
        use crate::test_support::{MockResponse, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 200 OK",
                body: r#"{"code":400,"message":"provided key invalid","data":null}"#.to_owned(),
            }],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let err = Request::new(&client, "bad", "not-a-key")
            .send()
            .await
            .unwrap_err();
        match err {
            crate::Error::Api { code, message, .. } => {
                assert_eq!(code.as_i32(), 400, "{code:?}");
                assert_eq!(message, "provided key invalid");
            }
            other => panic!("expected Api, got {other:?}"),
        }
    }
}
