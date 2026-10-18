//! auth 端点：LDAP 登录获取 token。
//!
//! 对应 `POST /api/auth/login/ldap`；请求体与常规登录一致（用户名与密码），
//! 成功返回 token。该端点不在 AList OpenAPI 规范中，路径与请求体
//! 以 AList 服务端路由定义与 auth 处理模块为准。
//! 服务端未启用 LDAP 登录时返回响应 403（`ldap is not enabled`）。
//!
//! # Version
//!
//! - 自 AList `v3.30.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::LoginResponse;

/// LDAP 登录请求构建器。
///
/// 通过 [`Auth::login_ldap`](super::Auth::login_ldap) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/login/ldap", model = LoginResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// LDAP 用户名（必选参数）。
    username: String,
    /// LDAP 密码（必选参数）。
    ///
    /// 以明文发送，由服务端直接转发给 LDAP 服务器校验，客户端无需预哈希。
    password: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(
        client: &'a crate::Client,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            client,
            username: username.into(),
            password: password.into(),
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 通过 LDAP 登录获取临时 token。
    ///
    /// 对应 AList `POST /api/auth/login/ldap`（数据来源：AList 服务端路由定义与
    /// AList 服务端 LDAP 登录处理模块的 `LoginLdap`；该端点不在 AList OpenAPI 规范中）。
    /// 请求体与常规登录同形（用户名与密码，无两步验证码），成功时返回
    /// [`LoginResponse`]（含 token；从 AList `v3.52.0` 起新增 `device_key`）。
    /// 站点未启用 LDAP 登录时返回响应 403（`ldap is not enabled`）。
    ///
    /// # Arguments
    ///
    /// * `username` - LDAP 用户名。
    /// * `password` - LDAP 密码。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`LoginResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如未启用 LDAP 登录返回 403、密码校验失败返回 400、短时间连续失败触发限流返回 429）
    /// 时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.30.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let login = client.auth().login_ldap("alice", "ldap-password").await?;
    /// println!("token: {}", login.token);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn login_ldap(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, username, password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言；请求体不含两步验证码字段。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "alice", "ldap-password")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/auth/login/ldap"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains("username="),
            "请求体字段不应出现在查询串: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"username\":\"alice\""), "{body}");
        assert!(body.contains("\"password\":\"ldap-password\""), "{body}");
        assert!(!body.contains("otp_code"), "LDAP 登录不含 2FA 字段: {body}");
    }

    /// 收发路径：mock 服务器返回 token，断言请求原文与响应解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_login_response() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/auth/login/ldap")
            .match_body(mockito::Matcher::JsonString(
                r#"{"username":"alice","password":"ldap-password"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"token":"ldap-token","device_key":"key-1"}}"#,
            )
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let login = Request::new(&client, "alice", "ldap-password")
            .send()
            .await
            .unwrap();
        assert_eq!(login.token, "ldap-token");
        assert_eq!(login.device_key.as_deref(), Some("key-1"));

        mock.assert_async().await;
    }

    /// 收发路径：服务端未启用 LDAP 时返回 HTTP 200 + 信封 `code` 403，
    /// 断言客户端将其映射为 API 错误。
    #[tokio::test]
    async fn send_maps_disabled_ldap_to_api_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/auth/login/ldap")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":403,"message":"ldap is not enabled","data":null}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let err = Request::new(&client, "alice", "ldap-password")
            .send()
            .await
            .unwrap_err();
        match err {
            crate::Error::Api { code, message, .. } => {
                assert_eq!(code.as_i32(), 403, "{code:?}");
                assert_eq!(message, "ldap is not enabled");
            }
            other => panic!("expected Api, got {other:?}"),
        }

        mock.assert_async().await;
    }
}
