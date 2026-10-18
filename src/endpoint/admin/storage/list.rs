//! admin-storage 端点：列出存储。
//!
//! 对应 `GET /api/admin/storage/list`；响应 `data` 为 `PageResponse<Storage>`
//! 分页包裹结构（`{"content": [...], "total": N}`）。不传分页参数时服务端
//! 一次返回全部存储（AList 服务端的 `PageRequest::Validate`
//! 会把缺省 `per_page` 放大为最大整数值）。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.15.0` 起，响应存储对象新增 `enable_sign` 字段（细粒度链接签名开关）；
//!   老版本不返回该字段。
//! - 从 AList `v3.35.0` 起，响应存储对象新增 `proxy_range` 字段（Range 请求代理开关）；
//!   老版本不返回该字段。
//! - 从 AList `v3.42.0` 起，响应存储对象新增 `disable_index` 字段（存储级禁止建立索引开关）；
//!   老版本不返回该字段。
//! - 从 AList `v3.56.0` 起，响应存储对象新增 `down_proxy_sign` 字段（下载代理 URL 附加签名开关）；
//!   老版本不返回该字段。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::storage::Storage, common::PageResponse};

/// 列出存储请求构建器。
///
/// 通过 [`Storage::list`](super::Storage::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/storage/list", model = PageResponse<Storage>)]
#[cfg_attr(feature = "into-stream", endpoint(into_stream = true, stream_item = Storage))]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（可选参数）。
    ///
    /// 缺省时服务端按第 1 页、不限条数处理，即返回全部存储。
    #[query]
    page: Option<i32>,
    /// 每页条数（可选参数）。
    ///
    /// 缺省时服务端返回全部存储。
    #[query]
    per_page: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Storage<'a> {
    /// 列出全部存储。
    ///
    /// 对应 AList `GET /api/admin/storage/list`；响应 `data` 为
    /// `{ "content": [Storage], "total": N }` 分页包裹结构，不传 `page`/`per_page`
    /// 时服务端一次返回全部存储。
    /// 数据来源：AList OpenAPI 规范的 `/api/admin/storage/list` 与
    /// AList 服务端 handles.Storage 模块（实现为 `ListStorages`）。
    ///
    /// # Arguments
    ///
    /// 无必选参数；可选的 `page`/`per_page` 通过返回的 [`Request`] 链式设置。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse`] 包裹的
    /// [`Storage`] 列表。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.15.0` 起，响应存储对象新增 `enable_sign` 字段（细粒度链接签名开关）；
    ///   老版本不返回该字段。
    /// - 从 AList `v3.35.0` 起，响应存储对象新增 `proxy_range` 字段（Range 请求代理开关）；
    ///   老版本不返回该字段。
    /// - 从 AList `v3.42.0` 起，响应存储对象新增 `disable_index` 字段（存储级禁止建立索引开关）；
    ///   老版本不返回该字段。
    /// - 从 AList `v3.56.0` 起，响应存储对象新增 `down_proxy_sign` 字段（下载代理 URL 附加签名开关）；
    ///   老版本不返回该字段。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let storages = client.admin().storage().list().await?;
    /// println!("共 {} 个存储", storages.total);
    /// for storage in storages.content {
    ///     println!("{} ({})", storage.mount_path, storage.driver);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// 启用 `into-stream` feature 后，还可调用
    /// [`.into_stream()`](Request::into_stream) 逐页流式产出存储。
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：缺省（无分页参数）时为 GET 且不带查询串。
    #[test]
    fn build_request_composes_get_url_without_query_by_default() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "缺省时不应携带查询参数: {url}");
    }

    /// 可选分页参数经链式 setter 进入查询串。
    #[test]
    fn build_request_applies_optional_pagination_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .per_page(30)
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(url.contains("page=2"), "URL 应包含页码: {url}");
        assert!(url.contains("per_page=30"), "URL 应包含每页条数: {url}");
    }

    /// 收发路径断言：mock 服务器返回 openapi 列表示例，解码 `PageResponse<Storage>`。
    #[tokio::test]
    async fn send_decodes_paginated_storage_list() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":1,"mount_path":"/lll","order":0,"driver":"Local","cache_expiration":0,"status":"work","addition":"{}","remark":"","modified":"2023-07-19T09:46:38.868739912+08:00","disabled":false,"enable_sign":false,"order_by":"name","order_direction":"asc","extract_folder":"front","web_proxy":false,"webdav_policy":"native_proxy","down_proxy_url":""}],"total":5}}"#;
        let mock = server
            .mock("GET", "/api/admin/storage/list")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let page = Request::new(&client).send().await.unwrap();
        assert_eq!(page.total, 5);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].id, 1);
        assert_eq!(page.content[0].mount_path, "/lll");

        mock.assert_async().await;
    }

    /// into-stream 翻页断言：逐页 `page=N` 请求，空页停止（含一次确认请求）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use futures::StreamExt;

        let mut server = mockito::Server::new_async().await;
        let mock1 = server
            .mock("GET", "/api/admin/storage/list?page=1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"content":[{"id":1,"mount_path":"/lll","order":0,"driver":"Local","cache_expiration":0,"status":"work","addition":"{}","remark":"","modified":"2023-07-19T09:46:38.868739912+08:00","disabled":false,"order_by":"name","order_direction":"asc","extract_folder":"front","web_proxy":false,"webdav_policy":"native_proxy","down_proxy_url":""}],"total":2}}"#,
            )
            .create_async()
            .await;
        let mock2 = server
            .mock("GET", "/api/admin/storage/list?page=2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"content":[{"id":2,"mount_path":"/aa","order":1,"driver":"Aliyundrive","cache_expiration":30,"status":"work","addition":"{}","remark":"","modified":"2022-11-26T21:50:44.142348853+08:00","disabled":false,"order_by":"","order_direction":"","extract_folder":"front","web_proxy":false,"webdav_policy":"302_redirect","down_proxy_url":""}],"total":2}}"#,
            )
            .create_async()
            .await;
        let mock3 = server
            .mock("GET", "/api/admin/storage/list?page=3")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":{"content":[],"total":2}}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let items: Vec<Storage> = Request::new(&client)
            .into_stream()
            .map(|item| item.unwrap())
            .collect::<Vec<_>>()
            .await;
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].mount_path, "/lll");
        assert_eq!(items[1].mount_path, "/aa");

        mock1.assert_async().await;
        mock2.assert_async().await;
        mock3.assert_async().await;
    }
}
