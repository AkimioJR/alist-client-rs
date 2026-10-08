//! admin-label-file-binding 端点：分页列出标签绑定记录。
//!
//! 对应 `GET /api/admin/label_file_binding/list`；响应 `data` 为
//! `{"content":[LabelFileBinding],"total":N}` 分页结构（handler 内部
//! `pageResp[model.LabelFileBinding]`，与 [`PageResponse`]
//! 形状一致），以 `PageResponse<LabelFileBinding>` 作为端点模型。
//! 该分组未收录进 openapi 文档；路由见 `examples/alist/server/router.go:210`，
//! 处理逻辑见 `examples/alist/server/handles/label_file_binding.go:125`
//! （实现为 `handles.ListLabelFileBinding`）。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::label_file_binding::LabelFileBinding, common::PageResponse};

/// 分页列出标签绑定记录请求构建器。
///
/// 通过 [`LabelFileBinding::list`](super::LabelFileBinding::list) 创建。
/// 可选参数使用链式 setter，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/label_file_binding/list",
    model = PageResponse<LabelFileBinding>
)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = LabelFileBinding)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（可选，从 1 开始；缺省或非法时服务端按第 1 页处理）。
    #[query]
    page: Option<i32>,
    /// 每页条数（可选；缺省或非法时服务端取 50，最大 200）。
    #[query]
    page_size: Option<i32>,
    /// 按文件名过滤（可选；精确匹配绑定记录的 `file_name`）。
    #[query]
    file_name: Option<String>,
    /// 按标签 ID 过滤（可选；支持逗号分隔多个标签，如 `"1,2"`）。
    ///
    /// 服务端逐段解析为无符号整数，任一段非法时返回 400。
    #[query]
    label_id: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点没有必选参数，可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            page_size: None,
            file_name: None,
            label_id: None,
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 分页列出标签绑定记录。
    ///
    /// 对应 AList `GET /api/admin/label_file_binding/list`；成功时响应 `data` 为
    /// `{"content":[LabelFileBinding],"total":N}`。
    /// 数据来源：`examples/alist/server/router.go:210`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:125`
    /// （实现为 `handles.ListLabelFileBinding`）。
    ///
    /// # Arguments
    ///
    /// 无必选参数；可选参数通过返回的 [`Request`] 链式 setter 设置
    /// （`page`、`page_size`、`file_name`、`label_id`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse<LabelFileBinding>`](crate::schema::common::PageResponse)。
    /// 启用 `into-stream` feature 时也可调用 `.into_stream()` 逐页流式产出绑定记录。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`label_id` 含非数字片段时服务端返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 第 2 页、每页 100 条，按标签 ID 过滤：
    /// let page = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .list()
    ///     .page(2)
    ///     .page_size(100) // 可选：每页条数，缺省 50、最大 200
    ///     .label_id("1,2") // 可选：按标签过滤，逗号分隔
    ///     .await?;
    /// println!("共 {} 条绑定记录", page.total);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 绑定记录示例 JSON（按 `examples/alist/internal/model/label_file_binding.go:5`
    /// 的 `model.LabelFileBinding` JSON tag 构造）。
    const BINDING_JSON: &str = r#"{"id":7,"user_id":1,"label_id":3,"file_name":"movie.mp4","create_time":"2024-06-01T08:00:00Z"}"#;

    /// 请求形状断言：GET 方法、URL 路径与全部查询参数（含 None 跳过）。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .page_size(100)
            .file_name("movie.mp4")
            .label_id("1,2")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label_file_binding/list"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("page=2"), "URL 应包含页码查询参数: {url}");
        assert!(
            url.contains("page_size=100"),
            "URL 应包含每页条数查询参数: {url}"
        );
        assert!(
            url.contains("file_name=movie.mp4"),
            "URL 应包含文件名过滤参数: {url}"
        );
        assert!(
            url.contains("label_id=1%2C2"),
            "URL 应包含标签过滤参数（逗号按百分号编码）: {url}"
        );

        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("page="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("page_size="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("file_name="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("label_id="), "None 查询参数应被跳过: {url}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码分页响应。
    #[tokio::test]
    async fn send_lists_bindings_from_page_response() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{BINDING_JSON}],"total":1}}}}"#
        );
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let page = Request::new(&client).page_size(50).send().await.unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].id, 7);
        assert_eq!(page.content[0].user_id, 1);
        assert_eq!(page.content[0].label_id, 3);
        assert_eq!(page.content[0].file_name, "movie.mp4");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/admin/label_file_binding/list?"),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("page_size=50"), "{}", recorded[0]);
    }

    /// into-stream 翻页断言（列表端点）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use futures::StreamExt;

        let requests = Arc::new(Mutex::new(Vec::new()));
        let page_body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{BINDING_JSON}],"total":1}}}}"#
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
        assert_eq!(ids, vec![7]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 2, "空页会多一次确认请求");
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
    }
}
