//! admin-setting 端点：列出设置。
//!
//! 对应 `GET /api/admin/setting/list`；响应 `data` 为设置项数组（非分页结构），
//! 以 `Vec<Setting>` 作为端点模型。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::Setting;

/// 列出设置请求构建器。
///
/// 通过 [`Setting::list`](super::Setting::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/setting/list", model = Vec<Setting>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 单个设置分组编号（可选），例如 `"1"`（站点）。
    #[query]
    group: Option<String>,
    /// 逗号分隔的多个设置分组编号（可选），例如 `"5,0"`；与 `group` 同时设置时服务端优先使用本参数。
    #[query]
    groups: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            group: None,
            groups: None,
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 列出全部设置项。
    ///
    /// 对应 AList `GET /api/admin/setting/list`；成功时响应 `data` 为设置项数组
    /// （直接为数组，非分页结构）。不设置分组时返回全部设置。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/list` 与
    /// `examples/alist/server/handles/setting.go`（实现为 `ListSettings`）。
    ///
    /// # Arguments
    ///
    /// 无。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// `Vec<[`Setting`]>`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// // 全部设置项
    /// let settings = client.admin().setting().list().await?;
    /// // 仅站点分组（分组编号见 Setting::group 字段文档）
    /// let site_settings = client.admin().setting().list().group("1").await?;
    /// println!("{settings:?}");
    /// println!("{site_settings:?}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、路径与查询串断言（`group`/`groups` 均为可选查询参数）。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();

        // 不带任何可选参数：不应有查询串
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains("group"), "未设置分组时不应有查询参数: {url}");

        // group：单分组
        let built = Request::new(&client)
            .group("1")
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(url.contains("group=1"), "URL 应包含 group 查询参数: {url}");
        assert!(!url.contains("groups="), "不应误写为 groups: {url}");

        // groups：多分组（逗号按查询串规则编码为 %2C）
        let built = Request::new(&client)
            .groups("5,0")
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(
            url.contains("groups=5%2C0"),
            "URL 应包含 groups 查询参数: {url}"
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码设置项数组。
    #[tokio::test]
    async fn send_returns_settings_array() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"key":"token","value":"alist-2a","help":"","type":"string","options":"","group":0,"flag":1}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let settings = Request::new(&client).send().await.unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].key, "token");
        assert_eq!(settings[0].value, "alist-2a");
        assert_eq!(settings[0].value_type, "string");
        assert_eq!(settings[0].group, 0);
        assert_eq!(settings[0].flag, 1);
        assert_eq!(settings[0].index, 0);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/setting/list"),
            "{}",
            recorded[0]
        );
    }
}
