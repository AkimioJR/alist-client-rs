//! share 分享管理端点：创建分享。
//!
//! 对应 `POST /api/share/create`；请求体携带分享目标路径与可选的分享 ID、
//! 名称、密码、有效期、访问上限与预览/下载开关，响应 `data` 反序列化为
//! [`ShareResponse`]。需要登录非访客用户的用户 token。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::share::ShareResponse;

/// 创建分享请求构建器。
///
/// 通过 [`Share::create`](super::Share::create) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/share/create", model = ShareResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 分享目标路径（必选参数，站点内绝对路径）。
    ///
    /// 服务端会以当前用户身份解析该路径并校验读取权限。
    path: String,
    /// 自定义分享 ID（可选参数）。
    ///
    /// 仅允许字母、数字、下划线与连字符，长度 1-32；缺省时由服务端随机生成。
    share_id: Option<String>,
    /// 分享名称（可选参数）。
    ///
    /// 缺省或空白时取分享对象名。
    name: Option<String>,
    /// 分享密码（可选参数）。
    ///
    /// 缺省或空字符串表示不设置密码。
    password: Option<String>,
    /// 过期时间（可选参数）。
    ///
    /// 服务端接受 RFC3339 文本（如 `2026-12-31T23:59:59+08:00`），
    /// 以及缺省时区的本地时间文本；缺省表示永不过期。与
    /// [`.expire_hours()`](Request::expire_hours) 同时设置时本参数优先。
    expire_at: Option<String>,
    /// 过期小时数（可选参数）。
    ///
    /// 从当前时间起算的小时数；缺省或 `0` 表示永不过期。
    expire_hours: Option<i64>,
    /// 访问次数上限（可选参数）。
    ///
    /// `0` 或缺省表示不限；达到上限后分享自动耗尽（禁用）。
    access_limit: Option<i64>,
    /// 是否阅后即焚（可选参数）。
    ///
    /// 为 `true` 时服务端将访问上限归一为 1；缺省为 `false`。
    burn_after_read: Option<bool>,
    /// 是否允许在线预览（可选参数，缺省 `true`）。
    allow_preview: Option<bool>,
    /// 是否允许下载（可选参数，缺省 `true`）。
    allow_download: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            share_id: None,
            name: None,
            password: None,
            expire_at: None,
            expire_hours: None,
            access_limit: None,
            burn_after_read: None,
            allow_preview: None,
            allow_download: None,
        }
    }
}

impl<'a> super::Share<'a> {
    /// 创建分享。
    ///
    /// 对应 AList `POST /api/share/create`；响应 `data` 为
    /// [`ShareResponse`]（含服务端生成的分享 ID 与完整分享链接 `url`）。
    /// 需要登录非访客用户凭据（服务端路由组挂载认证与非访客校验中间件，
    /// 且校验对目标路径的读取权限）。数据来源：AList 服务端分享管理模块
    /// （实现为 `CreateShare`）与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `path` - 分享目标路径（站点内绝对路径）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`ShareResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如无权限返回 403、自定义分享 ID 非法或已存在返回 400）时，返回 [`crate::Error`]。
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
    ///     .create("/data/docs")
    ///     .password("secret")
    ///     .expire_hours(24)
    ///     .allow_download(true)
    ///     .await?;
    /// println!("分享链接: {}", share.url);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn create(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径、必选 `path` 与全部可选字段序列化。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data/docs")
            .share_id("aBcD1234")
            .name("我的分享")
            .password("secret")
            .expire_at("2026-12-31T23:59:59+08:00")
            .expire_hours(24)
            .access_limit(10)
            .burn_after_read(true)
            .allow_preview(true)
            .allow_download(false)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/share/create"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状并解码分享实体。
    #[tokio::test]
    async fn send_posts_body_and_decodes_share() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"id":1,"share_id":"aBcD1234","name":"docs","root_path":"/data/docs","is_dir":true,"has_password":true,"burn_after_read":false,"access_limit":10,"access_count":0,"remaining_accesses":10,"allow_preview":true,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":"2026-12-31T23:59:59+08:00","created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-20T10:00:00+08:00","url":"https://alist.example/s/aBcD1234"}}"#;
        let mock = server
            .mock("POST", "/api/share/create")
            .match_body(mockito::Matcher::JsonString(
                r#"{"path":"/data/docs","password":"secret","expire_hours":24,"access_limit":10}"#
                    .to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let share = Request::new(&client, "/data/docs")
            .password("secret")
            .expire_hours(24)
            .access_limit(10)
            .send()
            .await
            .unwrap();
        assert_eq!(share.share_id, "aBcD1234");
        assert!(share.has_password);
        assert_eq!(share.access_limit, 10);
        assert_eq!(share.url, "https://alist.example/s/aBcD1234");

        mock.assert_async().await;
    }
}
