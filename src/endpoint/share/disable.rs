//! share 分享管理端点：禁用分享。
//!
//! 对应 `POST /api/share/disable`；请求体为 `{"share_id"}`，
//! 响应 `data` 为 `null`，以 `()` 作为端点模型。需要登录非访客用户的
//! 用户 token，且仅能禁用本人创建的分享。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 禁用分享请求构建器。
///
/// 通过 [`Share::disable`](super::Share::disable) 创建。
/// 本端点无可选参数，响应 `data` 为 `null`。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/share/disable", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标分享 ID（必选参数）。
    ///
    /// 仅能禁用当前用户本人创建的分享。
    share_id: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, share_id: impl Into<String>) -> Self {
        Self {
            client,
            share_id: share_id.into(),
        }
    }
}

impl<'a> super::Share<'a> {
    /// 禁用分享。
    ///
    /// 对应 AList `POST /api/share/disable`；被禁用的分享立即不可访问
    /// （公开端点对禁用分享返回 404），响应 `data` 为 `null`。
    /// 需要登录非访客用户凭据（服务端路由组挂载认证与非访客校验中间件，
    /// 且仅能禁用本人创建的分享）。数据来源：AList 服务端分享管理模块
    /// （实现为 `DisableShare`）与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 目标分享 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如分享不存在返回 404）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.59.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.share().disable("aBcD1234").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn disable(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径、JSON 请求体仅含 `share_id`。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/share/disable"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状与 `data: null` 解码为 `()`。
    #[tokio::test]
    async fn send_posts_share_id_and_decodes_unit() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/share/disable")
            .match_body(mockito::Matcher::JsonString(
                r#"{"share_id":"aBcD1234"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        Request::new(&client, "aBcD1234").await.unwrap();

        mock.assert_async().await;
    }
}
