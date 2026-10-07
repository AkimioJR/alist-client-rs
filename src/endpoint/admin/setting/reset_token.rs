//! admin-setting 端点：重置站点令牌。
//!
//! 对应 `POST /api/admin/setting/reset_token`；无请求参数，
//! 响应 `data` 为新生成的令牌字符串，以 `String` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 重置站点令牌请求构建器。
///
/// 通过 [`Setting::reset_token`](super::Setting::reset_token) 创建，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/reset_token", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Setting<'a> {
    /// 重置站点令牌。
    ///
    /// 对应 AList `POST /api/admin/setting/reset_token`；成功时响应 `data` 为
    /// 新生成的令牌字符串。重置后旧令牌立即失效，请及时更新客户端认证凭据。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/reset_token` 与
    /// `examples/alist/server/handles/setting.go`（实现为 `ResetToken`）。
    ///
    /// # Arguments
    ///
    /// 无。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回新令牌 `String`。
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
    /// let new_token = client.admin().setting().reset_token().await?;
    /// println!("新令牌: {new_token}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn reset_token(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径断言（无查询参数、无请求体）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/reset_token"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应有请求体");
    }

    /// 收发路径：mock 服务器记录请求原文并解码新令牌字符串。
    #[tokio::test]
    async fn send_returns_new_token_string() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":"alist-9d"}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let token = Request::new(&client).send().await.unwrap();
        assert_eq!(token, "alist-9d");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/setting/reset_token"),
            "{}",
            recorded[0]
        );
    }
}
