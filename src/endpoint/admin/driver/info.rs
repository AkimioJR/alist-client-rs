//! admin-driver 端点：获取单个驱动详情。
//!
//! 对应 `GET /api/admin/driver/info`；查询参数 `driver` 指定目标驱动名，
//! 响应 `data` 为该驱动的配置模板 [`DriverInfo`]。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::driver::DriverInfo;

/// 获取单个驱动详情请求构建器。
///
/// 通过 [`Driver::info`](super::Driver::info) 创建。`driver` 为必选查询参数
/// （只能经 `Request::new` 传入）；直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/driver/info", model = DriverInfo)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标驱动名称（必选参数）。
    ///
    /// 作为查询参数传递，如 `Local`、`115 Cloud`。
    #[query]
    driver: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点的 `driver` 为必选查询参数，
    /// 无可选业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, driver: impl Into<String>) -> Self {
        Self {
            client,
            driver: driver.into(),
        }
    }
}

impl<'a> super::Driver<'a> {
    /// 获取指定驱动的配置模板详情。
    ///
    /// 对应 AList `GET /api/admin/driver/info?driver=<名称>`；`data` 为该驱动的
    /// 配置模板（`common` 通用配置项、`additional` 驱动专有配置项、`config`
    /// 驱动行为开关），形状与 [`crate::schema::admin::driver::DriverListResponse`]
    /// 中同名键的条目一致。数据来源：AList OpenAPI 规范的
    /// `admin/driver/info`、AList 服务端 handles.Driver 模块
    /// （`GetDriverInfo`，按查询参数 `driver` 查找）。
    ///
    /// # Arguments
    ///
    /// * `driver` - 目标驱动名称，如 `Local`、`115 Cloud`。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`DriverInfo`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。驱动不存在时 AList 以 HTTP 200 + 响应 `code: 404`
    /// 返回 `driver [<名称>] not found`（见 AList 服务端 handles.Driver 模块）。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let info = client.admin().driver().info("Local").await?;
    /// for item in &info.additional {
    ///     println!("{} ({}) 默认值: {}", item.name, item.value_type, item.default);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "返回的请求构建器不会自动发送请求"]
    pub fn info(&self, driver: impl Into<String>) -> Request<'a> {
        Request::new(self.client, driver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法、正确路径、必选查询参数 `driver` 的编码。
    #[test]
    fn build_request_encodes_driver_query_param() {
        let client = crate::Client::new("https://alist.example").unwrap();

        let built = Request::new(&client, "Local")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/driver/info"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("driver=Local"),
            "URL 应携带 driver 查询参数: {url}"
        );

        // 含空格的驱动名需按 application/x-www-form-urlencoded 编码
        let built = Request::new(&client, "115 Cloud")
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(
            url.contains("driver=115+Cloud") || url.contains("driver=115%20Cloud"),
            "查询参数应编码空格: {url}"
        );
    }

    /// 收发路径：mock 服务器返回单个驱动模板，验证请求行、查询串与响应解码。
    #[tokio::test]
    async fn send_decodes_single_driver_info() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let body = r#"{"code":200,"message":"success","data":{"common":[{"name":"mount_path","type":"string","default":"","options":"","required":true,"help":""}],"additional":[],"config":{"name":"Local","local_sort":false,"only_local":true,"only_proxy":false,"no_cache":false,"no_upload":false,"need_ms":false,"default_root":"","alert":""}}}"#;
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let info: DriverInfo = Request::new(&client, "Local").send().await.unwrap();
        assert_eq!(info.config.name, "Local");
        assert_eq!(info.common[0].name, "mount_path");
        assert_eq!(info.common[0].value_type, "string");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/driver/info?driver=Local"),
            "{}",
            recorded[0]
        );
    }
}
