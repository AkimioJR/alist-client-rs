//! fs 端点：管理员获取文件原始直链。
//!
//! 对应 `POST /api/fs/link`（AList OpenAPI 规范未收录该端点；路由位于
//! 管理员中间件之后，仅管理员可用——直链可能携带 Cookie 等敏感请求头，
//! 供代理程序使用）。响应 `data` 为
//! [`DownloadLink`]，形状由 AList 服务端
//! fsmanage 模块 `Link` 处理器与 `model.Link` 决定；目标路径直接以传入的
//! 完整路径定位存储（不做用户主目录拼接）。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.25.1` 起，响应新增 `Concurrency`/`PartSize` 多线程分段
//!   下载参数（当时序列化键为 Go 字段名大写形式）；老版本不返回这两个字段。
//! - 从 AList `v3.26.0` 起，上述两字段序列化键改为小写 `concurrency`/
//!   `part_size`，且服务端对 `model.Link` 的范围读取器、文件句柄与 IP 缓存
//!   标记等内部字段显式忽略（不再参与 JSON 序列化）；更早版本可能额外输出
//!   这些字段（部分取值下服务端甚至会序列化失败）。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::DownloadLink;

/// 获取文件原始直链请求构建器。
///
/// 通过 [`Fs::link`](super::Fs::link) 创建。直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/link", model = DownloadLink)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件路径（必选参数）。
    ///
    /// 相对于某存储挂载点的完整路径；服务端不做用户主目录拼接，
    /// 直接以传入路径定位存储。
    path: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 获取文件原始直链（管理员）。
    ///
    /// 对应 AList `POST /api/fs/link`；仅管理员可用。响应 `data` 为
    /// [`DownloadLink`]，包含直链 URL、访问所需请求头与服务端缓存有效期；
    /// 仅本地代理的存储返回本站代理地址（此时无附加请求头与有效期）。
    /// 数据来源：AList 服务端路由定义与 fsmanage 模块的 `Link` 处理器
    /// （该端点不在 AList OpenAPI 规范中，以 AList 服务端实现为准）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标文件路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`DownloadLink`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如路径不存在）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.25.1` 起，响应新增 `Concurrency`/`PartSize` 多线程分段
    ///   下载参数（当时序列化键为 Go 字段名大写形式）；老版本不返回这两个字段。
    /// - 从 AList `v3.26.0` 起，上述两字段序列化键改为小写 `concurrency`/
    ///   `part_size`，且服务端对 `model.Link` 的范围读取器、文件句柄与 IP 缓存
    ///   标记等内部字段显式忽略（不再参与 JSON 序列化）；更早版本可能额外输出
    ///   这些字段（部分取值下服务端甚至会序列化失败）。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let link = client.fs().link("/local/movie.mkv").await?;
    /// println!("{} {:?} {:?}", link.url, link.header, link.expiration);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn link(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/local/movie.mkv")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/link"), "URL 应包含路径: {url}");
        assert!(
            built.url().query().is_none(),
            "本端点参数在 JSON 请求体，不应有查询串: {url}"
        );
    }

    /// 收发路径：mock 返回带请求头与有效期的直链，断言请求体与解码结果。
    #[tokio::test]
    async fn send_posts_path_and_decodes_link() {
        let mut server = mockito::Server::new_async().await;
        // 示例形状取自 AList 服务端 fsmanage 模块 `Link` 处理器的响应
        // （model.Link 的 JSON 序列化：Expiration 为无 tag 的 Go 字段名，
        // concurrency/part_size 为 v3.26.0+ 的小写键名）
        let body = r#"{"code":200,"message":"success","data":{"url":"https://example.com/d/file.mkv","header":{"User-Agent":["alist"]},"Expiration":3600000000000,"concurrency":4,"part_size":10485760}}"#;
        let mock = server
            .mock("POST", "/api/fs/link")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let link = Request::new(&client, "/local/movie.mkv")
            .send()
            .await
            .unwrap();
        assert_eq!(link.url, "https://example.com/d/file.mkv");
        assert_eq!(
            link.header.get("User-Agent").map(Vec::as_slice),
            Some(&["alist".to_owned()][..])
        );
        assert_eq!(link.expiration, Some(3_600_000_000_000));
        assert_eq!(link.concurrency, Some(4));
        assert_eq!(link.part_size, Some(10_485_760));

        mock.assert_async().await;
    }

    /// 收发路径：仅本地代理分支返回 `header: null` 与 `Expiration: null`，
    /// 分别归约为空映射与 [`None`]。
    #[tokio::test]
    async fn send_tolerates_null_header_and_expiration() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/fs/link")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"url":"http://127.0.0.1:5244/p/local/file.mkv?d&sign=x","header":null,"Expiration":null}}"#,
            )
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let link = Request::new(&client, "/local/file.mkv")
            .send()
            .await
            .unwrap();
        assert_eq!(link.url, "http://127.0.0.1:5244/p/local/file.mkv?d&sign=x");
        assert!(link.header.is_empty(), "null 的 header 归约为空映射");
        assert_eq!(link.expiration, None);

        mock.assert_async().await;
    }
}
