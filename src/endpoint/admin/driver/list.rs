//! admin-driver 端点：列出全部驱动配置模板。
//!
//! 对应 `GET /api/admin/driver/list`；`data` 为「驱动名 → 驱动模板」映射。
//! 驱动模板为异构复杂结构，按约定采用宽松模型
//! [`DriverListResponse`]
//! （`HashMap<String, serde_json::Value>`），条目的实际字段形状见
//! [`crate::schema::admin::driver::DriverInfo`]。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::driver::DriverListResponse;

/// 列出全部驱动配置模板请求构建器。
///
/// 通过 [`Driver::list`](super::Driver::list) 创建。本端点无业务参数；
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/driver/list",
    model = DriverListResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无任何必选或可选业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Driver<'a> {
    /// 列出全部驱动配置模板。
    ///
    /// 对应 AList `GET /api/admin/driver/list`；`data` 为「驱动名 → 驱动模板」映射，
    /// 每个模板含 `common`（通用配置项）、`additional`（驱动专有配置项）与 `config`
    /// （驱动行为开关）三部分。数据来源：`docs/api/alistv3.openapi.yaml` 的
    /// `admin/driver/list`、`examples/alist/server/handles/driver.go`（`ListDriverInfo`）
    /// 与 `examples/alist/internal/op/driver.go`（`GetDriverInfoMap`）。
    ///
    /// 响应为异构复杂结构（键为任意驱动名、条目随驱动种类与 AList 版本演进），
    /// 因此保持宽松模型 [`DriverListResponse`]（`HashMap<String, serde_json::Value>`）；
    /// 需要强类型视图时，可对单个条目执行
    /// `serde_json::from_value::<DriverInfo>`（见
    /// [`DriverInfo`](crate::schema::admin::driver::DriverInfo)）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`DriverListResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如未携带管理员凭据时的 `403`）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::driver::DriverInfo;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let templates = client.admin().driver().list().await?;
    /// println!("已注册驱动数: {}", templates.len());
    /// // 宽松模型：按驱动名取原始 JSON，再按需转为强类型模板
    /// if let Some(local) = templates.get("Local") {
    ///     let info: DriverInfo = serde_json::from_value(local.clone()).unwrap();
    ///     println!("Local 默认根路径: {}", info.config.default_root);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "返回的请求构建器不会自动发送请求"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、正确路径、无查询串、无请求体头。
    #[test]
    fn build_request_composes_get_url_without_query_or_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/driver/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应携带查询参数: {url}");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的 GET 请求不应携带 Content-Type 头"
        );
    }

    /// 收发路径：mock 服务器返回驱动模板映射，验证请求行与响应解码。
    #[tokio::test]
    async fn send_decodes_driver_template_map() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let body = r#"{"code":200,"message":"success","data":{"Local":{"common":[],"additional":[],"config":{"name":"Local","local_sort":false,"only_local":true,"only_proxy":false,"no_cache":false,"no_upload":false,"need_ms":false,"default_root":"","alert":""}}}}"#;
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let templates: DriverListResponse = Request::new(&client).send().await.unwrap();
        assert!(templates.contains_key("Local"));
        assert_eq!(templates["Local"]["config"]["name"], "Local");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/driver/list"),
            "{}",
            recorded[0]
        );
    }
}
