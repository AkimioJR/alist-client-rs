//! auth 端点：更新当前用户资料。
//!
//! 对应 `POST /api/me/update`；请求体为用户资料的子集字段（用户名、明文密码、
//! SSO 平台 ID），成功时响应 `data: null`，以 `()` 作为端点模型。
//! 该端点不在 AList OpenAPI 规范中，请求形状以 AList 服务端路由定义与
//! auth 处理模块为准。游客账号调用返回响应 403。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.13.1` 起，请求体新增 `sso_id` 字段；老版本忽略该字段。

use alist_client_derive::EndpointRequest;

/// 更新当前用户资料请求构建器。
///
/// 通过 [`Auth::me_update`](super::Auth::me_update) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
///
/// 注意服务端语义：服务端以请求体中的值覆写 `username` 与 `sso_id`；
/// 其中 `username` 在服务端为必填字段，缺省或为空串时服务端返回 400；
/// `password` 仅在传入非空值时生效。
/// 因此即使只想修改密码，也应先经 [`Auth::me`](super::Auth::me) 读取当前值
/// 并原样回传 `username`（需要时连同 `sso_id`），避免服务端校验失败。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/me/update", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 新用户名（可选参数）。
    ///
    /// 服务端总是以该字段覆写用户名；该字段在服务端为必填，缺省或为空串时
    /// 服务端返回 400。
    username: Option<String>,
    /// 新明文密码（可选参数）。
    ///
    /// 服务端自行加盐哈希；仅当传入非空值时生效，字段缺省或为空字符串时
    /// 保持原密码不变。
    password: Option<String>,
    /// SSO 平台用户 ID（可选参数）。
    ///
    /// 服务端总是以该字段覆写 SSO 绑定；字段缺省时 SSO 绑定被清空。
    /// 从 AList `v3.13.1` 起新增；老版本忽略该字段。
    sso_id: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            username: None,
            password: None,
            sso_id: None,
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 更新当前用户资料。
    ///
    /// 对应 AList `POST /api/me/update`（数据来源：AList 服务端路由定义与
    /// AList 服务端 auth 处理模块的 `UpdateCurrent`；该端点不在 AList OpenAPI 规范中）。
    /// 请求体为用户资料子集（用户名、明文密码、SSO 平台 ID），服务端仅处理
    /// 这三个字段，其余用户字段不受影响；成功时响应 `data` 为 `null`。
    /// 游客账号调用返回响应 403（`Guest user can not update profile`）。
    ///
    /// 覆写语义详见 [`Request`][struct@Request] 文档：
    /// `username` 为服务端必填（缺省或空串返回 400），`sso_id` 缺省时被清空，
    /// `password` 仅在非空时生效。
    ///
    /// # Arguments
    ///
    /// 本端点无必选参数；可选字段用链式 setter 配置：
    ///
    /// * `username` - 可选：新用户名（服务端必填，缺省或为空串时返回 400）。
    /// * `password` - 可选：新明文密码（仅非空值生效）。
    /// * `sso_id` - 可选：SSO 平台用户 ID（缺省时清空 SSO 绑定）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如请求体缺省或空 `username` 返回 400、游客账号调用返回 403、
    /// 服务端写库失败返回 500）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.13.1` 起，请求体新增 `sso_id` 字段；老版本忽略该字段。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 先读取当前资料并原样回传 username，避免服务端校验失败返回 400
    /// let me = client.auth().me().await?;
    /// client
    ///     .auth()
    ///     .me_update()
    ///     .username(me.username)
    ///     .password("new-password")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn me_update(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：method/URL/请求体断言（全字段）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .username("alice")
            .password("new-password")
            .sso_id("sso-123")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/me/update"), "URL 应包含路径: {url}");
        assert!(
            !url.contains("username="),
            "请求体字段不应出现在查询串: {url}"
        );

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"username\":\"alice\""), "{body}");
        assert!(body.contains("\"password\":\"new-password\""), "{body}");
        assert!(body.contains("\"sso_id\":\"sso-123\""), "{body}");
    }

    /// 请求形状：未配置的可选字段不出现在请求体中。
    #[test]
    fn build_request_skips_absent_optional_fields() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .password("new-password")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"password\":\"new-password\""), "{body}");
        assert!(!body.contains("username"), "{body}");
        assert!(!body.contains("sso_id"), "{body}");
    }

    /// 收发路径：mock 服务器返回 `data: null`，断言请求原文与 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/me/update")
            .match_header("authorization", "test-token")
            .match_body(mockito::Matcher::JsonString(
                r#"{"username":"alice","sso_id":"sso-123"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("test-token".to_owned()));

        client
            .auth()
            .me_update()
            .username("alice")
            .sso_id("sso-123")
            .send()
            .await
            .unwrap();

        mock.assert_async().await;
    }
}
