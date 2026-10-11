//! auth 端点：获取当前用户的登录会话列表。
//!
//! 对应 `GET /api/me/sessions`；无需请求体，返回 [`Vec<SessionResp>`]。
//!
//! # Version
//!
//! - 自 AList `v3.52.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::SessionResp;

/// 获取当前用户登录会话列表请求构建器。
///
/// 通过 [`Auth::sessions`](super::Auth::sessions) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/me/sessions", model = Vec<SessionResp>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Auth<'a> {
    /// 获取当前用户的登录会话列表。
    ///
    /// 对应 AList `GET /api/me/sessions`（数据来源：AList 服务端路由定义与
    /// session 处理模块）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Vec<SessionResp>`]。
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
    /// let sessions = client.auth().sessions().await?;
    /// for session in sessions {
    ///     println!("会话 ID: {}, IP: {}", session.session_id, session.ip);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn sessions(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL 断言；GET 请求不应携带请求体。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/me/sessions"), "URL 应包含路径: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回会话列表 JSON，断言 GET 方法与正确解码。
    #[tokio::test]
    async fn send_gets_sessions_and_decodes_session_list() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"session_id":"device-abc","last_active":1710000000,"status":1,"ua":"Mozilla/5.0","ip":"192.168.1.1"}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("test-token".to_owned()));

        let sessions = client.auth().sessions().send().await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "device-abc");
        assert_eq!(sessions[0].user_id, None);
        assert_eq!(sessions[0].last_active, 1710000000);
        assert_eq!(sessions[0].status, 1);
        assert_eq!(sessions[0].ua, "Mozilla/5.0");
        assert_eq!(sessions[0].ip, "192.168.1.1");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/me/sessions "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: test-token"),
            "应注入认证头: {}",
            recorded[0]
        );
    }
}
