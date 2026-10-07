//! admin-setting 端点：删除设置。
//!
//! 对应 `POST /api/admin/setting/delete`；`key` 为查询参数，响应 `data` 为 `null`，
//! 以 `()` 作为端点模型。openapi 示例未含标准信封（仅 `{}`），此处以 Go
//! `common.SuccessResp`（`examples/alist/server/common/resp.go`）为准。

use alist_client_derive::EndpointRequest;

/// 删除设置请求构建器。
///
/// 通过 [`Setting::delete`](super::Setting::delete) 创建；`key` 为必选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 要删除的设置键（必选，经查询参数传递）。
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
    /// 删除设置项。
    ///
    /// 对应 AList `POST /api/admin/setting/delete`（`key` 查询参数）；成功时响应
    /// `data` 为 `null`。该端点仅用于删除弃用的设置项。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/delete` 与
    /// `examples/alist/server/handles/setting.go`（实现为 `DeleteSetting`）。
    ///
    /// # Arguments
    ///
    /// * `key` - 要删除的设置键。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
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
    /// client.admin().setting().delete("deprecated_key").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn delete(&self, key: impl Into<String>) -> Request<'a> {
        Request::new(self.client, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 `key` 查询参数断言（无请求体）。
    #[test]
    fn build_request_composes_method_url_and_key_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "deprecated_key")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("key=deprecated_key"),
            "URL 应包含 key 查询参数: {url}"
        );
        assert!(built.body().is_none(), "本端点不应有请求体");
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `data: null` 为 `()`。
    #[tokio::test]
    async fn send_posts_key_query_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "deprecated_key")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/setting/delete?key=deprecated_key"),
            "{}",
            recorded[0]
        );
    }
}
