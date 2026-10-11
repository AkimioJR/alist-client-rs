//! share 公开分享端点：分享密码换取访问令牌。
//!
//! 对应 `POST /api/public/share/auth`；请求体为 `{"share_id", "password"}`，
//! 响应 `data` 反序列化为 [`PublicShareAuthResponse`]（`token` 字段）。
//! 无需用户认证；令牌有效期由服务端决定（缺省 24 小时，且不超过分享剩余有效时间）。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::share::PublicShareAuthResponse;

/// 分享密码鉴权请求构建器。
///
/// 通过 [`Public::auth`](super::Public::auth) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/public/share/auth",
    model = PublicShareAuthResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 分享 ID（必选参数）。
    ///
    /// 对应分享链接 `/s/{share_id}` 中的分享 ID 段。
    share_id: String,
    /// 分享密码（可选参数）。
    ///
    /// 无密码分享可不传；密码错误时服务端返回 403。
    password: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, share_id: impl Into<String>) -> Self {
        Self {
            client,
            share_id: share_id.into(),
            password: None,
        }
    }
}

impl<'a> super::Public<'a> {
    /// 分享密码换取访问令牌。
    ///
    /// 对应 AList `POST /api/public/share/auth`；无需用户认证，
    /// 密码正确时响应 `data.token` 为分享访问令牌（无密码分享为空字符串）。
    /// 该令牌与用户登录 token 无关，仅用于访问受密码保护的分享
    /// （浏览/详情端点通过请求体 `token` 字段传递，元信息端点通过查询参数
    /// `auth` 传递）。数据来源：AList 服务端公开分享处理模块
    /// （实现为 `AuthPublicShare`）与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 分享 ID（分享链接 `/s/{share_id}` 中的段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PublicShareAuthResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如密码错误返回 403、分享已耗尽/过期返回 410）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.59.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let token = client
    ///     .share()
    ///     .public()
    ///     .auth("aBcD1234")
    ///     .password("secret")
    ///     .await?
    ///     .token;
    /// let page = client
    ///     .share()
    ///     .public()
    ///     .list("aBcD1234")
    ///     .token(token)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn auth(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径、JSON 请求体字段（密码缺省时跳过）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .password("secret")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/public/share/auth"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状并返回令牌。
    #[tokio::test]
    async fn send_posts_password_and_decodes_token() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"token":"share-token-1"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "aBcD1234")
            .password("secret")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.token, "share-token-1");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/public/share/auth "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""share_id":"aBcD1234""#),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""password":"secret""#),
            "{}",
            recorded[0]
        );
    }

    /// 收发路径：无密码分享不发送 `password` 字段，响应 `token` 为空串。
    #[tokio::test]
    async fn send_skips_none_password() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"token":""}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "aBcD1234").await.unwrap();
        assert_eq!(resp.token, "");

        let recorded = requests.lock().unwrap();
        assert!(
            !recorded[0].contains("password"),
            "None 密码应被跳过: {}",
            recorded[0]
        );
    }
}
