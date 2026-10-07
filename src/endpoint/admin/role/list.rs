//! admin-role 端点：列出角色。
//!
//! 对应 `GET /api/admin/role/list`（openapi 未收录该分组，路由见
//! `examples/alist/server/router.go:150`）。处理函数 `handles.ListRoles`
//! 绑定 `model.PageReq`（查询串 `page`/`per_page`），返回
//! `common.PageResp{Content: roles, Total: total}`；
//! `per_page` 缺省（或小于 1）时服务端返回全部角色
//! （见 `examples/alist/server/handles/role.go:14-28` 与 `internal/model/req.go:13-19`）。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::role::Role, common::PageResp};

/// 列出角色请求构建器。
///
/// 通过 [`Role::list`](super::Role::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/role/list", model = PageResp<Role>)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = Role)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码，从 1 开始；缺省时服务端按第 1 页处理。
    #[query]
    page: Option<i32>,
    /// 每页条数；缺省（或小于 1）时服务端返回全部角色。
    #[query]
    per_page: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Role<'a> {
    /// 列出全部角色（分页）。
    ///
    /// 对应 AList `GET /api/admin/role/list`；响应 `data` 为
    /// `{ "content": [角色...], "total": 总数 }` 分页结构。
    /// 数据来源：`examples/alist/server/router.go:150`（`handles.ListRoles`）与
    /// `examples/alist/server/handles/role.go:14-28`；该分组不在 openapi 中，以 Go 源码为准。
    ///
    /// # Arguments
    ///
    /// * `page` - 可选：页码，从 1 开始。
    /// * `per_page` - 可选：每页条数；缺省时服务端一次性返回全部角色。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResp`](crate::schema::common::PageResp)`<`[`Role`](crate::schema::admin::role::Role)`>`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let page = client.admin().role().list().page(1).per_page(20).await?;
    /// for role in &page.content {
    ///     println!("{}: {}", role.id, role.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法与路径、查询参数齐全。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .per_page(10)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/role/list"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("page=2"), "URL 应包含页码: {url}");
        assert!(url.contains("per_page=10"), "URL 应包含每页条数: {url}");
    }

    /// 请求形状：可选查询参数缺省时不应出现在 URL 中。
    #[test]
    fn build_request_skips_none_query_params() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("page="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("per_page="), "None 查询参数应被跳过: {url}");
    }

    /// 收发路径：mock 服务器返回 `PageResp<Role>`，断言请求行与解码结果。
    #[tokio::test]
    async fn send_gets_role_page_and_decodes_content() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 示例形状取自 handles/role.go 返回的 PageResp 与 internal/model/role.go 的 JSON 标签
        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":1,"name":"admin","description":"","default":true,"permission_scopes":[{"path":"/","permission":65535}]}],"total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client)
            .page(1)
            .per_page(10)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].name, "admin");
        assert_eq!(page.content[0].permission_scopes[0].permission, 65535);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/role/list?page=1&per_page=10"),
            "{}",
            recorded[0]
        );
    }

    /// into-stream 翻页：逐页产出 `content` 元素，空页停止（含一次确认请求）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use std::sync::{Arc, Mutex};

        use futures::StreamExt;

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":1,"name":"admin","description":"","default":true,"permission_scopes":null}],"total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":2,"name":"guest","description":"","default":false,"permission_scopes":[]}],"total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[],"total":2}}"#,
                ),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let names: Vec<String> = Request::new(&client)
            .per_page(1)
            .into_stream()
            .map(|item| item.unwrap().name)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(names, vec!["admin".to_owned(), "guest".to_owned()]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 3); // 空页会多一次确认请求
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
        assert!(recorded[2].contains("page=3"), "{}", recorded[2]);
    }
}
