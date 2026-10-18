//! admin-index 端点：停止进行中的索引构建。
//!
//! 对应 `POST /api/admin/index/stop`；无请求体。服务端向进行中的构建任务
//! 发送停止信号，成功时响应 `data` 为 `null`，以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.6.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 停止索引构建请求构建器。
///
/// 通过 [`Index::stop`](super::Index::stop) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/index/stop", model = ())]
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

impl<'a> super::Index<'a> {
    /// 停止进行中的索引构建。
    ///
    /// 对应 AList `POST /api/admin/index/stop`；服务端向进行中的构建任务
    /// 发送停止信号并立即返回，成功时响应 `data` 为 `null`。
    /// 当前没有构建任务时服务端返回 `index is not running` 错误响应。
    /// 数据来源：AList 服务端路由定义的 admin `index` 分组与
    /// AList 服务端索引处理模块（实现为 `StopIndex`）。
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
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如当前没有构建任务时返回 `index is not running`）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.6.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().index().stop().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn stop(&self) -> Request<'a> {
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
            url.contains("/api/admin/index/stop"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回 `data: null`，以 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/admin/index/stop")
            .match_header("authorization", "admin-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":null}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        client.admin().index().stop().send().await.unwrap();

        mock.assert_async().await;
    }
}
