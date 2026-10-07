//! auth 端点：获取当前用户信息。
//!
//! 对应 `GET /api/me`；无需请求体，返回 [`MeResp`]
//! （`examples/alist/server/handles/auth.go:147-191` 的 `CurrentUser`）。
//! 注意 Go 侧 `role` 为数组（`model.Roles []int`，user.go:33），而 openapi 示例
//! 与老服务器返回单值 int，schema 层已做兼容；token 缺失时服务端返回游客信息。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::MeResp;

/// 获取当前用户信息请求构建器。
///
/// 通过 [`Auth::me`](super::Auth::me) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/me", model = MeResp)]
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
    /// 对应 AList `GET /api/me`（数据来源：`docs/api/alistv3.openapi.yaml` 的
    /// `/api/me` 与 `examples/alist/server/handles/auth.go` 的 `CurrentUser`，
    /// 实现于 auth.go:156-191）。成功时返回 [`MeResp`]：用户 ID、用户名、
    /// 根目录、角色 ID 列表（`role` 兼容数组/单值/`null` 三种历史形状）、
    /// 聚合权限位掩码、是否启用 2FA，以及新服务器的 `role_names`/`permissions`
    /// 字段（老服务器缺失或为 `null` 时归约为空值）。
    /// 响应中的 `password` 字段恒为空字符串（handler 置空，auth.go:162）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`MeResp`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）
    /// 时，返回 [`crate::Error`]。
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

    /// 收发路径：mock 服务器返回 openapi `/api/me` 示例（老服务器单值 role 形状），
    /// 断言 GET 方法、认证头注入与 schema 兼容解码。
    #[tokio::test]
    async fn send_gets_me_and_decodes_user_resp() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":1,"username":"admin","password":"","base_path":"/","role":2,"disabled":false,"permission":0,"sso_id":"","otp":true}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let me = client.auth().me().send().await.unwrap();
        assert_eq!(me.username, "admin");
        assert_eq!(me.role, vec![2], "单值 role 应展开为数组");
        assert!(me.otp);

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("GET /api/me "), "{}", recorded[0]);
        assert!(
            recorded[0].contains("authorization: token-1"),
            "应注入认证头: {}",
            recorded[0]
        );
    }

    /// 收发路径：新服务器返回数组 role 与 `null` 集合字段时同样可解码。
    #[tokio::test]
    async fn send_decodes_array_role_and_null_collections() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":2,"username":"user","password":"","base_path":"/","role":[2,3],"disabled":false,"permission":65535,"sso_id":"","otp":false,"role_names":null,"permissions":null}}"#,
            )],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let me = client.auth().me().send().await.unwrap();
        assert_eq!(me.role, vec![2, 3]);
        assert!(me.role_names.is_empty());
        assert!(me.permissions.is_empty());
    }
}
