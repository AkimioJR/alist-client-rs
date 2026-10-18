//! fs 端点：飞书（Lark）云文档导出下载。
//!
//! 对应 `GET /api/fs/lark/export/download`（AList OpenAPI 规范未收录该端点，
//! 见 AList 服务端路由定义与 lark 模块的 `LarkExportDownload` 处理器）。
//! 目标路径的存储须为 Lark 存储；服务端将导出文件二进制直接作为响应体
//! 流式返回（**不走 JSON 响应**），因此本端点手写实现、不使用
//! [`EndpointRequest`](alist_client_derive::EndpointRequest) 派生宏：
//! 仅提供 [`Request::send_bytes`] 返回原始响应字节，不生成语义错误的
//! `send`/`IntoFuture`。
//!
//! # Version
//!
//! - 自 AList `v3.61.0` 起提供本接口。

/// 飞书云文档导出下载请求构建器。
///
/// 通过 [`Fs::lark_export_download`](super::Fs::lark_export_download) 创建。
/// 本端点响应为二进制流，请使用
/// [`.send_bytes().await`](Request::send_bytes)；构建器未实现
/// [`IntoFuture`]，无法直接 `.await`。
#[must_use = "请求构建器不会自动发送请求，请调用 `.send_bytes().await`"]
pub struct Request<'a> {
    client: &'a crate::Client,
    /// 目标文件路径（必选参数）。
    ///
    /// 飞书云文档相对于某存储挂载点的完整路径。
    path: String,
    /// 飞书云文档的文件令牌（必选参数）。
    ///
    /// 对应查询参数 `file_token`，服务端以它调用飞书导出下载接口。
    file_token: String,
    /// 目标路径的目录密码（可选参数）。
    ///
    /// 路径受元信息密码保护时必填，错误时服务端返回 403。
    password: Option<String>,
    /// 导出文件的保存文件名（可选参数）。
    ///
    /// 对应查询参数 `filename`；不提供时服务端使用驱动返回的文件名，
    /// 仍为空时回退为路径的基名。
    filename: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走链式 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        path: impl Into<String>,
        file_token: impl Into<String>,
    ) -> Self {
        Self {
            client,
            path: path.into(),
            file_token: file_token.into(),
            password: None,
            filename: None,
        }
    }

    /// 目标路径的目录密码（可选参数）。
    ///
    /// 路径受元信息密码保护时必填，错误时服务端返回 403。
    #[inline]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// 导出文件的保存文件名（可选参数）。
    ///
    /// 不提供时服务端使用驱动返回的文件名，仍为空时回退为路径的基名。
    #[inline]
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
        self
    }

    /// 构建底层 `GET /api/fs/lark/export/download` 请求。
    ///
    /// 查询参数：`path`/`file_token` 必选，`password`/`filename` 缺省时跳过。
    /// 不含认证头（认证由 `Client::execute_bytes` 发送时注入）。语义与派生宏
    /// 生成的 `build_request` 一致，`pub(crate)` 供端点模块与测试使用。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let mut params: Vec<(&str, &str)> =
            vec![("path", &self.path), ("file_token", &self.file_token)];
        if let Some(password) = &self.password {
            params.push(("password", password));
        }
        if let Some(filename) = &self.filename {
            params.push(("filename", filename));
        }
        self.client
            .request(reqwest::Method::GET, "/api/fs/lark/export/download")
            .query(&params)
    }

    /// 发送请求并返回导出文件的原始字节。
    ///
    /// 走 `Client::execute_bytes`：限速、认证注入与 HTTP 状态检查与 JSON 端点
    /// 一致，但**不做响应解码**，响应体字节原样返回（服务端将导出文件二进制
    /// 直接作为响应体，不走 JSON 响应；响应的 `Content-Type` 与
    /// `Content-Disposition` 头由服务端依据文件名决定）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非 2xx 状态码（如 `path`/`file_token` 未提供、
    /// 目标存储不是 Lark 存储、密码错误或导出下载失败）时，返回 [`crate::Error`]。
    /// 与 JSON 端点不同，本方法收到 401/403 时**不做**凭据自动重登重试。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let bytes = client.fs().lark_export_download("/lark/notes", "docx-token")
    ///     .filename("notes.docx")
    ///     .send_bytes()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_bytes(&self) -> crate::Result<Vec<u8>> {
        self.client.execute_bytes(self.build_request()).await
    }
}

impl<'a> super::Fs<'a> {
    /// 飞书（Lark）云文档导出下载。
    ///
    /// 对应 AList `GET /api/fs/lark/export/download`；响应为导出文件的
    /// **二进制流**（不走 JSON 响应）。目标路径的存储须为 Lark 存储，
    /// 否则服务端返回 400 错误。导出文件的保存文件名由 `filename` 参数决定，
    /// 不提供时服务端使用驱动返回的文件名（仍为空时回退为路径基名）。
    /// 数据来源：AList 服务端路由定义与 lark 模块的 `LarkExportDownload`
    /// 处理器（该端点不在 AList OpenAPI 规范中，以 AList 服务端实现为准）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标飞书云文档路径。
    /// * `file_token` - 飞书云文档的文件令牌（导出下载凭据）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；调用 [`Request::send_bytes`] 后返回
    /// 导出文件的原始字节。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非 2xx 状态码时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.61.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let bytes = client.fs().lark_export_download("/lark/notes", "docx-token")
    ///     .filename("notes.docx")
    ///     .send_bytes()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn lark_export_download(
        &self,
        path: impl Into<String>,
        file_token: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, path, file_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：`GET` 方法、路径与全部查询参数（可选参数缺省时被跳过）。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/lark/notes", "docx-token")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/lark/export/download"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("path=%2Flark%2Fnotes"),
            "URL 应包含 path 查询参数: {url}"
        );
        assert!(
            url.contains("file_token=docx-token"),
            "URL 应包含 file_token 查询参数: {url}"
        );
        assert!(
            !url.contains("password="),
            "缺省的 password 不应出现: {url}"
        );
        assert!(
            !url.contains("filename="),
            "缺省的 filename 不应出现: {url}"
        );
    }

    /// 请求形状：可选参数经链式 setter 后进入查询串。
    #[test]
    fn build_request_carries_optional_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/lark/notes", "docx-token")
            .password("secret")
            .filename("notes.docx")
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(
            url.contains("password=secret"),
            "URL 应包含 password: {url}"
        );
        assert!(
            url.contains("filename=notes.docx"),
            "URL 应包含 filename: {url}"
        );
    }

    /// 收发路径：`send_bytes` 返回原始响应字节（不走 JSON 响应解码）。
    #[tokio::test]
    async fn send_bytes_returns_raw_export_body() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/fs/lark/export/download")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("path".into(), "/lark/notes".into()),
                mockito::Matcher::UrlEncoded("file_token".into(), "docx-token".into()),
                mockito::Matcher::UrlEncoded("filename".into(), "notes.docx".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body("PK-fake-docx-export")
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let bytes = Request::new(&client, "/lark/notes", "docx-token")
            .filename("notes.docx")
            .send_bytes()
            .await
            .unwrap();
        assert_eq!(bytes, b"PK-fake-docx-export");

        mock.assert_async().await;
    }

    /// 错误路径：服务端非 2xx（如目标存储不是 Lark）时返回状态码错误。
    #[tokio::test]
    async fn send_bytes_surfaces_http_error_status() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/fs/lark/export/download")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("path".into(), "/local/file".into()),
                mockito::Matcher::UrlEncoded("file_token".into(), "docx-token".into()),
            ]))
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":400,"message":"lark export download is not supported for this storage","data":null}"#,
            )
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let err = Request::new(&client, "/local/file", "docx-token")
            .send_bytes()
            .await
            .unwrap_err();
        assert!(
            matches!(err, crate::Error::HttpStatus { status, .. } if status.as_u16() == 400),
            "应返回 HTTP 状态码错误: {err:?}"
        );

        mock.assert_async().await;
    }
}
