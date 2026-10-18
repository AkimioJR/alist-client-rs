//! admin-setting 端点：配置 Transmission。
//!
//! 对应 `POST /api/admin/setting/set_transmission`；请求体为 `uri`/`seedtime`
//! 两字段对象，响应 `data` 为 `"ok"` 字符串，以 `String` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.38.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 配置 Transmission 请求构建器。
///
/// 通过 [`Setting::set_transmission`](super::Setting::set_transmission) 创建；
/// `uri`/`seedtime` 均为必选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/setting/set_transmission",
    model = String
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// Transmission RPC 地址（必选参数）。
    ///
    /// 例如 `http://localhost:9091/transmission/rpc`。
    uri: String,
    /// 做种时间（必选参数）。
    ///
    /// 字符串形式的数值（分钟），未设置时传空字符串或 `"0"`。
    seedtime: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        uri: impl Into<String>,
        seedtime: impl Into<String>,
    ) -> Self {
        Self {
            client,
            uri: uri.into(),
            seedtime: seedtime.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 配置 Transmission 离线下载。
    ///
    /// 对应 AList `POST /api/admin/setting/set_transmission`；保存
    /// `transmission_uri`/`transmission_seedtime` 设置项后，服务端会立即初始化
    /// Transmission 连接，成功时响应 `data` 为 `"ok"` 字符串，连接失败时返回非成功响应。
    /// 数据来源：AList 服务端离线下载处理模块（实现为 `SetTransmission`，
    /// 请求体 [`SetTransmissionRequest`](crate::schema::admin::setting::SetTransmissionRequest)
    /// 仅含 `uri`/`seedtime` 两字段）。
    ///
    /// # Arguments
    ///
    /// * `uri` - Transmission RPC 地址。
    /// * `seedtime` - 做种时间（分钟，字符串形式数值）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `"ok"` 字符串。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如 Transmission 连接初始化失败）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.38.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client
    ///     .admin()
    ///     .setting()
    ///     .set_transmission("http://localhost:9091/transmission/rpc", "30")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_transmission(
        &self,
        uri: impl Into<String>,
        seedtime: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, uri, seedtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"uri":...,"seedtime":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "http://localhost:9091/transmission/rpc", "30")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/set_transmission"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"uri":"http://localhost:9091/transmission/rpc","seedtime":"30"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `"ok"` 字符串。
    #[tokio::test]
    async fn send_returns_ok_string() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":"ok"}"#;
        let mock = server
            .mock("POST", "/api/admin/setting/set_transmission")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let data = Request::new(&client, "http://localhost:9091/transmission/rpc", "30")
            .send()
            .await
            .unwrap();
        assert_eq!(data, "ok");

        mock.assert_async().await;
    }
}
