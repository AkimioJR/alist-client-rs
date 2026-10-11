//! admin-setting 端点：配置 115 网盘离线下载。
//!
//! 对应 `POST /api/admin/setting/set_115`；请求体为 `temp_dir` 单字段对象，
//! 响应 `data` 为 `"ok"` 字符串，以 `String` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.42.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 配置 115 网盘离线下载请求构建器。
///
/// 通过 [`Setting::set_115`](super::Setting::set_115) 创建；
/// `temp_dir` 为必选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/set_115", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 115 网盘离线下载临时目录（必选参数）。
    ///
    /// 为 115 存储内的路径；传空字符串表示未指定（服务端跳过存储校验）。
    temp_dir: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, temp_dir: impl Into<String>) -> Self {
        Self {
            client,
            temp_dir: temp_dir.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 配置 115 网盘离线下载。
    ///
    /// 对应 AList `POST /api/admin/setting/set_115`；保存 `115_temp_dir`
    /// 设置项后，服务端会立即初始化 115 离线下载工具，成功时响应 `data` 为
    /// `"ok"` 字符串。`temp_dir` 非空时，服务端会校验该路径所在存储已初始化
    /// 且驱动类型为 115 网盘，否则返回非成功响应。
    /// 数据来源：AList 服务端离线下载处理模块（实现为 `Set115`，
    /// 请求体 [`SetTempDirRequest`](crate::schema::admin::setting::SetTempDirRequest)
    /// 仅含 `temp_dir` 一个字段，与 `set_pikpak`/`set_thunder`/`set_guangyapan`/
    /// `set_123_open` 共享同一形状）。
    ///
    /// # Arguments
    ///
    /// * `temp_dir` - 115 网盘离线下载临时目录。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `"ok"` 字符串。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如存储不存在、存储未初始化、驱动类型不支持或离线下载工具初始化失败）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.42.0` 起提供本接口。
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
    ///     .set_115("/115/offline-download")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_115(&self, temp_dir: impl Into<String>) -> Request<'a> {
        Request::new(self.client, temp_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"temp_dir":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/115/offline-download")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/set_115"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"temp_dir":"/115/offline-download"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `"ok"` 字符串。
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

        let data = Request::new(&client, "/115/offline-download")
            .send()
            .await
            .unwrap();
        assert_eq!(data, "ok");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/setting/set_115 "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""temp_dir":"/115/offline-download""#),
            "{}",
            recorded[0]
        );
    }
}
