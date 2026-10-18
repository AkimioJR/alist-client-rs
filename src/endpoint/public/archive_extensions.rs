//! public 端点：查询支持的压缩包扩展名。
//!
//! 对应 `GET /api/public/archive_extensions`（服务端路由注册为 `g.Any`，
//! 读端点按 GET 处理）；响应 `data` 为已注册解压驱动支持的压缩包扩展名
//! 字符串数组 [`ArchiveExtensionsResponse`]（自带前导点号，如 `.7z`、`.iso`；
//! 集合由服务端编译内置的解压驱动决定，且无序——服务端遍历 Go map，
//! 返回顺序不固定）。
//! 该端点未被 AList v3 OpenAPI 规范收录，API 形状以 AList 服务端实现为准。
//!
//! # Version
//!
//! - 自 AList `v3.42.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::public::ArchiveExtensionsResponse;

/// 查询压缩包扩展名请求构建器。
///
/// 通过 [`Public::archive_extensions`](super::Public::archive_extensions) 创建。
/// 本端点无业务参数（也无可选参数），直接 `.await` 执行强类型解码，
/// 或 [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/public/archive_extensions",
    model = ArchiveExtensionsResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无必选业务参数，也无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Public<'a> {
    /// 查询服务端支持的压缩包扩展名。
    ///
    /// 对应 AList `GET /api/public/archive_extensions`；无需认证，响应 `data`
    /// 为已注册解压驱动支持的压缩包扩展名字符串数组（自带前导点号，如
    /// `.7z`、`.iso`、`.rar`、`.tar`；集合由服务端编译内置的解压驱动决定，
    /// 且返回顺序不固定）。可用于前端按扩展名判断能否在线预览/解压。
    /// 数据来源：AList 服务端路由定义与 archive 工具注册表的
    /// `ArchiveExtensions` 处理器（该端点不在 AList v3 OpenAPI 规范中，
    /// 以 AList 服务端实现为准）。
    ///
    /// # Arguments
    ///
    /// 无参数；本端点不接收任何业务参数，请求不含查询串与请求体。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`ArchiveExtensionsResponse`]（压缩包扩展名列表）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.42.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let extensions = client.public().archive_extensions().await?;
    /// println!("支持在线预览/解压的扩展名: {extensions:?}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn archive_extensions(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、正确路径、无查询串、无请求体头。
    #[test]
    fn build_request_composes_get_url_without_query_or_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.ends_with("/api/public/archive_extensions"),
            "URL 应为 /api/public/archive_extensions: {url}"
        );
        assert!(!url.contains('?'), "本端点不应携带查询参数: {url}");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的 GET 请求不应携带 Content-Type 头"
        );
    }

    /// 收发路径：mock 服务器返回扩展名数组，验证请求行与响应解码。
    #[tokio::test]
    async fn send_decodes_archive_extensions() {
        // 示例形状取自 AList 服务端 archive 工具注册表的实际返回
        // （键即扩展名，自带前导点号；顺序不固定）
        let body = r#"{"code":200,"message":"success","data":[".7z",".iso",".rar",".tar"]}"#;
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/public/archive_extensions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let extensions: ArchiveExtensionsResponse = Request::new(&client).send().await.unwrap();
        assert_eq!(
            extensions,
            vec![
                ".7z".to_owned(),
                ".iso".to_owned(),
                ".rar".to_owned(),
                ".tar".to_owned()
            ]
        );

        mock.assert_async().await;
    }
}
