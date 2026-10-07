//! admin-label 端点：列出标签。
//!
//! 对应 `GET /api/label/list`（openapi 未收录该分组，路由以
//! `examples/alist/server/router.go` 的 `_label` 为准；实现为 `handles.ListLabel`）。
//! 注意：列表/详情两个读取端点挂在 `/api/label` 而非 `/api/admin/label` 下。
//! 响应 `data` 为 `{ "content": [Label], "total": N }` 分页结构，以
//! `PageResp<Label>` 解码。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::label::Label, common::PageResp};

/// 列出标签请求构建器。
///
/// 通过 [`Label::list`](super::Label::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/label/list", model = PageResp<Label>)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = Label)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（可选，从 1 开始；缺省时服务端按第 1 页处理）。
    #[query]
    page: Option<i32>,
    /// 每页条数（可选；缺省时服务端返回全部标签）。
    #[query]
    per_page: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点没有必选参数，可选分页参数走派生 setter。
    ///
    /// 返回类型 [`Request`] 已整体标记 `#[must_use]`，此处不再重复标注。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Label<'a> {
    /// 列出标签（分页）。
    ///
    /// 对应 AList `GET /api/label/list`；服务端按 `page`/`per_page` 查询参数分页，
    /// `per_page` 缺省时返回全部标签（`handles.ListLabel` 中 `PageReq.Validate` 的语义）。
    /// 成功时响应 `data` 为 `{ "content": [Label], "total": N }`，解码为
    /// [`PageResp<Label>`](crate::schema::common::PageResp)。
    /// 数据来源：`examples/alist/server/router.go` 的 `_label` 路由与
    /// `examples/alist/server/handles/label.go`（实现为 `ListLabel`）；
    /// openapi 文档未收录该分组。
    ///
    /// # Arguments
    ///
    /// 无必选参数；可选分页参数通过返回的 [`Request`] 链式 setter 设置
    /// （`page`、`per_page`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResp<Label>`](crate::schema::common::PageResp)。
    /// 启用 `into-stream` feature 时也可调用 `.into_stream()` 逐页流式产出标签。
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
    /// // 第 2 页、每页 50 条：
    /// client.admin().label().list().page(2).per_page(50).await?;
    /// // 不传分页参数时服务端返回全部标签：
    /// client.admin().label().list().await?;
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

    /// 标签条目示例 JSON（按 `examples/alist/internal/model/label.go` 的 JSON tag 构造）。
    const LABEL_JSON: &str = r##"{"id":1,"type":2,"name":"电影","description":"电影相关文件","bg_color":"#FF0000","create_time":"2024-06-01T12:00:00Z"}"##;

    /// 1) 纯 URL/方法断言：build_request().build() 检查 method 与 URL（含查询串）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .per_page(50)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/label/list"), "URL 应包含路径: {url}");
        assert!(url.contains("page=2"), "URL 应包含页码查询参数: {url}");
        assert!(
            url.contains("per_page=50"),
            "URL 应包含每页条数查询参数: {url}"
        );

        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("page="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("per_page="), "None 查询参数应被跳过: {url}");
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文。
    #[tokio::test]
    async fn send_lists_labels_from_page_response() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{LABEL_JSON}],"total":1}}}}"#
        );
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client).per_page(50).send().await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].id, 1);
        assert_eq!(page.content[0].label_type, 2);
        assert_eq!(page.content[0].name, "电影");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/label/list?"),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("per_page=50"), "{}", recorded[0]);
    }

    /// 3) into-stream 翻页断言（列表端点）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use std::sync::{Arc, Mutex};

        use futures::StreamExt;

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let page_body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{LABEL_JSON}],"total":1}}}}"#
        );
        let base_url = spawn_mock_server(
            vec![
                ok_json(page_body),
                ok_json(r#"{"code":200,"message":"success","data":{"content":[],"total":1}}"#),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let ids: Vec<u64> = Request::new(&client)
            .into_stream()
            .map(|item| item.unwrap().id)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(ids, vec![1]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 2, "空页会多一次确认请求");
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
    }
}
