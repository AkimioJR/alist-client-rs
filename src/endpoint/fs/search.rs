//! fs 端点：搜索文件或文件夹。
//!
//! 对应 `POST /api/fs/search`；依赖服务端索引（索引未启用时返回
//! `SearchNotAvailable` 错误）。请求体为 `parent`/`keywords`/`scope`/`page`/
//! `per_page`/`password`（Go `SearchReq` = `model.SearchReq` + `password`，
//! `server/handles/search.go:17-20` 与 `internal/model/search.go:15-21`），
//! 其中 `page`/`per_page` 由服务端 `Validate` 强制要求 ≥ 1（search.go:30-38）。
//! 响应为 `{ "content": [...], "total": n }` 分页形态。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::{common::PageResponse, fs::SearchResponse};

/// 搜索文件或文件夹请求构建器。
///
/// 通过 [`Fs::search`](super::Fs::search) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/search", model = PageResponse<SearchResponse>)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = SearchResponse)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 搜索目录（必选；结果限定在该目录之下）。
    parent: String,
    /// 搜索关键词（必选）。
    keywords: String,
    /// 每页条数（必选；服务端 `model.SearchReq.Validate` 要求 ≥ 1，
    /// 缺省时服务端直接返回 400，因此作为必选参数传入）。
    per_page: i32,
    /// 页码（可选，从 1 开始）。
    ///
    /// 服务端 `Validate` 要求 ≥ 1，缺省时会被拒绝，故默认以第 1 页发送；
    /// 启用 `into-stream` feature 时，[`Request::into_stream`] 会自动逐页递增。
    page: Option<i32>,
    /// 搜索范围（可选）：`0` 全部（服务端缺省值）、`1` 仅文件夹、`2` 仅文件。
    scope: Option<i32>,
    /// 搜索目录的密码（可选；目录受密码保护时必填）。
    password: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数：服务端强制要求 `per_page ≥ 1`
    /// （`model.SearchReq.Validate`，缺失直接 400），故为必选；页码默认第 1 页。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        parent: impl Into<String>,
        keywords: impl Into<String>,
        per_page: i32,
    ) -> Self {
        Self {
            client,
            parent: parent.into(),
            keywords: keywords.into(),
            per_page,
            page: Some(1),
            scope: None,
            password: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 搜索文件或文件夹。
    ///
    /// 对应 AList `POST /api/fs/search`；在服务端搜索索引中检索 `parent`
    /// 目录之下匹配 `keywords` 的文件/目录，返回分页结果
    /// （[`PageResponse<SearchResponse>`](crate::schema::common::PageResponse)）。
    /// 服务端需已启用搜索索引，否则返回 `SearchNotAvailable` 错误。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/search` 与
    /// `examples/alist/server/handles/search.go`（实现为 `handles.Search`，
    /// 请求 `SearchReq` search.go:17-20，元素 `SearchResponse` search.go:22-25）。
    ///
    /// # Arguments
    ///
    /// * `parent` - 搜索目录；结果限定在该目录之下。
    /// * `keywords` - 搜索关键词。
    /// * `per_page` - 每页条数；服务端要求 ≥ 1，缺失时直接返回 400，
    ///   因此作为必选参数传入。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse<SearchResponse>`](crate::schema::common::PageResponse)。
    ///
    /// 可选参数（链式 setter）：`page`（页码，默认第 1 页）、`scope`
    /// （`0` 全部 / `1` 仅文件夹 / `2` 仅文件）、`password`（目录密码）。
    /// 启用 `into-stream` feature 时可调用 [`Request::into_stream`] 获得
    /// 自动翻页的结果流：从当前页逐页请求，结果耗尽后 `content` 为空时终止
    /// （对应分页 `total` 计数用尽）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；未启用搜索索引、`page`/`per_page` 小于 1
    /// 时服务端返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 在 /local 下搜索关键词 test，每页 20 条
    /// let page = client.fs().search("/local", "test", 20).await?;
    /// for hit in &page.content {
    ///     println!("{}{}", hit.parent, hit.name);
    /// }
    /// // 仅搜索文件、跳到第 2 页
    /// let files = client.fs().search("/local", "test", 20).scope(2).page(2).await?;
    /// # let _ = files;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn search(
        &self,
        parent: impl Into<String>,
        keywords: impl Into<String>,
        per_page: i32,
    ) -> Request<'a> {
        Request::new(self.client, parent, keywords, per_page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/search，请求体含全部必选与默认字段。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/local", "test", 20)
            .scope(2)
            .password("secret".to_owned())
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/search"), "URL 应包含路径: {url}");
        assert!(built.url().query().is_none(), "搜索参数应位于请求体");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["parent"], "/local");
        assert_eq!(json["keywords"], "test");
        assert_eq!(json["per_page"], 20);
        assert_eq!(json["page"], 1, "页码缺省为第 1 页（服务端要求 ≥ 1）");
        assert_eq!(json["scope"], 2);
        assert_eq!(json["password"], "secret");
    }

    /// 2) 请求体序列化断言：未设置的可选字段应被跳过；页码可显式覆盖。
    #[test]
    fn build_request_skips_unset_options() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/", "kw", 10)
            .page(3)
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["page"], 3);
        assert!(json.get("scope").is_none(), "未设置的可选字段应被跳过");
        assert!(json.get("password").is_none(), "未设置的可选字段应被跳过");
    }

    /// 3) 收发路径断言：mock 服务器返回 openapi `/api/fs/search` 示例，
    /// 断言 POST 方法与分页解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_page() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"content":[{"parent":"/m","name":"4305da1e","is_dir":false,"size":393090,"type":0}],"total":1}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let page = client
            .fs()
            .search("/local", "test", 10)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].parent, "/m");
        assert_eq!(page.content[0].size, 393090);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/search "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""keywords":"test""#),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""per_page":10"#), "{}", recorded[0]);
        assert!(recorded[0].contains(r#""page":1"#), "{}", recorded[0]);
    }

    /// 4) into-stream 翻页断言：请求体 `page` 从 1 逐页递增，
    /// 结果耗尽返回空 `content` 时停止（最后一页多一次确认请求）。
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
                    r#"{"code":200,"message":"success","data":{"content":[{"parent":"/m","name":"a.txt","is_dir":false,"size":1,"type":0}],"total":1}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[],"total":1}}"#,
                ),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let names: Vec<String> = Request::new(&client, "/local", "test", 1)
            .into_stream()
            .map(|item| item.unwrap().name)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(names, vec!["a.txt".to_owned()]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 2, "空 content 页会多一次确认请求");
        assert!(recorded[0].contains(r#""page":1"#), "{}", recorded[0]);
        assert!(recorded[1].contains(r#""page":2"#), "{}", recorded[1]);
    }
}
