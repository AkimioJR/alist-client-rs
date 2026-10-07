//! fs 端点：读取压缩包元信息。
//!
//! 对应 `POST /api/fs/archive/meta`；响应 `data` 为压缩包元信息（注释、
//! 是否加密、递归文件树 [`ArchiveContentResp`](crate::schema::fs::ArchiveContentResp)、
//! 原始下载地址与签名）。该端点未收录于 `docs/api/alistv3.openapi.yaml`，
//! 请求/响应形状以 `examples/alist/server/handles/archive.go` 的
//! `ArchiveMetaReq`/`ArchiveMetaResp`（archive.go:25-44）为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::ArchiveMetaResp;

/// 读取压缩包元信息请求构建器。
///
/// 通过 [`Archive::meta`](super::Archive::meta) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/fs/archive/meta",
    model = ArchiveMetaResp
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
        }
    }
}

impl<'a> super::Archive<'a> {
    /// 读取压缩包元信息。
    ///
    /// 对应 AList `POST /api/fs/archive/meta`；返回压缩包注释、是否加密、
    /// 递归文件树（`content[].children` 嵌套）、原始下载地址（`raw_url`）
    /// 与访问签名（`sign`）。压缩包密码错误时 AList 以信封 `code: 202`
    /// 返回，[`Client::execute`](crate::Client::execute) 会将其转换为
    /// [`Error::Api`](crate::Error::Api)。
    ///
    /// 数据来源：`examples/alist/server/router.go:246-248`
    /// （`a.Any("/meta", handles.FsArchiveMeta)`，携带 JSON 请求体的读端点
    /// 按 POST 处理）与 `examples/alist/server/handles/archive.go:76-145`
    /// （实现 `FsArchiveMeta`）。
    ///
    /// # Arguments
    ///
    /// * `path` - 压缩包的完整路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`ArchiveMetaResp`](crate::schema::fs::ArchiveMetaResp)。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
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
    /// let meta = client.fs().archive().meta("/data/demo.zip")
    ///     .archive_pass("archive-secret".to_owned()) // 可选：压缩包密码
    ///     .await?;
    /// println!("是否加密: {}", meta.encrypted);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn meta(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) URL/方法断言：`POST /api/fs/archive/meta`，无查询串。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/demo.zip")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().ends_with("/api/fs/archive/meta"),
            "URL 应为 /api/fs/archive/meta: {}",
            built.url()
        );
        assert_eq!(built.url().query(), None, "本端点无查询参数");
    }

    /// 2) 请求体序列化断言：必选字段 `path` 必须出现；未设置的可选字段应被跳过。
    #[test]
    fn build_request_serializes_required_path_and_skips_unset_options() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/demo.zip")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/data/demo.zip");
        assert!(json.get("password").is_none(), "未设置的可选字段应被跳过");
        assert!(json.get("refresh").is_none(), "未设置的可选字段应被跳过");
        assert!(
            json.get("archive_pass").is_none(),
            "未设置的可选字段应被跳过"
        );
    }

    /// 3) 请求体序列化断言：可选字段链式 setter 后按 API 键名写入请求体。
    #[test]
    fn build_request_serializes_optional_fields_with_api_keys() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/demo.zip")
            .password("meta-secret".to_owned())
            .refresh(true)
            .archive_pass("zip-pass".to_owned())
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/data/demo.zip");
        assert_eq!(json["password"], "meta-secret");
        assert_eq!(json["refresh"], true);
        assert_eq!(json["archive_pass"], "zip-pass");
    }

    /// 4) 收发路径断言：mock 服务器 + 记录请求原文 + 解码递归文件树。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_tree() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{
            "comment":"archive comment",
            "encrypted":false,
            "content":[{"name":"a.txt","size":3,"is_dir":false,
                "modified":"2024-01-01T00:00:00Z","created":"2024-01-01T00:00:00Z",
                "sign":"","thumb":"","type":0,"hashinfo":"","children":[]}],
            "raw_url":"https://alist.example/ae/demo.zip",
            "sign":"sign-token"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let meta = Request::new(&client, "/data/demo.zip")
            .archive_pass("zip-pass".to_owned())
            .send()
            .await
            .unwrap();
        assert_eq!(meta.comment, "archive comment");
        assert!(!meta.encrypted);
        assert_eq!(meta.content.len(), 1);
        assert!(meta.content[0].children.is_empty());

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/archive/meta "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"path\":\"/data/demo.zip\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"archive_pass\":\"zip-pass\""),
            "{}",
            recorded[0]
        );
    }
}
