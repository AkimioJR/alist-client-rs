//! auth 端点：列出当前用户的 SSH 公钥。
//!
//! 对应 `GET /api/me/sshkey/list`；响应 `data` 为
//! `{"content": [公钥数组], "total": 总数}` 分页形态，复用
//! [`crate::schema::common::PageResponse`]，元素类型为
//! [`crate::schema::auth::SshPublicKey`]。
//! 该端点不在 AList OpenAPI 规范中，路径以 AList 服务端路由定义为准。
//!
//! # Version
//!
//! - 自 AList `v3.42.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::auth::SshPublicKey;

/// 列出当前用户 SSH 公钥请求构建器。
///
/// 通过 [`Auth::me_ssh_key_list`](super::Auth::me_ssh_key_list) 创建。
/// 可选参数使用链式 setter，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/me/sshkey/list",
    model = crate::schema::common::PageResponse<SshPublicKey>
)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = SshPublicKey)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（可选参数）。
    ///
    /// 从 1 开始；缺省或非法时服务端按第 1 页处理。
    #[query]
    page: Option<i32>,
    /// 每页条数（可选参数）。
    ///
    /// 缺省时服务端返回当前用户全部公钥。
    #[query]
    per_page: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Auth<'a> {
    /// 列出当前用户的 SSH 公钥列表。
    ///
    /// 对应 AList `GET /api/me/sshkey/list`；成功时响应 `data` 为
    /// `{"content": [公钥数组], "total": 总数}` 分页形态。
    /// 数据来源：AList 服务端路由定义与 AList 服务端 sshkey 处理模块
    /// （实现为 `handles.ListMyPublicKey`）。
    ///
    /// # Arguments
    ///
    /// 本端点无必选参数；可选字段用链式 setter 配置：
    ///
    /// * `page` - 可选：页码，从 1 开始。
    /// * `per_page` - 可选：每页条数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse<SshPublicKey>`](crate::schema::common::PageResponse)。
    /// 启用 `into-stream` feature 时也可调用 [`.into_stream()`](Request::into_stream)
    /// 逐页流式产出公钥。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如游客账号调用返回 401）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.42.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let page = client
    ///     .auth()
    ///     .me_ssh_key_list()
    ///     .page(1)
    ///     .per_page(10)
    ///     .await?;
    /// for key in page.content {
    ///     println!("{} ({})", key.title, key.fingerprint);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn me_ssh_key_list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SSH_KEY_JSON: &str = r#"{"id":1,"title":"laptop","fingerprint":"SHA256:abc","added_time":"2024-06-01T08:00:00Z","last_used_time":"2024-06-02T12:00:00Z"}"#;

    /// 请求形状断言：GET 方法、URL 路径与查询参数（可选 page/per_page）。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .per_page(20)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/me/sshkey/list"), "URL 应包含路径: {url}");
        assert!(url.contains("page=2"), "URL 应包含页码查询参数: {url}");
        assert!(
            url.contains("per_page=20"),
            "URL 应包含每页条数查询参数: {url}"
        );
        assert!(
            !url.contains("uid="),
            "当前用户列表不应携带管理员 uid 参数: {url}"
        );
        assert!(built.body().is_none(), "GET 列表请求不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );

        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("page="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("per_page="), "None 查询参数应被跳过: {url}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码分页响应。
    #[tokio::test]
    async fn send_lists_ssh_public_keys_from_page_response() {
        let mut server = mockito::Server::new_async().await;
        let body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{SSH_KEY_JSON}],"total":1}}}}"#
        );
        let mock = server
            .mock("GET", "/api/me/sshkey/list")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("page".into(), "1".into()),
                mockito::Matcher::UrlEncoded("per_page".into(), "10".into()),
            ]))
            .match_header("authorization", "token-1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;

        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let page = client
            .auth()
            .me_ssh_key_list()
            .page(1)
            .per_page(10)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].id, 1);
        assert_eq!(page.content[0].title, "laptop");
        assert_eq!(page.content[0].fingerprint, "SHA256:abc");

        mock.assert_async().await;
    }

    /// into-stream 翻页断言（列表端点）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use futures::StreamExt;

        let mut server = mockito::Server::new_async().await;
        let page_body = format!(
            r#"{{"code":200,"message":"success","data":{{"content":[{SSH_KEY_JSON}],"total":1}}}}"#
        );
        let mock_page1 = server
            .mock("GET", "/api/me/sshkey/list")
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "1".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(page_body)
            .create_async()
            .await;
        let mock_page2 = server
            .mock("GET", "/api/me/sshkey/list")
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "2".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":{"content":[],"total":1}}"#)
            .create_async()
            .await;

        let client = crate::Client::new(server.url()).unwrap();

        let ids: Vec<u64> = Request::new(&client)
            .into_stream()
            .map(|item| item.unwrap().id)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(ids, vec![1]);

        mock_page1.assert_async().await;
        mock_page2.assert_async().await;
    }
}
