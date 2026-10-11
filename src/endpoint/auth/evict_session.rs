//! auth 端点：踢出指定登录会话。
//!
//! 对应 `POST /api/me/sessions/evict`；请求体为会话标识 `session_id`，成功时响应 `data: null`，
//! 以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.52.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 踢出指定登录会话请求构建器。
///
/// 通过 [`Auth::evict_session`](super::Auth::evict_session) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/me/sessions/evict", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 会话设备标识键（必选参数）。
    ///
    /// 对应会话信息中的 `session_id`。
    session_id: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, session_id: impl Into<String>) -> Self {
        Self {
            client,
            session_id: session_id.into(),
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 踢出指定登录会话。
    ///
    /// 对应 AList `POST /api/me/sessions/evict`（数据来源：AList 服务端路由定义与
    /// session 处理模块）。成功时响应 `data` 为 `null`。
    ///
    /// # Arguments
    ///
    /// * `session_id` - 会话设备标识键。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.52.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.auth().evict_session("sess-device-key").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn evict_session(&self, session_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, session_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "device-key-123")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/me/sessions/evict"),
            "URL 应包含路径: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"session_id\":\"device-key-123\""), "{body}");
    }

    /// 收发路径：mock 服务器返回 `data: null`，以 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("test-token".to_owned()));

        client
            .auth()
            .evict_session("device-key-123")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/me/sessions/evict "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: test-token"),
            "应注入认证头: {}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"session_id\":\"device-key-123\""),
            "{}",
            recorded[0]
        );
    }
}
