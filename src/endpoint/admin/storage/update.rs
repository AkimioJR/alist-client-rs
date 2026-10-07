//! admin-storage 端点：更新存储。
//!
//! 对应 `POST /api/admin/storage/update`；请求体为 `model.Storage` 形状
//! （`examples/alist/server/handles/storage.go` 的 `UpdateStorage` 直接绑定），
//! `id` 用于定位目标存储且驱动名不可变更；成功时 Go 实现返回 `data: null`，
//! openapi 示例则记载 `{"id": N}`，故以 `Option<StorageCreateResp>` 建模。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::storage::StorageCreateResp;

/// 更新存储请求构建器。
///
/// 通过 [`Storage::update`](super::Storage::update) 创建；`id`、`mount_path`、
/// `driver` 与 `addition` 为必选参数，其余可选参数使用链式 setter。直接 `.await`
/// 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/storage/update", model = Option<StorageCreateResp>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标存储 ID（必选）；服务端按该 ID 更新记录。
    id: u64,
    /// 挂载路径（必选）；重命名挂载路径时服务端会同步迁移内存中的存储实例。
    mount_path: String,
    /// 驱动名称（必选）；必须与原存储一致，否则服务端报 `driver cannot be changed`。
    driver: String,
    /// 驱动特定的附加信息（必选），JSON 字符串，字段由各驱动定义。
    addition: String,
    /// 排序值（可选）。
    order: Option<i32>,
    /// 备注名（可选）。
    remark: Option<String>,
    /// 缓存过期时间，单位秒（可选）。
    cache_expiration: Option<i32>,
    /// 存储状态（可选）；通常留空由服务端维护（如 `work`）。
    status: Option<String>,
    /// 是否禁用（可选）；置 `true` 时服务端更新数据库后不会重新挂载驱动。
    disabled: Option<bool>,
    /// 是否禁止建立索引（可选）；新版本服务端字段。
    disable_index: Option<bool>,
    /// 是否启用签名（可选）；新版本服务端字段。
    enable_sign: Option<bool>,
    /// 对象排序字段（可选），例如 `name`。
    order_by: Option<String>,
    /// 对象排序方向（可选），例如 `asc`。
    order_direction: Option<String>,
    /// 列目录时文件夹的展开时机（可选），例如 `front`。
    extract_folder: Option<String>,
    /// 是否启用 Web 代理（可选）。
    web_proxy: Option<bool>,
    /// WebDAV 策略（可选）：`302_redirect`、`use_proxy_url` 或 `native_proxy`。
    webdav_policy: Option<String>,
    /// 是否代理 Range 请求（可选）；新版本服务端字段。
    proxy_range: Option<bool>,
    /// 下载代理 URL（可选）。
    down_proxy_url: Option<String>,
    /// 下载代理 URL 是否附加签名（可选）；新版本服务端字段。
    down_proxy_sign: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        id: u64,
        mount_path: impl Into<String>,
        driver: impl Into<String>,
        addition: impl Into<String>,
    ) -> Self {
        Self {
            client,
            id,
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
    /// 更新指定存储。
    ///
    /// 对应 AList `POST /api/admin/storage/update`；请求体为完整 `model.Storage`
    /// 形状（服务端整体覆写记录），`driver` 不可变更。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/storage/update` 与
    /// `examples/alist/server/handles/storage.go`（实现为 `UpdateStorage`）。
    /// 注意：成功响应的 `data` 两种服务端行为不同——Go 实现
    /// （`UpdateStorage` 内 `common.SuccessResp(c)`）返回 `null`，openapi 示例
    /// 返回 `{ "id": N }`，故模型为 `Option<StorageCreateResp>`。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标存储 ID。
    /// * `mount_path` - 挂载路径。
    /// * `driver` - 驱动名称（必须与原值一致）。
    /// * `addition` - 驱动特定的附加信息，JSON 字符串。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// `Option<StorageCreateResp>`：`data: null` 时为 [`None`]（当前 Go 实现），
    /// `data: {"id": N}` 时为 [`Some`]（openapi 记载的行为）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如存储不存在或驱动名被变更）时，返回 [`crate::Error`]。
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
    ///     .storage()
    ///     .update(2, "/aa", "Aliyundrive", r#"{"refresh_token":"new"}"#)
    ///     .remark("已更新")
    ///     .cache_expiration(30)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn update(
        &self,
        id: u64,
        mount_path: impl Into<String>,
        driver: impl Into<String>,
        addition: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, id, mount_path, driver, addition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 纯 URL/方法断言：POST + 请求体含必选字段与 `id`；未设置的可选字段不序列化。
    #[test]
    fn build_request_posts_body_with_id() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 7, "/spam", "Local", "{}")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/storage/update"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains("id="),
            "id 属于请求体字段，不应出现在查询串: {url}"
        );

        let body = built
            .body()
            .and_then(|body| body.as_bytes())
            .expect("请求体应为字节缓冲");
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["id"], 7);
        assert_eq!(json["mount_path"], "/spam");
        assert_eq!(json["driver"], "Local");
        assert_eq!(json["addition"], "{}");
        assert!(
            json.get("order").is_none(),
            "未设置的可选字段不应序列化: {json}"
        );
    }

    /// 兼容钉扎（Go 实现）：成功响应 `data: null` → `None`。
    #[tokio::test]
    async fn send_decodes_null_data_as_none() {
        use crate::test_support::{ok_json, spawn_mock_server};

        // 响应形状：examples/alist/server/handles/storage.go UpdateStorage → SuccessResp(c)
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, 7, "/spam", "Local", "{}")
            .send()
            .await
            .unwrap();
        assert_eq!(resp, None);
    }

    /// 兼容钉扎（openapi 示例）：成功响应 `data: {"id": N}` → `Some`。
    #[tokio::test]
    async fn send_decodes_id_data_as_some() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 响应示例：docs/api/alistv3.openapi.yaml /api/admin/storage/update（data: {"id": 7}）
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":7}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, 7, "/spam", "Local", "{}")
            .remark("已更新")
            .send()
            .await
            .unwrap();
        assert_eq!(resp, Some(StorageCreateResp { id: 7 }));

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/storage/update "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"id\":7"), "{}", recorded[0]);
        assert!(
            recorded[0].contains("\"remark\":\"已更新\""),
            "{}",
            recorded[0]
        );
    }
}
