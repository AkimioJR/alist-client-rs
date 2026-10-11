//! share 分享管理端点：列出当前用户的分享。
//!
//! 对应 `GET /api/share/list`；分页参数（`page`/`per_page`）经查询串传递
//! （服务端对 GET 请求按表单绑定读取），响应 `data` 为
//! `{ "content": [分享...], "total": 总数 }` 通用分页结构。
//! 需要登录非访客用户的用户 token，仅返回本人创建的分享。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::{common::PageResponse, share::ShareResponse};

/// 列出当前用户分享请求构建器。
///
/// 通过 [`Share::list`](super::Share::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型；
/// `into-stream` feature 下还可调用 [`.into_stream()`](Request::into_stream)
/// 得到自动翻页的条目流。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/share/list", model = PageResponse<ShareResponse>)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = ShareResponse)
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 页码（可选参数，查询参数 `page`）。
    ///
    /// 从 1 开始；缺省或小于 1 时服务端按第 1 页处理。
    #[query]
    page: Option<i32>,
    /// 每页条数（可选参数，查询参数 `per_page`）。
    ///
    /// 缺省或小于 1 时服务端一次返回全部分享。
    #[query]
    per_page: Option<i32>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self {
            client,
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Share<'a> {
    /// 列出当前用户创建的分享（分页）。
    ///
    /// 对应 AList `GET /api/share/list`；响应 `data` 为
    /// `{ "content": [ShareResponse...], "total": 总数 }` 分页结构。
    /// 需要登录非访客用户凭据（服务端路由组挂载认证与非访客校验中间件），
    /// 仅返回当前用户本人创建的分享。数据来源：AList 服务端分享管理模块
    /// （实现为 `ListShares`）与服务端路由定义。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResponse`]`<`[`ShareResponse`]`>`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
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
    /// let page = client.share().list().page(1).per_page(20).await?;
    /// for share in &page.content {
    ///     println!("{}\t{}（启用：{}）", share.share_id, share.name, share.enabled);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法与路径、查询参数齐全。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client)
            .page(2)
            .per_page(10)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/share/list"), "URL 应包含路径: {url}");
        assert!(url.contains("page=2"), "URL 应包含页码: {url}");
        assert!(url.contains("per_page=10"), "URL 应包含每页条数: {url}");
        assert!(built.body().is_none(), "本端点无请求体");
    }

    /// 请求形状：可选查询参数缺省时不应出现在 URL 中。
    #[test]
    fn build_request_skips_none_query_params() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        let url = built.url().as_str();
        assert!(!url.contains("page="), "None 查询参数应被跳过: {url}");
        assert!(!url.contains("per_page="), "None 查询参数应被跳过: {url}");
    }

    /// 收发路径：mock 服务器返回分页分享列表并解码。
    #[tokio::test]
    async fn send_gets_share_page_and_decodes_content() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":1,"share_id":"aBcD1234","name":"docs","root_path":"/data/docs","is_dir":true,"has_password":false,"burn_after_read":false,"access_limit":0,"access_count":0,"remaining_accesses":0,"allow_preview":true,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":null,"created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-20T10:00:00+08:00","url":"https://alist.example/s/aBcD1234"}],"total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client)
            .page(1)
            .per_page(10)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content.len(), 1);
        assert_eq!(page.content[0].share_id, "aBcD1234");
        assert!(page.content[0].enabled);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/share/list?page=1&per_page=10"),
            "{}",
            recorded[0]
        );
    }

    /// into-stream 翻页：逐页产出分享实体，空页停止（含一次确认请求）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use std::sync::{Arc, Mutex};

        use futures::StreamExt;

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":1,"share_id":"idOne000","name":"first","root_path":"/a","is_dir":true,"has_password":false,"burn_after_read":false,"access_limit":0,"access_count":0,"remaining_accesses":0,"allow_preview":true,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":null,"created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-20T10:00:00+08:00","url":"https://alist.example/s/idOne000"}],"total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":2,"share_id":"idTwo0000","name":"second","root_path":"/b","is_dir":false,"has_password":true,"burn_after_read":false,"access_limit":0,"access_count":0,"remaining_accesses":0,"allow_preview":true,"allow_download":true,"enabled":true,"view_count":0,"download_count":0,"last_access_at":null,"consumed_at":null,"expires_at":null,"created_at":"2026-03-20T10:00:00+08:00","updated_at":"2026-03-20T10:00:00+08:00","url":"https://alist.example/s/idTwo0000"}],"total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[],"total":2}}"#,
                ),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let names: Vec<String> = Request::new(&client)
            .per_page(1)
            .into_stream()
            .map(|item| item.unwrap().name)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(names, vec!["first".to_owned(), "second".to_owned()]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 3); // 空页会多一次确认请求
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
        assert!(recorded[2].contains("page=3"), "{}", recorded[2]);
    }
}
