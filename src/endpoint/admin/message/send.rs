//! admin-message 端点：向服务端投递应答消息。
//!
//! 对应 `POST /api/admin/message/send`；请求体为单字段 `message` 字符串对象，
//! 服务端将其投递给等待应答的内部消费者，成功时响应 `data` 为 `null`，
//! 以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 向服务端投递应答消息请求构建器。
///
/// 通过 [`Message::send`](super::Message::send) 创建；`message` 为必选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/message/send", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 投递给服务端内部消费者的应答消息文本（必选参数）。
    ///
    /// 典型场景为响应 [`get`](super::Message::get) 取出的交互请求
    /// （例如输入验证码）；服务端没有等待应答的消费者时投递失败。
    message: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, message: impl Into<String>) -> Self {
        Self {
            client,
            message: message.into(),
        }
    }
}

impl<'a> super::Message<'a> {
    /// 向服务端投递应答消息。
    ///
    /// 对应 AList `POST /api/admin/message/send`；将消息文本投递给服务端
    /// 正在等待应答的内部消费者（例如请求验证码输入的驱动逻辑），
    /// 成功时响应 `data` 为 `null`。服务端没有等待应答的消费者时返回
    /// `nowhere needed` 错误响应（响应 `code` 为 500）。
    /// 数据来源：AList 服务端路由定义的 admin `message` 分组与
    /// AList 服务端 message 模块（实现为 `SendHandle`）。
    ///
    /// # Arguments
    ///
    /// * `message` - 投递给服务端内部消费者的应答消息文本。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如没有等待应答的消费者时返回 `nowhere needed`）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().message().send("123456").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn send(&self, message: impl Into<String>) -> Request<'a> {
        Request::new(self.client, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"message":...}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "123456")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/message/send"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"message":"123456"}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `data: null`。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/admin/message/send")
            .match_header("authorization", "admin-token")
            .match_body(mockito::Matcher::PartialJsonString(
                r#"{"message":"verification-code"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        client
            .admin()
            .message()
            .send("verification-code")
            .send()
            .await
            .unwrap();

        mock.assert_async().await;
    }
}
