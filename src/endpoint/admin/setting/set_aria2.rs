//! admin-setting 端点：配置 aria2。
//!
//! 对应 `POST /api/admin/setting/set_aria2`；请求体为 `uri`/`secret` 两字段对象，
//! 响应 `data` 为 aria2 版本字符串，以 `String` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 配置 aria2 请求构建器。
///
/// 通过 [`Setting::set_aria2`](super::Setting::set_aria2) 创建；
/// `uri`/`secret` 均为必选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/set_aria2", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// aria2 JSON-RPC 地址（必选），例如 `http://localhost:6800/jsonrpc`。
    uri: String,
    /// aria2 RPC 密钥（必选；未设置时传空字符串）。
    secret: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        uri: impl Into<String>,
        secret: impl Into<String>,
    ) -> Self {
        Self {
            client,
            uri: uri.into(),
            secret: secret.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 配置 aria2 离线下载。
    ///
    /// 对应 AList `POST /api/admin/setting/set_aria2`；保存 `aria2_uri`/`aria2_secret`
    /// 设置项后，服务端会立即初始化 aria2 连接，成功时响应 `data` 为 aria2 版本字符串
    /// （如 `1.36.0`），连接失败时返回非成功响应。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/set_aria2` 与
    /// `examples/alist/server/handles/offline_download.go`（实现为 `SetAria2`，
    /// 请求体 `SetAria2Req` 仅含 `uri`/`secret` 两字段）。
    ///
    /// # Arguments
    ///
    /// * `uri` - aria2 JSON-RPC 地址。
    /// * `secret` - aria2 RPC 密钥；未设置时传空字符串。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 aria2 版本 `String`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如 aria2 连接初始化失败）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let version = client
    ///     .admin()
    ///     .setting()
    ///     .set_aria2("http://localhost:6800/jsonrpc", "SECRET")
    ///     .await?;
    /// println!("aria2 版本: {version}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_aria2(&self, uri: impl Into<String>, secret: impl Into<String>) -> Request<'a> {
        Request::new(self.client, uri, secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"uri":...,"secret":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "http://localhost:6800/jsonrpc", "SECRET")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/set_aria2"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"uri":"http://localhost:6800/jsonrpc","secret":"SECRET"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码 aria2 版本字符串。
    #[tokio::test]
    async fn send_returns_version_string() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":"1.36.0"}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let version = Request::new(&client, "http://localhost:6800/jsonrpc", "")
            .send()
            .await
            .unwrap();
        assert_eq!(version, "1.36.0");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/setting/set_aria2 "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""uri":"http://localhost:6800/jsonrpc""#),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""secret":""#), "{}", recorded[0]);
    }
}
