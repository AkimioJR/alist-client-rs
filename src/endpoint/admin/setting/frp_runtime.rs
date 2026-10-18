//! admin-setting 端点：查询 FRP 运行时状态。
//!
//! 对应 `GET /api/admin/setting/frp_runtime`；查询串可携带可选 `limit`
//! （日志条数上限，服务端默认 200），响应 `data` 为 [`FrpRuntimeInfo`]。
//!
//! # Version
//!
//! - 自 AList `v3.60.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::FrpRuntimeInfo;

/// 查询 FRP 运行时状态请求构建器。
///
/// 通过 [`Setting::frp_runtime`](super::Setting::frp_runtime) 创建。可选参数使用
/// 链式 setter，直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/setting/frp_runtime",
    model = FrpRuntimeInfo
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 返回日志的最大条数（可选参数）。
    ///
    /// 服务端从日志文件尾部读取指定条数；不设置时服务端默认取 200 条。
    /// 非正数或超过服务端上限（300）时由服务端钳制处理。
    #[query]
    limit: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            limit: None,
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 查询 FRP 运行时状态与近期日志。
    ///
    /// 对应 AList `GET /api/admin/setting/frp_runtime`；成功时响应 `data` 为
    /// [`FrpRuntimeInfo`]（`status` 为 FRP 客户端状态，`logs` 为最近的运行日志行）。
    /// 数据来源：AList 服务端 handles.Setting 模块（实现为 `GetFRPRuntime`，
    /// 响应模型对应 AList 服务端 FRP 管理模块的 `RuntimeInfo`）。
    ///
    /// # Arguments
    ///
    /// * 无必选参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 [`FrpRuntimeInfo`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.60.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// // 最近 50 条日志
    /// let runtime = client
    ///     .admin()
    ///     .setting()
    ///     .frp_runtime()
    ///     .limit(50)
    ///     .await?;
    /// println!("FRP 状态: {}，日志 {} 条", runtime.status, runtime.logs.len());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn frp_runtime(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、路径与 `limit` 查询串断言（可选参数，缺省时不出现）。
    #[test]
    fn build_request_composes_method_url_and_limit_query() {
        let client = crate::Client::new("https://alist.example").unwrap();

        // 不带 limit：不应有查询串
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/frp_runtime"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains("limit="),
            "未设置 limit 时不应有查询参数: {url}"
        );

        // 带 limit
        let built = Request::new(&client)
            .limit(50)
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(url.contains("limit=50"), "URL 应包含 limit 查询参数: {url}");
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `FrpRuntimeInfo`。
    #[tokio::test]
    async fn send_returns_runtime_info() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"status":"running","logs":["[2026-01-01T00:00:00+08:00] initialized","[2026-01-01T00:00:01+08:00] service started"]}}"#;
        let mock = server
            .mock("GET", "/api/admin/setting/frp_runtime?limit=50")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let runtime = Request::new(&client).limit(50).send().await.unwrap();
        assert_eq!(runtime.status, "running");
        assert_eq!(runtime.logs.len(), 2);
        assert!(runtime.logs[1].ends_with("service started"));

        mock.assert_async().await;
    }
}
