//! admin-setting 端点：设置站点令牌。
//!
//! 对应 `POST /api/admin/setting/set_token`；请求体为 `token` 单字段对象，
//! 响应 `data` 为所设置的令牌字符串，以 `String` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.56.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 设置站点令牌请求构建器。
///
/// 通过 [`Setting::set_token`](super::Setting::set_token) 创建；
/// `token` 为必选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/set_token", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 新的站点访问令牌（必选参数）。
    ///
    /// 服务端将其保存为 `token` 设置项并刷新签名实例；与
    /// [`reset_token`](super::Setting::reset_token) 的随机重置不同，
    /// 本端点允许指定任意令牌值。
    token: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, token: impl Into<String>) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }
}

impl<'a> super::Setting<'a> {
    /// 设置站点访问令牌。
    ///
    /// 对应 AList `POST /api/admin/setting/set_token`；将 `token` 设置项保存为
    /// 指定值并刷新签名实例，成功时响应 `data` 为所设置的令牌字符串。
    /// 保存成功后旧令牌立即失效，请及时更新客户端认证凭据。
    /// 数据来源：AList 服务端 handles.Setting 模块（实现为 `SetToken`，
    /// 请求体 [`SetTokenRequest`](crate::schema::admin::setting::SetTokenRequest)
    /// 仅含 `token` 一个字段，服务端标记为必填）。
    ///
    /// # Arguments
    ///
    /// * `token` - 新的站点访问令牌。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回所设置的令牌 `String`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.56.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let token = client
    ///     .admin()
    ///     .setting()
    ///     .set_token("my-custom-token")
    ///     .await?;
    /// println!("已设置令牌: {token}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_token(&self, token: impl Into<String>) -> Request<'a> {
        Request::new(self.client, token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"token":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "alist-9d")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/set_token"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"token":"alist-9d"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码令牌字符串。
    #[tokio::test]
    async fn send_returns_token_string() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":"alist-9d"}"#;
        let mock = server
            .mock("POST", "/api/admin/setting/set_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let token = Request::new(&client, "alist-9d").send().await.unwrap();
        assert_eq!(token, "alist-9d");

        mock.assert_async().await;
    }
}
