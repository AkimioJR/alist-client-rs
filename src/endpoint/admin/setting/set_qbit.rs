//! admin-setting 端点：配置 qBittorrent。
//!
//! 对应 `POST /api/admin/setting/set_qbit`；请求体为 `url`/`seedtime` 两字段对象，
//! 响应 `data` 为 `"ok"` 字符串，以 `String` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 配置 qBittorrent 请求构建器。
///
/// 通过 [`Setting::set_qbit`](super::Setting::set_qbit) 创建；
/// `url`/`seedtime` 均为必选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/set_qbit", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// qBittorrent WebUI 地址（必选，可内嵌凭据），例如 `http://user:pass@localhost:8080/`。
    url: String,
    /// 做种时间（必选，字符串形式的数值），例如 `"30"`。
    seedtime: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        url: impl Into<String>,
        seedtime: impl Into<String>,
    ) -> Self {
        Self {
            client,
            url: url.into(),
            seedtime: seedtime.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 配置 qBittorrent 离线下载。
    ///
    /// 对应 AList `POST /api/admin/setting/set_qbit`；保存 `qbittorrent_url`/
    /// `qbittorrent_seedtime` 设置项后，服务端会立即初始化 qBittorrent 连接，
    /// 成功时响应 `data` 为 `"ok"` 字符串，连接失败时返回非成功信封。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/set_qbit` 与
    /// `examples/alist/server/handles/offline_download.go`（实现为 `SetQbittorrent`，
    /// 请求体 `SetQbittorrentReq` 仅含 `url`/`seedtime` 两字段）。
    ///
    /// # Arguments
    ///
    /// * `url` - qBittorrent WebUI 地址，可内嵌用户名与密码。
    /// * `seedtime` - 做种时间（字符串形式的数值）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `String`（`"ok"`）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如 qBittorrent 连接初始化失败）时，返回 [`crate::Error`]。
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
    ///     .set_qbit("http://admin:password@localhost:8080/", "30")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_qbit(&self, url: impl Into<String>, seedtime: impl Into<String>) -> Request<'a> {
        Request::new(self.client, url, seedtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"url":...,"seedtime":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "http://user:pass@localhost:8080/", "30")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/set_qbit"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"url":"http://user:pass@localhost:8080/","seedtime":"30"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码响应字符串。
    #[tokio::test]
    async fn send_returns_ok_string() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":"ok"}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let status = Request::new(&client, "http://user:pass@localhost:8080/", "30")
            .send()
            .await
            .unwrap();
        assert_eq!(status, "ok");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/setting/set_qbit "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""url":"http://user:pass@localhost:8080/""#),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""seedtime":"30""#),
            "{}",
            recorded[0]
        );
    }
}
