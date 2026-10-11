//! public 端点：查询离线下载工具。
//!
//! 对应 `GET /api/public/offline_download_tools`（服务端路由注册为 `g.Any`，
//! 读端点按 GET 处理）；响应 `data` 为就绪离线下载工具名的字符串数组
//! [`OfflineDownloadToolsResponse`]（服务端已按字典序排序）。
//! 该端点未被 AList v3 OpenAPI 规范收录，API 形状以 AList 服务端实现为准。
//! OpenList v4 服务端额外接受可选的 `path` 查询参数（把目标存储的原生
//! 离线下载工具并入结果），AList 服务端不读取任何查询参数；本客户端按
//! AList 形状建模，不发送查询串。
//!
//! # Version
//!
//! - 自 AList `v3.29.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::public::OfflineDownloadToolsResponse;

/// 查询离线下载工具请求构建器。
///
/// 通过 [`Public::offline_download_tools`](super::Public::offline_download_tools)
/// 创建。本端点无业务参数（也无可选参数），直接 `.await` 执行强类型解码，
/// 或 [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/public/offline_download_tools",
    model = OfflineDownloadToolsResponse
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
    /// 查询服务端支持的离线下载工具。
    ///
    /// 对应 AList `GET /api/public/offline_download_tools`；无需认证，响应
    /// `data` 为就绪离线下载工具名的字符串数组（服务端已按字典序排序，
    /// 仅包含服务端判定就绪的工具，如 `115 Cloud`、`aria2`、`qBittorrent`）。
    /// 数据来源：AList 服务端路由定义与 offline download 工具管理器的
    /// `OfflineDownloadTools` 处理器（该端点不在 AList v3 OpenAPI 规范中，
    /// 以 AList 服务端实现为准）。
    ///
    /// # Arguments
    ///
    /// 无参数；本端点不接收任何业务参数，请求不含查询串与请求体。
    /// OpenList v4 服务端额外支持可选的 `path` 查询参数（把目标存储的原生
    /// 离线下载工具并入结果），AList 服务端不读取任何查询参数，本客户端
    /// 按 AList 形状建模。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`OfflineDownloadToolsResponse`]（就绪工具名列表）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.29.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// for tool in client.public().offline_download_tools().await? {
    ///     println!("就绪的离线下载工具: {tool}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn offline_download_tools(&self) -> Request<'a> {
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
            url.ends_with("/api/public/offline_download_tools"),
            "URL 应为 /api/public/offline_download_tools: {url}"
        );
        assert!(!url.contains('?'), "本端点不应携带查询参数: {url}");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的 GET 请求不应携带 Content-Type 头"
        );
    }

    /// 收发路径：mock 服务器返回就绪工具名数组，验证请求行与响应解码。
    #[tokio::test]
    async fn send_decodes_ready_tool_names() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        // 示例形状取自 AList 服务端 offline download 工具管理器的实际返回
        // （按字典序排序的就绪工具名）
        let body = r#"{"code":200,"message":"success","data":["115 Cloud","aria2","qBittorrent"]}"#;
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let tools: OfflineDownloadToolsResponse = Request::new(&client).send().await.unwrap();
        assert_eq!(
            tools,
            vec![
                "115 Cloud".to_owned(),
                "aria2".to_owned(),
                "qBittorrent".to_owned()
            ]
        );

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/public/offline_download_tools "),
            "{}",
            recorded[0]
        );
    }
}
