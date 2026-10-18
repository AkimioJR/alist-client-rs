//! share 分享管理端点：更新分享。
//!
//! 对应 `POST /api/share/update`；请求体携带目标分享 ID 与可选的修改项，
//! 响应 `data` 反序列化为 [`Share`]。需要登录非访客用户的用户
//! token，且仅能更新本人创建的分享。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::share::Share;

/// 更新分享请求构建器。
///
/// 通过 [`Share::update`](super::Share::update) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/share/update", model = Share)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标分享 ID（必选参数）。
    ///
    /// 仅能更新当前用户本人创建的分享。
    share_id: String,
    /// 新分享 ID（可选参数）。
    ///
    /// 仅允许字母、数字、下划线与连字符，长度 1-32；缺省保持不变。
    new_share_id: Option<String>,
    /// 新分享名称（可选参数）。
    ///
    /// 缺省或空白时保持不变。
    name: Option<String>,
    /// 新分享密码（可选参数）。
    ///
    /// 缺省或空字符串保持现有密码不变；服务端不提供移除密码的操作。
    password: Option<String>,
    /// 过期时间（可选参数）。
    ///
    /// 服务端以可空字符串建模：缺省（`None`）表示不修改；
    /// `Some("")` 表示清除过期时间（永不过期）；`Some(时间文本)` 表示设置，
    /// 服务端接受 RFC3339 文本以及缺省时区的本地时间文本。
    expire_at: Option<String>,
    /// 访问次数上限（可选参数）。
    ///
    /// 缺省（`None`）表示不修改；`Some(0)` 表示取消上限；
    /// `Some(n)`（`n > 0`）表示设置新上限。
    access_limit: Option<i64>,
    /// 是否允许在线预览（可选参数，缺省保持不变）。
    allow_preview: Option<bool>,
    /// 是否允许下载（可选参数，缺省保持不变）。
    allow_download: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, share_id: impl Into<String>) -> Self {
        Self {
            client,
            share_id: share_id.into(),
            new_share_id: None,
            name: None,
            password: None,
            expire_at: None,
            access_limit: None,
            allow_preview: None,
            allow_download: None,
        }
    }
}

impl<'a> super::Share<'a> {
    /// 更新分享。
    ///
    /// 对应 AList `POST /api/share/update`；响应 `data` 为更新后的
    /// [`Share`]。需要登录非访客用户凭据（服务端路由组挂载认证与
    /// 非访客校验中间件，且仅能更新本人创建的分享）。未设置的可选参数
    /// 不会出现在请求体中，服务端保持对应属性不变。数据来源：AList 服务端
    /// 分享管理模块（实现为 `UpdateShare`）与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 目标分享 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Share`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如分享不存在返回 404、新分享 ID 非法或已存在返回 400）时，返回 [`crate::Error`]。
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
    /// let share = client
    ///     .share()
    ///     .update("aBcD1234")
    ///     .password("new-secret")
    ///     .access_limit(5)
    ///     .await?;
    /// assert_eq!(share.share_id, "aBcD1234");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn update(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径、必选 `share_id` 与全部可选字段序列化。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .new_share_id("nEwId5678")
            .name("新名称")
            .password("new-secret")
            .expire_at("2026-12-31T23:59:59+08:00")
            .access_limit(5)
            .allow_preview(false)
            .allow_download(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/share/update"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状并解码更新后的分享实体。
    #[tokio::test]
    async fn send_posts_body_and_decodes_share() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"id":1,"share_id":"aBcD1234","name":"docs","root_path":"/data/docs","is_dir":true,"has_password":true,"burn_after_read":false,"access_limit":5,"access_count":0,"remaining_accesses":5,"allow_preview":false,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":null,"created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-21T10:00:00+08:00","url":"https://alist.example/s/aBcD1234"}}"#;
        let mock = server
            .mock("POST", "/api/share/update")
            .match_body(mockito::Matcher::JsonString(
                r#"{"share_id":"aBcD1234","access_limit":5,"allow_preview":false}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let share = Request::new(&client, "aBcD1234")
            .access_limit(5)
            .allow_preview(false)
            .send()
            .await
            .unwrap();
        assert_eq!(share.share_id, "aBcD1234");
        assert_eq!(share.access_limit, 5);
        assert!(!share.allow_preview);

        mock.assert_async().await;
    }

    /// 收发路径：`expire_at` 传空串表示清除过期时间（服务端可空字符串语义）。
    #[tokio::test]
    async fn send_expire_at_empty_string_clears_expiry() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"id":1,"share_id":"aBcD1234","name":"docs","root_path":"/data/docs","is_dir":true,"has_password":false,"burn_after_read":false,"access_limit":0,"access_count":0,"remaining_accesses":0,"allow_preview":true,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":null,"created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-21T10:00:00+08:00","url":"https://alist.example/s/aBcD1234"}}"#;
        let mock = server
            .mock("POST", "/api/share/update")
            .match_body(mockito::Matcher::JsonString(
                r#"{"share_id":"aBcD1234","expire_at":""}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let _ = Request::new(&client, "aBcD1234")
            .expire_at(String::new())
            .send()
            .await;

        mock.assert_async().await;
    }
}
