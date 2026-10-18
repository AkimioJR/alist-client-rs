//! auth 端点：注销当前 token。
//!
//! 对应 `GET /api/auth/logout`；无需请求体，服务端吊销 `Authorization` 头携带的
//! 当前 token 并将会话置为 inactive，成功时响应 `data: null`，以 `()` 作为端点模型。
//! 该端点不在 AList OpenAPI 规范中，路径以 AList 服务端路由定义为准。
//!
//! # Version
//!
//! - 自 AList `v3.37.1` 起提供本接口。
//! - 从 AList `v3.52.0` 起，注销时同步将当前设备会话置为 inactive。

use alist_client_derive::EndpointRequest;

/// 注销请求构建器。
///
/// 通过 [`Auth::logout`](super::Auth::logout) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/auth/logout", model = ())]
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
    /// 注销当前 token。
    ///
    /// 对应 AList `GET /api/auth/logout`（数据来源：AList 服务端路由定义与
    /// AList 服务端 auth 处理模块的 `LogOut`；该端点不在 AList OpenAPI 规范中）。
    /// 服务端吊销 `Authorization` 头携带的当前 token 并使其失效；成功时响应
    /// `data` 为 `null`。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
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
    /// - 自 AList `v3.37.1` 起提供本接口。
    /// - 从 AList `v3.52.0` 起，注销时同步将当前设备会话置为 inactive。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.auth().logout().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn logout(&self) -> Request<'a> {
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
        assert!(url.contains("/api/auth/logout"), "URL 应包含路径: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );
    }

    /// 收发路径：mock 服务器返回 `data: null`，断言认证头注入与 `()` 解码。
    #[tokio::test]
    async fn send_gets_logout_and_decodes_null_data() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/auth/logout")
            .match_header("authorization", "token-to-revoke")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-to-revoke".to_owned()));

        client.auth().logout().send().await.unwrap();

        mock.assert_async().await;
    }
}
