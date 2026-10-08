//! admin-storage 端点：查询单个存储。
//!
//! 对应 `GET /api/admin/storage/get`；按 `id` 查询参数获取存储详情，
//! 响应 `data` 为单个 [`Storage`]。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::storage::Storage;

/// 查询存储详情请求构建器。
///
/// 通过 [`Storage::get`](super::Storage::get) 创建；`id` 为必选参数。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/storage/get", model = Storage)]
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
    /// 查询指定 ID 的存储详情。
    ///
    /// 对应 AList `GET /api/admin/storage/get`；`id` 经 URL 查询串传递
    /// （服务端按 `strconv.Atoi` 解析）。
    /// 数据来源：AList OpenAPI 规范的 `/api/admin/storage/get` 与
    /// AList 服务端 handles.Storage 模块（实现为 `GetStorage`）。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标存储 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Storage`]。
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
    /// let storage = client.admin().storage().get(2).await?;
    /// println!("{}: {}", storage.mount_path, storage.driver);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：GET + `id` 查询参数。
    #[test]
    fn build_request_composes_get_url_with_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 5).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/get"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=5"), "URL 应包含存储 ID 查询参数: {url}");
    }

    /// 收发路径断言：mock 服务器返回 AList OpenAPI 获取存储详情示例，解码 `Storage`。
    #[tokio::test]
    async fn send_decodes_storage_detail() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 示例 JSON：AList OpenAPI 规范 /api/admin/storage/get 响应示例
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":2,"mount_path":"/aa","order":1,"driver":"Aliyundrive","cache_expiration":30,"status":"work","addition":"{\"root_folder_id\":\"\",\"refresh_token\":\"\"}","remark":"","modified":"2022-11-26T21:50:44.142348853+08:00","disabled":false,"order_by":"","order_direction":"","extract_folder":"front","web_proxy":false,"webdav_policy":"302_redirect","down_proxy_url":""}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let storage = Request::new(&client, 2).send().await.unwrap();
        assert_eq!(storage.id, 2);
        assert_eq!(storage.mount_path, "/aa");
        assert_eq!(storage.driver, "Aliyundrive");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/admin/storage/get?id=2 "),
            "{}",
            recorded[0]
        );
    }
}
