//! public端点：连通性检测。
//!
//! 对应 `GET /ping`；服务端以纯文本 `pong` 应答（`examples/alist/server/router.go:30-32`
//! 直接 `c.String(200, "pong")`），**不走 JSON 响应**，因此本端点手写实现、不使用
//! [`EndpointRequest`](alist_client_derive::EndpointRequest) 派生宏：
//! 仅提供 [`Request::send_text`] 返回原始响应文本，不生成语义错误的
//! `send`/`IntoFuture`。端点文件模板与命名约定见 `docs/design.md`。

/// ping 连通性检测请求构建器。
///
/// 通过 [`Public::ping`](super::Public::ping) 创建。
/// 本端点响应为纯文本，请使用 [`.send_text().await`](Request::send_text)；
/// 构建器未实现 [`IntoFuture`]，无法直接 `.await`。
#[must_use = "请求构建器不会自动发送请求，请调用 `.send_text().await`"]
pub struct Request<'a> {
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无必选业务参数，也无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }

    /// 构建 `GET /ping` 请求；不含认证头（认证由 `Client::execute_text`
    /// 发送时注入）。语义与派生宏生成的 `build_request` 一致，`pub(crate)` 供端点模块与测试使用。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        self.client.request(reqwest::Method::GET, "/ping")
    }

    /// 发送 ping 请求并返回原始响应文本。
    ///
    /// 正常情况下返回 `pong`。本方法走 `Client::execute_text`：
    /// 与 JSON 端点一致地执行客户端限速、认证注入与 HTTP 状态检查，但**不做响应解码**，
    /// 响应体原样返回。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非 2xx 状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let pong = client.public().ping().send_text().await?;
    /// assert_eq!(pong, "pong");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_text(&self) -> crate::Result<String> {
        self.client.execute_text(self.build_request()).await
    }
}

impl<'a> super::Public<'a> {
    /// ping 连通性检测。
    ///
    /// 对应 AList `GET /ping`；响应为**纯文本** `pong`，不经过 JSON 响应。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/ping`（示例响应 `pong`）与
    /// `examples/alist/server/router.go:30-32`（`g.Any("/ping", ...)`，
    /// 读端点按 GET 处理）。
    ///
    /// # Arguments
    ///
    /// 无参数；本端点不接收任何业务参数，请求不含查询串与请求体。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；调用 [`Request::send_text`] 后返回站点应答文本。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非 2xx 状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let pong = client.public().ping().send_text().await?;
    /// assert_eq!(pong, "pong");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn ping(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) URL/方法断言：`GET /ping`（无查询串、无请求体）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        assert_eq!(built.url().as_str(), "https://alist.example/ping");
        assert_eq!(built.url().query(), None, "本端点无查询参数");
        assert!(built.body().is_none(), "本端点无请求体");
    }

    /// 2) 收发路径断言：`send_text` 返回原始文本 `pong`（不走响应解码）。
    #[tokio::test]
    async fn send_text_returns_plain_pong_body() {
        use crate::test_support::{MockResponse, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 200 OK",
                body: "pong".to_string(),
            }],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let text = Request::new(&client).send_text().await.unwrap();
        assert_eq!(text, "pong");
    }

    /// 3) HTTP 非 2xx 时按状态码报错（而非尝试解析 JSON 响应）。
    #[tokio::test]
    async fn send_text_maps_http_error_status() {
        use crate::test_support::{MockResponse, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 500 Internal Server Error",
                body: "boom".to_string(),
            }],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let err = Request::new(&client).send_text().await.unwrap_err();
        assert!(
            matches!(err, crate::Error::HttpStatus { status, .. } if status.as_u16() == 500),
            "应返回 HttpStatus 错误: {err:?}"
        );
    }
}
