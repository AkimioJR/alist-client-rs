//! share 公开分享端点：获取分享内对象详情。
//!
//! 对应 `POST /api/public/share/get`；请求体为分享 ID、分享内相对路径与
//! 分享访问令牌，响应 `data` 反序列化为 [`PublicShareDetail`]
//! （对象详情与所在存储驱动标识）。无需用户认证；受密码保护的分享
//! 必须携带有效分享令牌。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::share::PublicShareDetail;

/// 获取分享内对象详情请求构建器。
///
/// 通过 [`Public::get`](super::Public::get) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/public/share/get",
    model = PublicShareDetail
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 分享 ID（必选参数）。
    ///
    /// 对应分享链接 `/s/{share_id}` 中的分享 ID 段。
    share_id: String,
    /// 分享内相对路径（可选参数，以 `/` 起始）。
    ///
    /// 缺省时获取分享根目录/文件本身；文件分享不支持子路径。
    path: Option<String>,
    /// 分享访问令牌（可选参数，请求体 `token` 字段）。
    ///
    /// 受密码保护的分享必传（通过鉴权端点以分享密码换取）。
    /// 注意这是分享令牌，与用户登录 token（`Authorization` 请求头）无关。
    token: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, share_id: impl Into<String>) -> Self {
        Self {
            client,
            share_id: share_id.into(),
            path: None,
            token: None,
        }
    }
}

impl<'a> super::Public<'a> {
    /// 获取分享内对象详情。
    ///
    /// 对应 AList `POST /api/public/share/get`；无需用户认证，
    /// 响应 `data` 为 [`PublicShareDetail`]（对象详情 `item` 与
    /// 所在存储驱动标识 `provider`）。受密码保护的分享必须通过
    /// [`.token()`](Request::token) 携带有效分享访问令牌。
    /// 数据来源：AList 服务端公开分享处理模块（实现为 `GetPublicShare`）
    /// 与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 分享 ID（分享链接 `/s/{share_id}` 中的段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PublicShareDetail`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如缺少或无效分享令牌返回 401、对象不存在返回 404）时，返回 [`crate::Error`]。
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
    /// let detail = client
    ///     .share()
    ///     .public()
    ///     .get("aBcD1234")
    ///     .path("/demo.txt")
    ///     .token("share-token")
    ///     .await?;
    /// println!("存储驱动: {}", detail.provider);
    /// if let Some(url) = detail.item.download_url {
    ///     println!("下载直链: {url}");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径、JSON 请求体字段（可选项缺省时跳过）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .path("/demo.txt")
            .token("share-token")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/public/share/get"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状并解码详情响应。
    #[tokio::test]
    async fn send_posts_body_and_decodes_get() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"item":{"name":"demo.txt","size":2048,"is_dir":false,"modified":"2026-03-20T10:00:00+08:00","created":"2026-03-20T10:00:00+08:00","thumb":"","type":0,"path":"/demo.txt","download_url":"https://alist.example/sd/aBcD1234/demo.txt?auth=t"},"provider":"Local"}}"#;
        let mock = server
            .mock("POST", "/api/public/share/get")
            .match_body(mockito::Matcher::JsonString(
                r#"{"share_id":"aBcD1234","path":"/demo.txt","token":"share-token"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let detail = Request::new(&client, "aBcD1234")
            .path("/demo.txt")
            .token("share-token")
            .send()
            .await
            .unwrap();
        assert_eq!(detail.provider, "Local");
        assert_eq!(detail.item.name, "demo.txt");
        assert_eq!(detail.item.path, "/demo.txt");

        mock.assert_async().await;
    }

    /// 收发路径：可选项缺省时请求体仅含 `share_id`；直接 `.await` 走 IntoFuture。
    #[tokio::test]
    async fn send_with_defaults_via_into_future() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"item":{"name":"root","size":0,"is_dir":true,"modified":"2026-03-20T10:00:00+08:00","created":"2026-03-20T10:00:00+08:00","thumb":"","type":1,"path":"/"},"provider":"unknown"}}"#;
        let mock = server
            .mock("POST", "/api/public/share/get")
            .match_body(mockito::Matcher::JsonString(
                r#"{"share_id":"aBcD1234"}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let detail = Request::new(&client, "aBcD1234").await.unwrap();
        assert!(detail.item.is_dir);
        assert_eq!(detail.provider, "unknown");

        mock.assert_async().await;
    }
}
