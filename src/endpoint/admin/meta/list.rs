//! admin-meta 端点：列出元信息。
//!
//! 对应 `GET /api/admin/meta/list`；查询参数 `page`/`per_page`（均可选，
//! 服务端对缺省值回退为「第 1 页、每页全部」，见 `examples/alist/internal/model/req.go`
//! 的 `PageReq.Validate`），响应 `data` 为 `{ "content": [...], "total": n }` 分页形态。
//! 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/list` 与
//! `examples/alist/server/handles/meta.go`（`ListMetas` 绑定 `model.PageReq`）。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::meta::Meta, common::PageResp};

/// 列出元信息请求构建器。
///
/// 通过 [`Meta::list`](super::Meta::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/meta/list", model = PageResp<Meta>)]
#[cfg_attr(feature = "into-stream", endpoint(into_stream = true, stream_item = Meta))]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（从 1 开始）；缺省时服务端默认第 1 页。
    #[query]
    page: Option<i32>,
    /// 每页条数；缺省时服务端默认返回全部条目。
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

impl<'a> super::Meta<'a> {
    /// 列出元信息（分页）。
    ///
    /// 对应 AList `GET /api/admin/meta/list`；响应 `data` 为
    /// `{ "content": [Meta], "total": n }`。不设置分页参数时，服务端
    /// （`PageReq.Validate`）会回退为「第 1 页、每页全部」，即一次性返回全部元信息。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/list` 与
    /// `examples/alist/server/handles/meta.go`（实现为 `ListMetas`，经 `op.GetMetas` 分页查询）。
    ///
    /// # Arguments
    ///
    /// 无必选参数；`page` 与 `per_page` 为可选分页参数，通过 [`Request`] 的链式 setter 设置。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 [`PageResp`]，
    /// 其 `content` 为 [`Meta`] 列表、`total` 为元信息总条数。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 分页获取
    /// let page = client.admin().meta().list().page(1).per_page(20).await?;
    /// for meta in &page.content {
    ///     println!("{} -> {}", meta.id, meta.path);
    /// }
    /// // 不设置分页参数则一次取回全部
    /// let all = client.admin().meta().list().await?;
    /// println!("共 {} 条", all.total);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build_request().build() 断言：方法、路径与查询串。
    #[test]
    fn build_request_composes_method_url_and_query() {
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
            url.contains("/api/admin/meta/list"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("page=2"), "URL 应包含 page 查询参数: {url}");
        assert!(
            url.contains("per_page=10"),
            "URL 应包含 per_page 查询参数: {url}"
        );
    }

    /// None 查询参数应被整体跳过（服务端回退默认：第 1 页、每页全部）。
    #[test]
    fn build_request_skips_absent_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/meta/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains("page="), "缺省 page 不应出现在查询串: {url}");
        assert!(
            !url.contains("per_page="),
            "缺省 per_page 不应出现在查询串: {url}"
        );
    }

    /// 收发路径断言：`docs/api/alistv3.openapi.yaml` `/api/admin/meta/list`
    /// 的 200 响应示例应能解码为 `PageResp<Meta>`。
    #[tokio::test]
    async fn send_decodes_openapi_list_example() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":1,"path":"/a","password":"i","p_sub":false,"write":false,"w_sub":false,"hide":"","h_sub":false,"readme":"","r_sub":false}],"total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client).send().await.unwrap();
        assert_eq!(resp.total, 1);
        assert_eq!(resp.content.len(), 1);
        assert_eq!(resp.content[0].id, 1);
        assert_eq!(resp.content[0].path, "/a");
        assert_eq!(resp.content[0].password, "i");
    }

    /// into-stream 翻页断言：逐页 `page=N` 请求，空页停止（含一次确认请求）。
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
                    r#"{"code":200,"message":"success","data":{"content":[{"id":1,"path":"/a"}],"total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":2,"path":"/b"}],"total":2}}"#,
                ),
                ok_json(r#"{"code":200,"message":"success","data":{"content":[],"total":2}}"#),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let items: Vec<Meta> = Request::new(&client)
            .into_stream()
            .map(|item| item.unwrap())
            .collect::<Vec<_>>()
            .await;
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].path, "/a");
        assert_eq!(items[1].path, "/b");

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 3, "空页会多一次确认请求: {:?}", recorded);
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
        assert!(recorded[2].contains("page=3"), "{}", recorded[2]);
    }
}
