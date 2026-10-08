//! fs 端点：表单（multipart/form-data）上传。
//!
//! 对应 `PUT /api/fs/form`；请求信息全部位于 HTTP 头与 multipart 请求体中，
//! 服务端实现为 `examples/alist/server/handles/fsup.go` 的 `FsForm`
//! （fsup.go:112-200，路由注册见 `server/router.go:240`）：
//!
//! | 请求头 | 语义 |
//! |---|---|
//! | `File-Path` | 目标绝对路径；服务端 `url.PathUnescape`（fsup.go:113-118），客户端需 URL 编码 |
//! | `Password` | 目录（元信息）密码；由 `server/middlewares/fsup.go:17` 读取 |
//! | `Overwrite` | 仅 `"false"` 时禁止覆盖，缺省允许覆盖（fsup.go:120） |
//! | `As-Task` | 仅 `"true"` 时转为后台上传任务（fsup.go:119） |
//! | `Last-Modified` | 文件修改时间，epoch 毫秒整数（fsup.go:19-28） |
//! | `X-File-Md5` / `X-File-Sha1` / `X-File-Sha256` | 可选哈希校验（fsup.go:155-164） |
//!
//! multipart 文件字段名必须为 `file`（fsup.go:143 `c.FormFile("file")`）；
//! 文件 mimetype 取自文件分部的 `Content-Type`（fsup.go:165-168），缺省由
//! 服务端按扩展名推断。`Content-Length` 由 reqwest 按表单总长度自动携带。
//! 直传成功时响应 `data` 为 `null`，转后台任务时为 `{"task": ...}`，
//! 因此端点模型为 `Option<UploadResponse>`。本端点**手写**请求构建器
//! （不走 [`EndpointRequest`](alist_client_derive::EndpointRequest) 派生宏，
//! 采用消费式请求体），支持直接 `.await` 或调用
//! [`send_upload`](Request::send_upload)。

use bytes::Bytes;
use reqwest::Method;

use crate::schema::common::UploadResponse;

/// 表单上传请求构建器。
///
/// 通过 [`Upload::form`](super::Upload::form) 创建。可选参数使用链式 setter，
/// 可直接 `.await` 或调用 [`send_upload`](Request::send_upload)。
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send_upload().await`"]
pub struct Request<'a> {
    client: &'a crate::Client,
    /// 目标绝对路径（`File-Path` 头，发送时 URL 编码）。
    path: String,
    /// 文件字节内容（multipart `file` 字段）。
    content: Bytes,
    /// multipart 文件分部文件名；缺省取目标路径的末段。
    file_name: Option<String>,
    /// 目录（元信息）密码（`Password` 头）。
    password: Option<String>,
    /// 是否允许覆盖已有文件（`Overwrite` 头；服务端缺省允许）。
    overwrite: Option<bool>,
    /// 是否转为后台上传任务（`As-Task` 头）。
    as_task: Option<bool>,
    /// 文件修改时间，epoch 毫秒（`Last-Modified` 头）。
    last_modified: Option<i64>,
    /// 文件 mimetype（multipart 文件分部的 `Content-Type`）。
    content_type: Option<String>,
    /// MD5 哈希（`X-File-Md5` 头）。
    md5: Option<String>,
    /// SHA-1 哈希（`X-File-Sha1` 头）。
    sha1: Option<String>,
    /// SHA-256 哈希（`X-File-Sha256` 头）。
    sha256: Option<String>,
}

/// 取目标路径末段作为 multipart 文件分部的缺省文件名。
fn default_file_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map_or_else(|| "file".to_owned(), ToString::to_string)
}

/// 构造器。
///
/// 用于创建 [`Request`] 实例，接收必选参数。
impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走链式 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        path: impl Into<String>,
        content: impl Into<Bytes>,
    ) -> Self {
        Self {
            client,
            path: path.into(),
            content: content.into(),
            file_name: None,
            password: None,
            overwrite: None,
            as_task: None,
            last_modified: None,
            content_type: None,
            md5: None,
            sha1: None,
            sha256: None,
        }
    }
}

/// 可选参数配置。
///
/// 包含所有可选 Header 与上传行为控制的链式 setter 方法。
impl<'a> Request<'a> {
    /// multipart 文件分部文件名（可选；缺省取目标路径末段）。
    #[inline]
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 目录（元信息）密码（可选；`Password` 头）。
    #[inline]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// 是否允许覆盖已有文件（可选；`Overwrite` 头）。
    ///
    /// 缺省时服务端允许覆盖；仅显式传 `false` 时，目标文件已存在的上传
    /// 会被服务端以 `403 file exists` 拒绝（fsup.go:127-133）。
    #[inline]
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    /// 是否转为后台上传任务（可选；`As-Task` 头）。
    ///
    /// 缺省时直传（响应 `data` 为 `null`）；传 `true` 时服务端创建后台任务
    /// 并返回任务信息（fsup.go:180-188）。
    #[inline]
    pub fn as_task(mut self, as_task: bool) -> Self {
        self.as_task = Some(as_task);
        self
    }

    /// 文件修改时间（可选；`Last-Modified` 头，epoch 毫秒整数）。
    ///
    /// 缺省时服务端以收到请求的时间作为修改时间（fsup.go:19-28）。
    #[inline]
    pub fn last_modified(mut self, last_modified: i64) -> Self {
        self.last_modified = Some(last_modified);
        self
    }

    /// 文件 mimetype（可选；随 multipart `file` 分部的 `Content-Type` 发送）。
    ///
    /// 缺省时服务端按目标文件扩展名推断 mimetype（fsup.go:165-168）。
    #[inline]
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// 文件 MD5 哈希（可选；`X-File-Md5` 头，十六进制小写文本）。
    #[inline]
    pub fn md5(mut self, md5: impl Into<String>) -> Self {
        self.md5 = Some(md5.into());
        self
    }

    /// 文件 SHA-1 哈希（可选；`X-File-Sha1` 头，十六进制小写文本）。
    #[inline]
    pub fn sha1(mut self, sha1: impl Into<String>) -> Self {
        self.sha1 = Some(sha1.into());
        self
    }

    /// 文件 SHA-256 哈希（可选；`X-File-Sha256` 头，十六进制小写文本）。
    #[inline]
    pub fn sha256(mut self, sha256: impl Into<String>) -> Self {
        self.sha256 = Some(sha256.into());
        self
    }
}

/// 请求组装与发送。
///
/// 将配置好的参数组装为底层 HTTP 请求并发送（亦可通过 [`core::future::IntoFuture`] 直接 `.await`）。
impl<'a> Request<'a> {
    /// 组装带上传头与 multipart 请求体的请求构建器（发送前；测试亦用于形状断言）。
    fn into_builder(self) -> crate::Result<reqwest::RequestBuilder> {
        let mut part = reqwest::multipart::Part::stream(self.content).file_name(
            self.file_name
                .unwrap_or_else(|| default_file_name(&self.path)),
        );
        if let Some(content_type) = &self.content_type {
            part = part.mime_str(content_type)?;
        }
        let mut builder = self
            .client
            .request(Method::PUT, "/api/fs/form")
            .header(
                "File-Path",
                // 服务端 url.PathUnescape 解码；Cow 不满足 HeaderValue 约束，取所有权字符串
                urlencoding::encode(&self.path).into_owned(),
            )
            .multipart(reqwest::multipart::Form::new().part("file", part));
        if let Some(password) = &self.password {
            builder = builder.header("Password", password);
        }
        if let Some(overwrite) = self.overwrite {
            builder = builder.header("Overwrite", if overwrite { "true" } else { "false" });
        }
        if let Some(as_task) = self.as_task {
            builder = builder.header("As-Task", if as_task { "true" } else { "false" });
        }
        if let Some(last_modified) = self.last_modified {
            builder = builder.header("Last-Modified", last_modified.to_string());
        }
        if let Some(md5) = &self.md5 {
            builder = builder.header("X-File-Md5", md5);
        }
        if let Some(sha1) = &self.sha1 {
            builder = builder.header("X-File-Sha1", sha1);
        }
        if let Some(sha256) = &self.sha256 {
            builder = builder.header("X-File-Sha256", sha256);
        }
        Ok(builder)
    }

    /// 附加上传头与 multipart 请求体并发送。
    ///
    /// multipart 文件字段名为 `file`；`Content-Length` 由 reqwest 按表单总长度
    /// 自动携带。直传成功（未启用 `as_task(true)`）时响应 `data` 为 `null`，
    /// 返回 `None`；转后台任务时返回
    /// [`UploadResponse`]。
    ///
    /// # Errors
    ///
    /// 语义同 `Client::execute`：当网络请求失败或
    /// AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。注意：multipart 请求体无法克隆，401/403 自动
    /// 重登重试不可用，将直接返回原始错误（见 `docs/design.md` §8）。
    pub async fn send_upload(self) -> crate::Result<Option<UploadResponse>> {
        self.client.execute(self.into_builder()?).await
    }

    /// 发送上传请求（[`send_upload`](Self::send_upload) 的别名，与其他端点的 `send` 保持一致）。
    #[inline]
    pub async fn send(self) -> crate::Result<Option<UploadResponse>> {
        self.send_upload().await
    }
}

impl<'a> core::future::IntoFuture for Request<'a> {
    type Output = crate::Result<Option<UploadResponse>>;
    type IntoFuture =
        core::pin::Pin<Box<dyn core::future::Future<Output = Self::Output> + Send + 'a>>;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send_upload())
    }
}

impl<'a> super::Upload<'a> {
    /// 以 multipart/form-data 表单上传文件。
    ///
    /// 对应 AList `PUT /api/fs/form`（服务端实现 `FsForm`，
    /// `examples/alist/server/handles/fsup.go:112-200`）。文件以名为 `file`
    /// 的 multipart 字段发送；目标文件路径通过 `File-Path` 头 URL 编码后
    /// 发送；`Content-Length` 由 reqwest 按表单总长度自动携带。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标绝对路径（含文件名，如 `/data/demo.zip`；末段同时作为
    ///   multipart 文件分部的缺省文件名，可用
    ///   [`file_name`](Request::file_name) 覆盖）。
    /// * `content` - 文件字节内容（`impl Into<Bytes>`，如 `Vec<u8>`、
    ///   `Bytes`；非 `'static` 的切片引用可先 `.to_vec()`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可选参数链式设置后可直接 `.await`，
    /// 或调用 [`send_upload`](Request::send_upload) 发送。
    ///
    /// # Errors
    ///
    /// 构建请求本身不失败；发送时的错误语义见
    /// [`send_upload`](Request::send_upload)。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let resp = client
    ///     .fs()
    ///     .upload()
    ///     .form("/data/demo.zip", vec![0u8; 16])
    ///     .as_task(true) // 可选：转为后台上传任务
    ///     .await?;
    /// if let Some(resp) = resp {
    ///     println!("上传任务: {}", resp.task.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn form(&self, path: impl Into<String>, content: impl Into<Bytes>) -> Request<'a> {
        Request::new(self.client, path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 请求形状断言：`PUT /api/fs/form`，multipart 请求体，
    ///    `File-Path` URL 编码；无显式 Content-Length（reqwest 自动携带）。
    #[test]
    fn into_builder_composes_multipart_request() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/dir 1/a.zip", b"hello".to_vec())
            .into_builder()
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::PUT);
        assert!(
            built.url().as_str().ends_with("/api/fs/form"),
            "URL 应为 /api/fs/form: {}",
            built.url()
        );
        assert_eq!(
            built
                .headers()
                .get("File-Path")
                .and_then(|v| v.to_str().ok()),
            Some("%2Fdata%2Fdir%201%2Fa.zip"),
            "File-Path 应为 URL 编码后的目标路径"
        );
        let content_type = built
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .expect("multipart 请求必须携带 Content-Type");
        assert!(
            content_type.starts_with("multipart/form-data; boundary="),
            "Content-Type 应为 multipart/form-data: {content_type}"
        );
        assert!(built.body().is_some(), "multipart 请求必须携带请求体");
    }

    /// 2) 请求形状断言：可选头在设置后按 API 语义发送，未设置时不发送。
    #[test]
    fn into_builder_sends_optional_headers_semantically() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/a.zip", b"h".to_vec())
            .file_name("local.zip".to_owned())
            .password("meta-secret".to_owned())
            .overwrite(false)
            .as_task(true)
            .last_modified(1_700_000_000_000)
            .content_type("application/zip".to_owned())
            .md5("d41d8cd98f00b204e9800998ecf8427e".to_owned())
            .into_builder()
            .unwrap()
            .build()
            .unwrap();
        let headers = built.headers();
        assert_eq!(
            headers.get("Password").and_then(|v| v.to_str().ok()),
            Some("meta-secret")
        );
        assert_eq!(
            headers.get("Overwrite").and_then(|v| v.to_str().ok()),
            Some("false")
        );
        assert_eq!(
            headers.get("As-Task").and_then(|v| v.to_str().ok()),
            Some("true")
        );
        assert_eq!(
            headers.get("Last-Modified").and_then(|v| v.to_str().ok()),
            Some("1700000000000")
        );
        assert_eq!(
            headers.get("X-File-Md5").and_then(|v| v.to_str().ok()),
            Some("d41d8cd98f00b204e9800998ecf8427e")
        );

        let unset = Request::new(&client, "/data/a.zip", b"h".to_vec())
            .into_builder()
            .unwrap()
            .build()
            .unwrap();
        let headers = unset.headers();
        assert!(headers.get("Password").is_none(), "未设置的密码头不应发送");
        assert!(
            headers.get("Overwrite").is_none(),
            "缺省覆盖语义由服务端提供"
        );
        assert!(headers.get("As-Task").is_none(), "缺省直传语义由服务端提供");
        assert!(
            headers.get("Last-Modified").is_none(),
            "未设置的修改时间不应发送"
        );
        assert!(
            headers.get("X-File-Md5").is_none(),
            "未设置的哈希头不应发送"
        );
    }

    /// 3) 收发路径断言：mock 服务器 + 记录请求原文；multipart 字段名为 `file`，
    ///    缺省文件名取自目标路径末段；`data: null` 解码为 `None`。
    #[tokio::test]
    async fn send_upload_posts_multipart_and_decodes_null_task() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "/data/a.zip", b"hello".to_vec())
            .send_upload()
            .await
            .unwrap();
        assert_eq!(resp, None, "直传成功时 data 为 null，应解码为 None");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("PUT /api/fs/form "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0]
                .to_ascii_lowercase()
                .contains("file-path: %2fdata%2fa.zip"),
            "File-Path 应为 URL 编码后的目标路径: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#"name="file""#),
            "multipart 文件字段名必须为 file: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#"filename="a.zip""#),
            "缺省文件名应取自目标路径末段: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("hello"),
            "multipart 请求体应携带文件内容: {}",
            recorded[0]
        );
    }

    /// 4) 收发路径断言：`As-Task` 时响应 `data` 携带任务信息，解码为 `Some`。
    #[tokio::test]
    async fn send_upload_decodes_background_task() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"task":{
                    "id":"sdH2LbjyWRk","name":"upload a.zip to [/data](/alist)",
                    "state":0,"status":"uploading","progress":0,"error":""}}}"#,
            )],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "/data/a.zip", b"hello".to_vec())
            .as_task(true)
            .send_upload()
            .await
            .unwrap();
        let task = resp.expect("As-Task 上传应返回任务信息").task;
        assert_eq!(task.id, "sdH2LbjyWRk");
        assert_eq!(task.status, "uploading");
    }

    /// 3) IntoFuture 支持直接 `.await` 发送请求。
    #[tokio::test]
    async fn request_is_awaitable_via_into_future() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client
            .fs()
            .upload()
            .form("/data/await.zip", vec![0u8; 8])
            .await
            .unwrap();
        assert!(resp.is_none());

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("PUT /api/fs/form "), "{}", recorded[0]);
        assert!(
            recorded[0].contains("file-path: %2Fdata%2Fawait.zip"),
            "{}",
            recorded[0]
        );
    }
}
