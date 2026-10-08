//! admin-setting 端点：按多个键批量查询设置。
//!
//! 对应 `GET /api/admin/setting/get` 的 `keys` 查询参数形态（服务端在 `key`
//! 为空时的批量分支）；响应 `data` 为设置项 **数组**，以 `Vec<Setting>` 作为端点模型。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::Setting;

/// 按多个键批量查询设置请求构建器。
///
/// 通过 [`Setting::get_by_keys`](super::Setting::get_by_keys) 创建；`keys` 为必选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/setting/get", model = Vec<Setting>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 逗号分隔的多个设置键（必选参数）。
    ///
    /// 例如 `site_title,announcements`。
    #[query]
    keys: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, keys: impl Into<String>) -> Self {
        Self {
            client,
            keys: keys.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 按逗号分隔的多个键批量查询设置项。
    ///
    /// 对应 AList `GET /api/admin/setting/get`（`keys` 查询参数形态）；成功时响应
    /// `data` 为设置项数组。服务端（`GetSetting`）在 `key` 查询参数为空时走本批量分支，
    /// 逐键查询并整体返回；任一键不存在时整个请求返回非成功响应（不会部分返回）。
    /// 与 [`Setting::get`](super::Setting::get) 的单键形态互斥：本端点只发送 `keys`，
    /// 不携带 `key` 参数。
    /// 数据来源：AList OpenAPI 规范的 `admin/setting/get`（`keys` 参数）与
    /// AList 服务端 handles.Setting 模块（实现为 `GetSetting`，批量分支经
    /// `op.GetSettingItemInKeys` 逐键查询）。
    ///
    /// # Arguments
    ///
    /// * `keys` - 逗号分隔的多个设置键，例如 `site_title,announcements`。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// `Vec<[`Setting`]>`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如任一键不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let settings = client
    ///     .admin()
    ///     .setting()
    ///     .get_by_keys("site_title,announcements")
    ///     .await?;
    /// println!("{settings:?}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get_by_keys(&self, keys: impl Into<String>) -> Request<'a> {
        Request::new(self.client, keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、路径与 `keys` 查询参数断言（逗号按查询串规则编码为 %2C，
    /// 且不携带单键形态的 `key` 参数）。
    #[test]
    fn build_request_composes_method_url_and_keys_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "site_title,announcements")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/get"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("keys=site_title%2Cannouncements"),
            "URL 应包含 keys 查询参数: {url}"
        );
        assert!(!url.contains("key="), "批量形态不应携带单键参数 key: {url}");
    }

    /// 收发路径：mock 服务器记录请求原文并解码设置项数组。
    #[tokio::test]
    async fn send_returns_settings_array() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"key":"site_title","value":"AList","help":"","type":"string","options":"","group":1,"flag":0,"index":3},{"key":"announcements","value":"","help":"","type":"text","options":"","group":1,"flag":0,"index":4}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let settings = Request::new(&client, "site_title,announcements")
            .send()
            .await
            .unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings[0].key, "site_title");
        assert_eq!(settings[0].value, "AList");
        assert_eq!(settings[0].value_type, "string");
        assert_eq!(settings[0].group, 1);
        assert_eq!(settings[0].flag, 0);
        assert_eq!(settings[0].index, 3);
        assert_eq!(settings[1].key, "announcements");
        assert_eq!(settings[1].value_type, "text");
        assert_eq!(settings[1].index, 4);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/setting/get?keys=site_title%2Cannouncements"),
            "{}",
            recorded[0]
        );
    }
}
