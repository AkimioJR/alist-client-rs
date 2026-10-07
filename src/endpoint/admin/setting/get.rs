//! admin-setting 端点：查询单个设置。
//!
//! 对应 `GET /api/admin/setting/get` 的 `key` 查询参数形态；响应 `data` 为单个设置项。
//! 按逗号分隔的多个键批量查询（`keys` 查询参数形态）见 [`super::get_by_keys`]。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::Setting;

/// 查询设置请求构建器。
///
/// 通过 [`Setting::get`](super::Setting::get) 创建；`key` 为必选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/setting/get", model = Setting)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 设置键（必选），例如 `site_title`。
    #[query]
    key: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, key: impl Into<String>) -> Self {
        Self {
            client,
            key: key.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 按键查询单个设置项。
    ///
    /// 对应 AList `GET /api/admin/setting/get`（`key` 查询参数形态）；成功时响应
    /// `data` 为单个设置项。按逗号分隔的多个键批量查询请使用
    /// [`Setting::get_by_keys`](super::Setting::get_by_keys)。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/get` 与
    /// `examples/alist/server/handles/setting.go`（实现为 `GetSetting`）。
    ///
    /// # Arguments
    ///
    /// * `key` - 设置键，例如 `site_title`。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Setting`](crate::schema::admin::setting::Setting)。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如键不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let setting = client.admin().setting().get("site_title").await?;
    /// println!("{setting:?}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self, key: impl Into<String>) -> Request<'a> {
        Request::new(self.client, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、路径与 `key` 查询参数断言。
    #[test]
    fn build_request_composes_method_url_and_key_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "site_title")
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
            url.contains("key=site_title"),
            "URL 应包含 key 查询参数: {url}"
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码单个设置项。
    #[tokio::test]
    async fn send_returns_single_setting() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"key":"hide_files","value":"/\\/README.md/i","help":"","type":"text","options":"","group":4,"flag":0}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let setting = Request::new(&client, "hide_files").send().await.unwrap();
        assert_eq!(setting.key, "hide_files");
        assert_eq!(setting.value, "/\\/README.md/i");
        assert_eq!(setting.value_type, "text");
        assert_eq!(setting.group, 4);
        assert_eq!(setting.flag, 0);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/setting/get?key=hide_files"),
            "{}",
            recorded[0]
        );
    }
}
