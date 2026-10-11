//! admin-session 端点：查询全部登录会话（管理员）。
//!
//! 对应 `GET /api/admin/session/list`；无需请求体，返回
//! [`Vec<SessionResp>`]
//! （管理员列表总会携带 `user_id`）。
//!
//! # Version
//!
//! - 自 AList `v3.52.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::session::SessionResp;

/// 查询全部登录会话请求构建器。
///
/// 通过 [`Session::list`](super::Session::list) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/session/list", model = Vec<SessionResp>)]
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

impl<'a> super::Session<'a> {
    /// 查询全部登录会话（管理员）。
    ///
    /// 对应 AList `GET /api/admin/session/list`；返回所有用户的登录会话列表，
    /// 成功时响应 `data` 为 [`Vec<SessionResp>`]
    /// （复用 auth 域会话模型，管理员列表总会携带 `user_id` 字段）。
    /// 数据来源：AList 服务端路由定义的 admin `session` 分组与
    /// AList 服务端 session 处理模块（实现为 `ListSessions`）。
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
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200）时，返回 [`crate::Error`]。
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
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// for session in client.admin().session().list().await? {
    ///     println!("用户 {} 的会话 {}: {}", session.user_id.unwrap_or(0), session.session_id, session.ip);
    /// }
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

    /// 请求形状：GET 方法与路径断言；本端点不应携带请求体。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/session/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回会话列表 JSON（含 `user_id`），断言正确解码。
    #[tokio::test]
    async fn send_gets_all_sessions_and_decodes_session_list() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"session_id":"device-admin","user_id":1,"last_active":1710000000,"status":1,"ua":"Mozilla/5.0","ip":"192.168.1.1"},{"session_id":"device-guest","user_id":2,"last_active":1710000100,"status":0,"ua":"curl/8.0","ip":"10.0.0.2"}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        let sessions = client.admin().session().list().send().await.unwrap();
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].session_id, "device-admin");
        assert_eq!(sessions[0].user_id, Some(1));
        assert_eq!(sessions[0].last_active, 1710000000);
        assert_eq!(sessions[1].session_id, "device-guest");
        assert_eq!(sessions[1].user_id, Some(2));
        assert_eq!(sessions[1].ip, "10.0.0.2");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/admin/session/list "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: admin-token"),
            "应注入认证头: {}",
            recorded[0]
        );
    }
}
