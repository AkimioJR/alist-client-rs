//! share 公开分享端点：浏览分享内目录列表。
//!
//! 对应 `POST /api/public/share/list`；请求体为分享 ID、分享内相对路径、
//! 分享访问令牌与分页参数（`page`/`per_page` 均为 JSON 请求体字段，
//! 非查询串），响应 `data` 反序列化为 [`PublicShareListResponse`]。
//! 无需用户认证；受密码保护的分享必须携带有效分享令牌。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。
//! - 从 AList `v3.61.0` 起，请求体 `per_page` 支持 `-1` 表示一次返回全部条目
//!   （此时响应 `has_more` 恒为 `false`）；老版本将负值按缺省每页条数处理。

use alist_client_derive::EndpointRequest;

use crate::schema::share::PublicShareListResponse;

/// 浏览分享内目录列表请求构建器。
///
/// 通过 [`Public::list`](super::Public::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型；
/// `into-stream` feature 下还可调用 [`.into_stream()`](Request::into_stream)
/// 得到自动翻页的条目流。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/public/share/list",
    model = PublicShareListResponse
)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(into_stream = true, stream_item = crate::schema::share::PublicShareObjResponse)
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
    /// 缺省时浏览分享根目录；文件分享不支持子路径。
    path: Option<String>,
    /// 分享访问令牌（可选参数，请求体 `token` 字段）。
    ///
    /// 受密码保护的分享必传（通过鉴权端点以分享密码换取）。
    /// 注意这是分享令牌，与用户登录 token（`Authorization` 请求头）无关。
    token: Option<String>,
    /// 页码（可选参数，请求体 `page`）。
    ///
    /// 从 1 开始；缺省或小于 1 时服务端按第 1 页处理。
    page: Option<i32>,
    /// 每页条数（可选参数，请求体 `per_page`）。
    ///
    /// 缺省或 `0` 时服务端使用默认每页条数；从 AList `v3.61.0` 起，
    /// `-1` 表示一次返回全部条目。
    per_page: Option<i32>,
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
            page: None,
            per_page: None,
        }
    }
}

impl<'a> super::Public<'a> {
    /// 浏览分享内目录列表。
    ///
    /// 对应 AList `POST /api/public/share/list`；无需用户认证，
    /// 响应 `data` 为 [`PublicShareListResponse`]（内容、总数、页码、
    /// `has_more` 与总页数）。受密码保护的分享必须通过
    /// [`.token()`](Request::token) 携带有效分享访问令牌。
    /// 数据来源：AList 服务端公开分享处理模块（实现为 `ListPublicShare`）
    /// 与服务端路由定义。
    ///
    /// # Arguments
    ///
    /// * `share_id` - 分享 ID（分享链接 `/s/{share_id}` 中的段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PublicShareListResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如缺少或无效分享令牌返回 401、路径不是目录返回 400）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.59.0` 起提供本接口。
    /// - 从 AList `v3.61.0` 起，请求体 `per_page` 支持 `-1` 表示一次返回全部条目
    ///   （此时响应 `has_more` 恒为 `false`）；老版本将负值按缺省每页条数处理。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let page = client
    ///     .share()
    ///     .public()
    ///     .list("aBcD1234")
    ///     .path("/sub")
    ///     .token("share-token")
    ///     .page(1)
    ///     .per_page(30)
    ///     .await?;
    /// for item in &page.content {
    ///     println!("{}\t{} 字节", item.name, item.size);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self, share_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, share_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径；分页与令牌均为 JSON 请求体字段（非查询串）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "aBcD1234")
            .path("/sub")
            .token("share-token")
            .page(2)
            .per_page(30)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/public/share/list",
            "分页与令牌应在请求体而非查询串"
        );
        assert!(built.body().is_some(), "本端点携带 JSON 请求体");
    }

    /// 收发路径：mock 服务器断言请求体形状并解码列表响应。
    #[tokio::test]
    async fn send_posts_body_and_decodes_list() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{"content":[{"name":"demo.txt","size":1024,"is_dir":false,"modified":"2026-03-20T10:00:00+08:00","created":"2026-03-20T10:00:00+08:00","thumb":"","type":0,"path":"/demo.txt","download_url":"https://alist.example/sd/aBcD1234/demo.txt?auth=t"}],"total":1,"page":1,"per_page":30,"has_more":false,"pages_total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client, "aBcD1234")
            .token("share-token")
            .page(1)
            .per_page(30)
            .send()
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].name, "demo.txt");
        assert_eq!(page.per_page, 30);
        assert!(!page.has_more);
        assert!(page.content[0].download_url.is_some());

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/public/share/list "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""share_id":"aBcD1234""#),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""token":"share-token""#),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""page":1"#), "{}", recorded[0]);
        assert!(recorded[0].contains(r#""per_page":30"#), "{}", recorded[0]);
        assert!(
            !recorded[0].contains(r#""path""#),
            "None 路径应被跳过: {}",
            recorded[0]
        );
    }

    /// into-stream 翻页：翻页字段为请求体 `page`，逐页产出 `content` 元素。
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
                    r#"{"code":200,"message":"success","data":{"content":[{"name":"a.txt","size":1,"is_dir":false,"modified":"2026-03-20T10:00:00+08:00","created":"2026-03-20T10:00:00+08:00","thumb":"","type":0,"path":"/a.txt"}],"total":2,"page":1,"per_page":1,"has_more":true,"pages_total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"name":"b.txt","size":2,"is_dir":false,"modified":"2026-03-20T10:00:00+08:00","created":"2026-03-20T10:00:00+08:00","thumb":"","type":0,"path":"/b.txt"}],"total":2,"page":2,"per_page":1,"has_more":false,"pages_total":2}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[],"total":2,"page":3,"per_page":1,"has_more":false,"pages_total":2}}"#,
                ),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let names: Vec<String> = Request::new(&client, "aBcD1234")
            .token("share-token")
            .per_page(1)
            .into_stream()
            .map(|item| item.unwrap().name)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(names, vec!["a.txt".to_owned(), "b.txt".to_owned()]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 3); // 空页会多一次确认请求
        assert!(recorded[0].contains(r#""page":1"#), "{}", recorded[0]);
        assert!(recorded[1].contains(r#""page":2"#), "{}", recorded[1]);
        assert!(recorded[2].contains(r#""page":3"#), "{}", recorded[2]);
    }
}
