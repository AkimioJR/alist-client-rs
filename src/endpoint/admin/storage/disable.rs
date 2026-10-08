//! admin-storage 端点：禁用存储。
//!
//! 对应 `POST /api/admin/storage/disable`；按 `id` 查询参数禁用存储，
//! 响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 禁用存储请求构建器。
///
/// 通过 [`Storage::disable`](super::Storage::disable) 创建；`id` 为必选参数。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/storage/disable", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标存储 ID（必选参数）。
    ///
    /// 经 URL 查询参数传递。
    #[query]
    id: u64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, id: u64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::Storage<'a> {
    /// 禁用指定 ID 的存储。
    ///
    /// 对应 AList `POST /api/admin/storage/disable`；`id` 经 URL 查询串传递，
    /// 成功时响应 `data` 为 `null`。
    /// 数据来源：AList OpenAPI 规范的 `/api/admin/storage/disable` 与
    /// AList 服务端 handles.Storage 模块（实现为 `DisableStorage`）。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标存储 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如存储不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().storage().disable(6).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn disable(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：POST + `id` 查询参数。
    #[test]
    fn build_request_posts_disable_url_with_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 6).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/disable"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=6"), "URL 应包含存储 ID 查询参数: {url}");
    }

    /// 收发路径断言：mock 服务器返回 `data: null`，解码为 `()`。
    #[tokio::test]
    async fn send_disables_storage_and_decodes_unit() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 6).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/storage/disable?id=6 "),
            "{}",
            recorded[0]
        );
    }
}
