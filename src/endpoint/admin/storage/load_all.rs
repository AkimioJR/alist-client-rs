//! admin-storage 端点：重新加载全部存储。
//!
//! 对应 `POST /api/admin/storage/load_all`；触发服务端从数据库重新加载
//! （卸载并重挂）全部已启用的存储驱动，响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 重新加载全部存储请求构建器。
///
/// 通过 [`Storage::load_all`](super::Storage::load_all) 创建；无任何参数。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/storage/load_all", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Storage<'a> {
    /// 从数据库重新加载全部已启用的存储驱动。
    ///
    /// 对应 AList `POST /api/admin/storage/load_all`；服务端在后台协程中逐个
    /// 卸载并重挂存储（`LoadAllStorages`），本请求立即返回。
    /// 数据来源：AList OpenAPI 规范的 `/api/admin/storage/load_all` 与
    /// AList 服务端 handles.Storage 模块（实现为 `LoadAllStorages`）。
    ///
    /// # Arguments
    ///
    /// 无参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().storage().load_all().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn load_all(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：POST、无查询串、无请求体。
    #[test]
    fn build_request_posts_load_all_url_without_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/load_all"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "不应携带查询参数: {url}");
        assert!(built.body().is_none(), "无请求体字段的请求不应携带 body");
    }

    /// 收发路径断言：请求不携带 JSON Content-Type（无 body 字段）。
    #[tokio::test]
    async fn send_posts_without_json_content_type() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/storage/load_all "),
            "{}",
            recorded[0]
        );
        assert!(
            !recorded[0]
                .to_ascii_lowercase()
                .contains("content-type: application/json"),
            "无请求体字段的请求不应携带 JSON Content-Type: {}",
            recorded[0]
        );
    }
}
