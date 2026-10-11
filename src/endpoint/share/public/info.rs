//! share 公开分享端点：获取分享公开元信息。
//!
//! 对应 `GET /api/public/share/info`；响应 `data` 反序列化为
//! [`PublicShareInfoResponse`]。无需用户认证；受密码保护的分享
//! 可通过查询参数 `auth` 携带分享访问令牌，使响应 `authed` 为 `true`。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::share::PublicShareInfoResponse;

/// 获取分享公开元信息请求构建器。
///
/// 通过 [`Public::info`](super::Public::info) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/public/share/info",
    model = PublicShareInfoResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 分享 ID（必选参数，查询参数 `share_id`）。
    ///
    /// 对应分享链接 `/s/{share_id}` 中的分享 ID 段。
    #[query]
    share_id: String,
    /// 分享访问令牌（可选参数，查询参数 `auth`）。
    ///
    /// 受密码保护的分享在通过鉴权端点换取令牌后传入，响应 `authed` 才可能为
    /// `true`（无密码分享无需令牌）。注意这是分享令牌，与用户登录 token
    /// （`Authorization` 请求头）无关；服务端还接受请求头 `X-Share-Token`
    /// 传递令牌，本端点未使用该通道。
    #[query]
    auth: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, share_id: impl Into<String>) -> Self {
        Self {
            client,
            share_id: share_id.into(),
            auth: None,
        }
    }
}

impl<'a> super::Public<'a> {
    /// 获取分享公开元信息。
    ///
    /// 对应 AList `GET /api/public/share/info`；无需用户认证，
    /// 响应 `data` 为 [`PublicShareInfoResponse`]（是否需要密码、是否已耗尽、
    /// 当前令牌是否通过校验等）。数据来源：AList 服务端公开分享处理模块
    /// （实现为 `GetPublicShareInfo`）与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 分享 ID（分享链接 `/s/{share_id}` 中的段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PublicShareInfoResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如分享不存在返回 404、已耗尽/过期返回 410）时，返回 [`crate::Error`]。
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
    /// let info = client.share().public().info("aBcD1234").await?;
    /// if info.has_password && !info.authed {
    ///     // 需先通过 `client.share().public().auth(...)` 换取分享访问令牌
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn info(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法与路径、`share_id` 查询参数、令牌 setter 写入 `auth`。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .auth("share-token")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/public/share/info"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("share_id=aBcD1234"),
            "URL 应包含分享 ID: {url}"
        );
        assert!(
            url.contains("auth=share-token"),
            "URL 应包含分享令牌参数 auth: {url}"
        );
        assert!(built.body().is_none(), "本端点无请求体");
    }

    /// 请求形状：令牌缺省时不应出现 `auth` 查询参数。
    #[test]
    fn build_request_skips_none_auth_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .build_request()
            .build()
            .unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("auth="), "None 令牌应被跳过: {url}");
        assert!(
            url.contains("share_id=aBcD1234"),
            "URL 应包含分享 ID: {url}"
        );
    }

    /// 收发路径：mock 服务器返回公开元信息并解码。
    #[tokio::test]
    async fn send_gets_share_info_and_decodes() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{"share_id":"aBcD1234","name":"docs","is_dir":true,"has_password":true,"burn_after_read":false,"access_limit":10,"access_count":3,"remaining_accesses":7,"allow_preview":true,"allow_download":true,"authed":true,"consumed_at":null,"expires_at":"2026-12-31T23:59:59+08:00","created_at":"2026-03-20T10:00:00+08:00"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let info = Request::new(&client, "aBcD1234")
            .auth("share-token")
            .send()
            .await
            .unwrap();
        assert_eq!(info.share_id, "aBcD1234");
        assert!(info.has_password);
        assert!(info.authed);
        assert_eq!(info.remaining_accesses, 7);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/public/share/info?share_id=aBcD1234&auth=share-token"),
            "{}",
            recorded[0]
        );
    }
}
