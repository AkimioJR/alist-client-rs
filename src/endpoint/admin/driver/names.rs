//! admin-driver 端点：列出驱动名称。
//!
//! 对应 `GET /api/admin/driver/names`；`data` 为已注册驱动的名称数组
//! [`DriverNamesResp`]
//! （`Vec<String>`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::driver::DriverNamesResp;

/// 列出驱动名称请求构建器。
///
/// 通过 [`Driver::names`](super::Driver::names) 创建。本端点无业务参数；
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/driver/names",
    model = DriverNamesResp
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
    /// 列出驱动名称。
    ///
    /// 对应 AList `GET /api/admin/driver/names`；`data` 为已注册驱动的名称数组
    /// （如 `["Local", "115 Cloud", ...]`）。数据来源：`docs/api/alistv3.openapi.yaml`
    /// 的 `admin/driver/names`、`examples/alist/server/handles/driver.go`
    /// （`ListDriverNames`）与 `examples/alist/internal/op/driver.go`
    /// （`GetDriverNames`）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`DriverNamesResp`]（驱动名列表）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如未携带管理员凭据时的 `403`）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let names = client.admin().driver().names().await?;
    /// for name in names {
    ///     println!("驱动: {name}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "返回的请求构建器不会自动发送请求"]
    pub fn names(&self) -> Request<'a> {
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
            url.contains("/api/admin/driver/names"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应携带查询参数: {url}");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的 GET 请求不应携带 Content-Type 头"
        );
    }

    /// 收发路径：mock 服务器返回驱动名数组，验证请求行与响应解码。
    #[tokio::test]
    async fn send_decodes_driver_names() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let body = r#"{"code":200,"message":"success","data":["Local","115 Cloud","UC"]}"#;
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let names: DriverNamesResp = Request::new(&client).send().await.unwrap();
        assert_eq!(
            names,
            vec!["Local".to_owned(), "115 Cloud".to_owned(), "UC".to_owned()]
        );

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/driver/names"),
            "{}",
            recorded[0]
        );
    }
}
