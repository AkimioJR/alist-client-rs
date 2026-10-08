//! fs 端点：列出压缩包内目录内容。
//!
//! 对应 `POST /api/fs/archive/list`；响应 `data` 为
//! `{content: [...], total: N}` 分页形状（共享模型
//! [`PageResponse`]，元素为
//! [`ObjResp`]）。该端点未收录于
//! `docs/api/alistv3.openapi.yaml`，请求/响应形状以
//! `examples/alist/server/handles/archive.go` 的 `ArchiveListReq`/`ArchiveListResp`
//! （archive.go:147-156）为准；分页字段与 `internal/model/req.go` 的
//! `PageRequest`（`page`/`per_page`）一致，随 JSON 请求体发送。

use alist_client_derive::EndpointRequest;

use crate::schema::{common::PageResponse, fs::ObjResp};

/// 列出压缩包内目录内容请求构建器。
///
/// 通过 [`Archive::list`](super::Archive::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
///
/// 与 `meta` 的递归文件树不同，本端点仅列出 `inner_path` 指定目录
/// （缺省为压缩包根）下的直接子项，并支持分页。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/fs/archive/list",
    model = PageResponse<ObjResp>
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 压缩包的完整路径（必选）。
    path: String,
    /// 目录（元信息）密码（可选）；对应 Go `ArchiveMetaReq.Password`。
    password: Option<String>,
    /// 是否强制刷新服务端缓存的归档元信息（可选）；对应 Go `ArchiveMetaReq.Refresh`。
    refresh: Option<bool>,
    /// 加密压缩包的解压密码（可选）；对应 Go `ArchiveMetaReq.ArchivePass`。
    archive_pass: Option<String>,
    /// 页码，从 1 开始（可选，随 JSON 请求体发送）；缺省时服务端按第 1 页处理。
    page: Option<i32>,
    /// 每页条数（可选，随 JSON 请求体发送）；服务端对小于 1 的值返回全部条目。
    per_page: Option<i32>,
    /// 压缩包内部路径（可选）；缺省列出压缩包根目录。
    inner_path: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            password: None,
            refresh: None,
            archive_pass: None,
            page: None,
            per_page: None,
            inner_path: None,
        }
    }
}

impl<'a> super::Archive<'a> {
    /// 列出压缩包内目录内容。
    ///
    /// 对应 AList `POST /api/fs/archive/list`；返回 `inner_path`
    /// （缺省为压缩包根）下直接子项的分页列表（`content` 与 `total`）。
    /// 压缩包密码错误时 AList 以响应 `code: 202` 返回。
    ///
    /// 数据来源：`examples/alist/server/router.go:246-248`
    /// （`a.Any("/list", handles.FsArchiveList)`，携带 JSON 请求体的读端点
    /// 按 POST 处理）与 `examples/alist/server/handles/archive.go:158-222`
    /// （实现 `FsArchiveList`；分页经 `pagination(objs, &req.PageRequest)`）。
    ///
    /// # Arguments
    ///
    /// * `path` - 压缩包的完整路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse<ObjResp>`](crate::schema::common::PageResponse)。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如目录密码错误、压缩包密码错误）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let page = client.fs().archive().list("/data/demo.zip")
    ///     .inner_path("docs".to_owned()) // 可选：压缩包内部子目录
    ///     .page(1)
    ///     .per_page(50)
    ///     .await?;
    /// for entry in &page.content {
    ///     println!("{} ({} 字节)", entry.name, entry.size);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) URL/方法断言：`POST /api/fs/archive/list`，分页字段随请求体而非查询串发送。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/demo.zip")
            .page(2)
            .per_page(50)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().ends_with("/api/fs/archive/list"),
            "URL 应为 /api/fs/archive/list: {}",
            built.url()
        );
        assert!(
            !built.url().as_str().contains("page="),
            "分页字段应随 JSON 请求体发送，而不是查询串: {}",
            built.url()
        );
    }

    /// 2) 请求体序列化断言：分页与压缩包内部路径按 API 键名写入请求体。
    #[test]
    fn build_request_serializes_pagination_in_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/demo.zip")
            .page(2)
            .per_page(50)
            .inner_path("docs".to_owned())
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/data/demo.zip");
        assert_eq!(json["page"], 2);
        assert_eq!(json["per_page"], 50);
        assert_eq!(json["inner_path"], "docs");
    }

    /// 3) 收发路径断言：mock 服务器 + 记录请求原文 + 解码分页列表。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_page() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{
            "content":[{"name":"a.txt","size":3,"is_dir":false,
                "modified":"2024-01-01T00:00:00Z","created":"2024-01-01T00:00:00Z",
                "sign":"","thumb":"","type":0,"hashinfo":""}],
            "total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client, "/data/demo.zip")
            .page(1)
            .per_page(10)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].name, "a.txt");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/archive/list "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"path\":\"/data/demo.zip\""),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"page\":1"), "{}", recorded[0]);
        assert!(recorded[0].contains("\"per_page\":10"), "{}", recorded[0]);
    }
}
