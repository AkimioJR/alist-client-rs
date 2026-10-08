//! admin-storage 端点：创建存储。
//!
//! 对应 `POST /api/admin/storage/create`；请求体为 `model.Storage` 形状
//! （AList 服务端 handles.Storage 模块的 `CreateStorage` 直接绑定），
//! 其中 `addition` 为驱动特定的 JSON 字符串；响应 `data` 为 `{ "id": N }`。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::storage::StorageCreateResponse;
pub use crate::schema::admin::storage::WebdavPolicy;

/// 创建存储请求构建器。
///
/// 通过 [`Storage::create`](super::Storage::create) 创建；`mount_path`、`driver`
/// 与 `addition` 为必选参数，其余可选参数使用链式 setter。直接 `.await` 执行
/// 强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/storage/create", model = StorageCreateResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 挂载路径（必选参数）。
    ///
    /// 服务端全局唯一，例如 `/spam`。
    mount_path: String,
    /// 驱动名称（必选参数）。
    ///
    /// 例如 `Local`，创建后不可变更。
    driver: String,
    /// 驱动特定的附加信息（必选参数）。
    ///
    /// JSON 字符串，字段由各驱动定义，例如 `{"root_folder_path":"/data"}`。
    addition: String,
    /// 排序值（可选参数）。
    order: Option<i32>,
    /// 备注名（可选参数）。
    remark: Option<String>,
    /// 缓存过期时间（可选参数）。
    ///
    /// 单位为秒。
    cache_expiration: Option<i32>,
    /// 存储状态（可选参数）。
    ///
    /// 通常留空由服务端维护（如 `work`）。
    status: Option<String>,
    /// 创建后是否立即禁用（可选参数）。
    ///
    /// 新版本服务端字段。
    disabled: Option<bool>,
    /// 是否禁止建立索引（可选参数）。
    ///
    /// 新版本服务端字段。
    disable_index: Option<bool>,
    /// 是否启用签名（可选参数）。
    ///
    /// 新版本服务端字段。
    enable_sign: Option<bool>,
    /// 对象排序字段（可选参数）。
    ///
    /// 例如 `name`。
    order_by: Option<String>,
    /// 对象排序方向（可选参数）。
    ///
    /// 例如 `asc`。
    order_direction: Option<String>,
    /// 列目录时文件夹的展开时机（可选参数）。
    ///
    /// 例如 `front`。
    extract_folder: Option<String>,
    /// 是否启用 Web 代理（可选参数）。
    web_proxy: Option<bool>,
    /// WebDAV 策略（可选参数）。
    ///
    /// 可选值包括 `302_redirect`、`use_proxy_url` 或 `native_proxy`（见 [`WebdavPolicy`]）。
    webdav_policy: Option<WebdavPolicy>,
    /// 是否代理 Range 请求（可选参数）。
    ///
    /// 新版本服务端字段。
    proxy_range: Option<bool>,
    /// 下载代理 URL（可选参数）。
    down_proxy_url: Option<String>,
    /// 下载代理 URL 是否附加签名（可选参数）。
    ///
    /// 新版本服务端字段。
    down_proxy_sign: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        mount_path: impl Into<String>,
        driver: impl Into<String>,
        addition: impl Into<String>,
    ) -> Self {
        Self {
            client,
            mount_path: mount_path.into(),
            driver: driver.into(),
            addition: addition.into(),
            order: None,
            remark: None,
            cache_expiration: None,
            status: None,
            disabled: None,
            disable_index: None,
            enable_sign: None,
            order_by: None,
            order_direction: None,
            extract_folder: None,
            web_proxy: None,
            webdav_policy: None,
            proxy_range: None,
            down_proxy_url: None,
            down_proxy_sign: None,
        }
    }
}

impl<'a> super::Storage<'a> {
    /// 创建存储。
    ///
    /// 对应 AList `POST /api/admin/storage/create`；服务端保存存储并实例化驱动，
    /// 响应 `data` 为 `{ "id": N }`（`CreateStorage` 的 `gin.H{"id": id}`）。
    /// `addition` 为驱动特定的 JSON 字符串，字段模板可经
    /// `client.admin().driver()` 相关端点查询。
    /// 数据来源：AList OpenAPI 规范的 `/api/admin/storage/create` 与
    /// AList 服务端 handles.Storage 模块（实现为 `CreateStorage`）。
    ///
    /// # Arguments
    ///
    /// * `mount_path` - 挂载路径（服务端唯一，例如 `/spam`）。
    /// * `driver` - 驱动名称（例如 `Local`）。
    /// * `addition` - 驱动特定的附加信息，JSON 字符串。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`StorageCreateResponse`]
    /// （新存储的 ID）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如驱动名不存在或挂载路径重复）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{
    ///     Authentication, Client,
    ///     schema::admin::storage::WebdavPolicy,
    /// };
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let resp = client
    ///     .admin()
    ///     .storage()
    ///     .create("/spam", "Local", r#"{"root_folder_path":"/data"}"#)
    ///     .cache_expiration(30)
    ///     .webdav_policy(WebdavPolicy::NativeProxy)
    ///     .extract_folder("front")
    ///     .await?;
    /// println!("新存储 ID: {}", resp.id);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn create(
        &self,
        mount_path: impl Into<String>,
        driver: impl Into<String>,
        addition: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, mount_path, driver, addition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：POST + JSON Content-Type；未设置的可选字段不出现在请求体。
    #[test]
    fn build_request_posts_required_body_only() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/spam", "Local", r#"{"root_folder_path":"/data"}"#)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/create"),
            "URL 应包含路径: {url}"
        );

        let body = built
            .body()
            .and_then(|body| body.as_bytes())
            .expect("请求体应为字节缓冲");
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["mount_path"], "/spam");
        assert_eq!(json["driver"], "Local");
        assert_eq!(json["addition"], r#"{"root_folder_path":"/data"}"#);
        assert!(
            json.get("order").is_none() && json.get("remark").is_none(),
            "未设置的可选字段不应序列化: {json}"
        );
    }

    /// 可选字段经链式 setter 后进入请求体。
    #[test]
    fn build_request_serializes_optional_fields_via_setters() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/spam", "Local", "{}")
            .order(3)
            .remark("备份")
            .cache_expiration(30)
            .status("work")
            .disabled(false)
            .disable_index(true)
            .enable_sign(true)
            .order_by("name")
            .order_direction("asc")
            .extract_folder("front")
            .web_proxy(false)
            .webdav_policy(WebdavPolicy::NativeProxy)
            .proxy_range(true)
            .down_proxy_url("https://proxy.example.com")
            .down_proxy_sign(true)
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["order"], 3);
        assert_eq!(json["remark"], "备份");
        assert_eq!(json["cache_expiration"], 30);
        assert_eq!(json["status"], "work");
        assert_eq!(json["disabled"], false);
        assert_eq!(json["disable_index"], true);
        assert_eq!(json["enable_sign"], true);
        assert_eq!(json["order_by"], "name");
        assert_eq!(json["order_direction"], "asc");
        assert_eq!(json["extract_folder"], "front");
        assert_eq!(json["web_proxy"], false);
        assert_eq!(json["webdav_policy"], "native_proxy");
        assert_eq!(json["proxy_range"], true);
        assert_eq!(json["down_proxy_url"], "https://proxy.example.com");
        assert_eq!(json["down_proxy_sign"], true);
    }

    /// 收发路径断言：mock 服务器按 Go 实现返回 `data: {"id": N}`。
    #[tokio::test]
    async fn send_creates_storage_and_decodes_id() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 响应示例：AList OpenAPI 规范 /api/admin/storage/create（data: {"id": 7}）
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":7}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "/spam", "Local", "{}")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.id, 7);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/storage/create "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"mount_path\":\"/spam\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"driver\":\"Local\""),
            "{}",
            recorded[0]
        );
    }
}
