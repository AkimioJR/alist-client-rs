//! auth 端点：获取当前用户信息。
//!
//! 对应 `GET /api/me`；无需请求体，返回 [`CurrentUser`]（类型别名，同 [`User`](crate::schema::auth::User)）
//! （AList 服务端 auth 模块的 `CurrentUser`）。
//! token 缺失时服务端返回游客信息。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.10.1` 起，响应新增 `disabled` 字段；老版本不返回该字段。
//! - 从 AList `v3.13.1` 起，响应新增 `sso_id` 字段；老版本不返回该字段。
//! - 从 AList `v3.46.0` 起，`role` 由单值 int 变为角色 ID 数组，
//!   并新增 `role_names` 与 `permissions` 字段；老版本 `role` 为单值 int
//!   且不返回这两个字段，schema 层已对历史形状做兼容。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::CurrentUser;

/// 获取当前用户信息请求构建器。
///
/// 通过 [`Auth::me`](super::Auth::me) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/me", model = CurrentUser)]
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
    /// 获取当前用户信息。
    ///
    /// 对应 AList `GET /api/me`（数据来源：AList OpenAPI 规范的
    /// `/api/me` 与 AList 服务端 auth 模块的 `CurrentUser`）。成功时返回 [`CurrentUser`]（类型别名，同 [`User`](crate::schema::auth::User)）：用户 ID、用户名、
    /// 根目录、角色 ID 列表、聚合权限位掩码、是否启用 2FA，以及
    /// `role_names`/`permissions` 字段。
    /// 响应中的 `password` 字段恒为空字符串（服务端 handler 置空）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`CurrentUser`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.10.1` 起，响应新增 `disabled` 字段；老版本不返回该字段。
    /// - 从 AList `v3.13.1` 起，响应新增 `sso_id` 字段；老版本不返回该字段。
    /// - 从 AList `v3.46.0` 起，`role` 由单值 int 变为角色 ID 数组，
    ///   并新增 `role_names` 与 `permissions` 字段；老版本 `role` 为单值 int
    ///   且不返回这两个字段，schema 层已对历史形状做兼容。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let me = client.auth().me().await?;
    /// println!("当前用户: {}（角色 {:?}）", me.username, me.role);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn me(&self) -> Request<'a> {
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
        assert!(url.contains("/api/me"), "URL 应包含路径: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回规范 `/api/me` 示例（老服务器单值 role 形状），
    /// 断言 GET 方法、认证头注入与 schema 兼容解码。
    #[tokio::test]
    async fn send_gets_me_and_decodes_current_user() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/me")
            .match_header("authorization", "token-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"id":1,"username":"admin","password":"","base_path":"/","role":2,"disabled":false,"permission":0,"sso_id":"","otp":true}}"#,
            )
            .create_async()
            .await;

        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let me = client.auth().me().send().await.unwrap();
        assert_eq!(me.username, "admin");
        assert_eq!(me.role, vec![2], "单值 role 应展开为数组");
        assert!(me.otp);

        mock.assert_async().await;
    }

    /// 收发路径：新服务器返回数组 role 与 `null` 集合字段时同样可解码。
    #[tokio::test]
    async fn send_decodes_array_role_and_null_collections() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/me")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"id":2,"username":"user","password":"","base_path":"/","role":[2,3],"disabled":false,"permission":65535,"sso_id":"","otp":false,"role_names":null,"permissions":null}}"#,
            )
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let me = client.auth().me().send().await.unwrap();
        assert_eq!(me.role, vec![2, 3]);
        assert!(me.role_names.is_empty());
        assert!(me.permissions.is_empty());

        mock.assert_async().await;
    }
}
