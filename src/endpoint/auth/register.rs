//! auth 端点：注册新用户。
//!
//! 对应 `POST /api/auth/register`；请求体为用户名与明文密码（服务端注册时
//! 自行加盐哈希，`examples/alist/server/handles/auth.go:139` 的 `SetPassword`），
//! 成功时响应 `data: null`，以 `()` 作为端点模型。该端点不在 openapi 中，
//! 路径以 `examples/alist/server/router.go:75` 为准；仅当站点开启
//! `allow_register` 设置时可用，否则返回信封 403。

use alist_client_derive::EndpointRequest;

/// 注册请求构建器。
///
/// 通过 [`Auth::register`](super::Auth::register) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/auth/register", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 新用户名（必选）。
    username: String,
    /// 明文密码（必选；服务端注册时自行加盐哈希）。
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
    /// 注册新用户。
    ///
    /// 对应 AList `POST /api/auth/register`（数据来源：
    /// `examples/alist/server/router.go:75` 与 `examples/alist/server/handles/auth.go`
    /// 的 `Register`，实现于 auth.go:124-145；该端点不在 openapi 中）。
    /// 服务端以默认角色创建用户并自行对明文密码加盐哈希；成功时响应 `data`
    /// 为 `null`。站点未开启注册时返回信封 403（`registration is disabled`）。
    ///
    /// # Arguments
    ///
    /// * `username` - 新用户名。
    /// * `password` - 明文密码。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 如注册被禁用返回 403、用户名已存在返回 500）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// client.auth().register("alice", "password").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn register(
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

    /// 请求形状：method/URL/请求体断言。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "alice", "password")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/auth/register"), "URL 应包含路径: {url}");

        let body = built.body().and_then(|body| body.as_bytes()).unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"username\":\"alice\""), "{body}");
        assert!(body.contains("\"password\":\"password\""), "{body}");
    }

    /// 收发路径：mock 服务器返回 `data: null`，以 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        client
            .auth()
            .register("alice", "password")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/auth/register "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"username\":\"alice\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"password\":\"password\""),
            "{}",
            recorded[0]
        );
    }
}
