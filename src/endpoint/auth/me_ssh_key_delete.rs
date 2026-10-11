//! auth 端点：删除当前用户的 SSH 公钥。
//!
//! 对应 `POST /api/me/sshkey/delete`；以必选查询参数 `id` 指定密钥 ID，
//! 响应 `data: null`，以 `()` 作为端点模型。该端点不在 AList OpenAPI 规范中，
//! 路径以 AList 服务端路由定义为准。密钥不存在时返回响应 404。
//!
//! # Version
//!
//! - 自 AList `v3.42.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 删除当前用户 SSH 公钥请求构建器。
///
/// 通过 [`Auth::me_ssh_key_delete`](super::Auth::me_ssh_key_delete) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/me/sshkey/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标密钥 ID（必选参数）。
    ///
    /// 作为 URL 查询参数 `id` 发送。
    #[query]
    id: u64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点只有密钥 ID 必选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, id: u64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::Auth<'a> {
    /// 删除当前用户的指定 SSH 公钥。
    ///
    /// 对应 AList `POST /api/me/sshkey/delete`（数据来源：AList 服务端路由定义与
    /// AList 服务端 sshkey 处理模块的 `DeleteMyPublicKey`；该端点不在 AList
    /// OpenAPI 规范中）。服务端仅允许删除属于当前用户的密钥；
    /// 成功时响应 `data` 为 `null`。密钥不存在或不属于当前用户时返回响应 404。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标密钥 ID（必选参数）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如查询参数 `id` 格式非法返回 400、密钥不存在返回 404）时，
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
    /// client.auth().me_ssh_key_delete(1).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn me_ssh_key_delete(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状：POST /api/me/sshkey/delete，查询串携带 id，无请求体。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 7).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/me/sshkey/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=7"), "URL 应包含查询参数 id: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );
    }

    /// 收发路径：mock 服务器返回 `data: null`，断言请求原文与 `()` 解码。
    #[tokio::test]
    async fn send_deletes_ssh_public_key() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("test-token".to_owned()));

        client.auth().me_ssh_key_delete(7).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/me/sshkey/delete?id=7"),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: test-token"),
            "应注入认证头: {}",
            recorded[0]
        );
    }

    /// 收发路径：密钥不存在时服务端返回 HTTP 200 + 信封 `code` 404，
    /// 断言客户端将其映射为 API 错误。
    #[tokio::test]
    async fn send_maps_missing_key_to_api_error() {
        use crate::test_support::{MockResponse, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 200 OK",
                body: r#"{"code":404,"message":"failed to get public key","data":null}"#.to_owned(),
            }],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let err = client
            .auth()
            .me_ssh_key_delete(999)
            .send()
            .await
            .unwrap_err();
        match err {
            crate::Error::Api { code, message, .. } => {
                assert_eq!(code.as_i32(), 404, "{code:?}");
                assert_eq!(message, "failed to get public key");
            }
            other => panic!("expected Api, got {other:?}"),
        }
    }
}
