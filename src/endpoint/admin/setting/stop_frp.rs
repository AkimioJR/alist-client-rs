//! admin-setting 端点：停止 FRP。
//!
//! 对应 `POST /api/admin/setting/stop_frp`；无请求参数，
//! 响应 `data` 为停止后的 FRP 状态字符串，以 `String` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.60.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 停止 FRP 请求构建器。
///
/// 通过 [`Setting::stop_frp`](super::Setting::stop_frp) 创建，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/stop_frp", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Setting<'a> {
    /// 停止 FRP 内网穿透。
    ///
    /// 对应 AList `POST /api/admin/setting/stop_frp`；服务端优雅关闭 FRP 客户端，
    /// 成功时响应 `data` 为停止后的 FRP 状态字符串（正常应为 `stopped`）。
    /// 数据来源：AList 服务端 handles.Setting 模块（实现为 `StopFRP`）。
    ///
    /// # Arguments
    ///
    /// 无。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 FRP 状态 `String`。
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
    /// let status = client.admin().setting().stop_frp().await?;
    /// println!("FRP 状态: {status}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn stop_frp(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径断言（无查询参数、无请求体）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/stop_frp"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应有请求体");
    }

    /// 收发路径：mock 服务器记录请求原文并解码状态字符串。
    #[tokio::test]
    async fn send_returns_status_string() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":"stopped"}"#;
        let mock = server
            .mock("POST", "/api/admin/setting/stop_frp")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let status = Request::new(&client).send().await.unwrap();
        assert_eq!(status, "stopped");

        mock.assert_async().await;
    }
}
