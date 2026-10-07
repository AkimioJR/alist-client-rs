//! admin-storage 端点：删除存储。
//!
//! 对应 `POST /api/admin/storage/delete`；按 `id` 查询参数删除存储，
//! 响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 删除存储请求构建器。
///
/// 通过 [`Storage::delete`](super::Storage::delete) 创建；`id` 为必选参数。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/storage/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标存储 ID（必选）。
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
    /// 删除指定 ID 的存储。
    ///
    /// 对应 AList `POST /api/admin/storage/delete`；`id` 经 URL 查询串传递，
    /// 成功时响应 `data` 为 `null`。注意：openapi 将 `id` 标记为可选，但服务端
    /// 实现按 `strconv.Atoi(c.Query("id"))` 解析（`server/handles/storage.go`
    /// 的 `DeleteStorage`），缺失时返回 400，故客户端将其建模为必选参数。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/storage/delete` 与
    /// `examples/alist/server/handles/storage.go`。
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
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
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
    /// client.admin().storage().delete(2).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn delete(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：POST + `id` 查询参数。
    #[test]
    fn build_request_posts_delete_url_with_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 2).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=2"), "URL 应包含存储 ID 查询参数: {url}");
    }

    /// 收发路径断言：mock 服务器返回 `data: null`，解码为 `()`。
    #[tokio::test]
    async fn send_deletes_storage_and_decodes_unit() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 2).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/storage/delete?id=2 "),
            "{}",
            recorded[0]
        );
    }
}
