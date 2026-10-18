//! admin-message 端点：取用服务端待推送消息。
//!
//! 对应 `POST /api/admin/message/get`；无请求体，响应 `data` 为
//! [`Message`]
//! （队列为空时服务端返回 `no message` 错误响应）。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::message::Message;

/// 取用服务端待推送消息请求构建器。
///
/// 通过 [`Message::get`](super::Message::get) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/message/get", model = Message)]
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

impl<'a> super::Message<'a> {
    /// 取用服务端待推送消息。
    ///
    /// 对应 AList `POST /api/admin/message/get`；从服务端消息队列取出一条
    /// 待推送消息，成功时响应 `data` 为 [`Message`]（`type` 消息类型、
    /// `content` 任意 JSON 载荷），取出后消息即从队列移除。
    /// 队列为空时服务端返回 `no message` 错误响应（响应 `code` 为 404）。
    /// 数据来源：AList 服务端路由定义的 admin `message` 分组与
    /// AList 服务端 message 模块（实现为 `GetHandle`，
    /// 消息模型对应 AList 服务端 message 模块的 `Message`）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 [`Message`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如队列为空时返回 `no message`）时，返回 [`crate::Error`]。
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
    /// match client.admin().message().get().await {
    ///     Ok(message) => println!("收到消息 {}: {message:?}", message.r#type),
    ///     Err(_) => println!("当前队列无待推送消息"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径断言；本端点不应携带请求体。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/message/get"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回消息 JSON，断言 POST 方法与正确解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_message() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/admin/message/get")
            .match_header("authorization", "admin-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":{"type":"string","content":"dev mode"}}"#,
            )
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        let message = client.admin().message().get().send().await.unwrap();
        assert_eq!(message.r#type, "string");
        assert_eq!(message.content, serde_json::json!("dev mode"));

        mock.assert_async().await;
    }
}
