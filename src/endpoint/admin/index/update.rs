//! admin-index 端点：按路径增量更新搜索索引。
//!
//! 对应 `POST /api/admin/index/update`；请求体为 `paths`（待更新路径列表）
//! 与 `max_depth`（扫描深度）。服务端在后台异步删除这些路径的旧索引并
//! 重建，成功时响应 `data` 为 `null`，以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.7.1` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 按路径增量更新搜索索引请求构建器。
///
/// 通过 [`Index::update`](super::Index::update) 创建；`paths` 与 `max_depth`
/// 为必选参数，直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/index/update", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待增量更新的路径列表（必选参数）。
    ///
    /// 服务端先删除这些路径下的旧索引条目，再按给定深度重新扫描；
    /// 传入空列表时服务端不做任何扫描（等效于无操作）。
    paths: Vec<String>,
    /// 重新扫描的最大目录深度（必选参数）。
    ///
    /// 服务端直接将其传给索引构建器，不做钳制；全量构建接口
    /// （[`build`](super::Index::build)）使用的服务端默认深度为 20。
    max_depth: i32,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, paths: Vec<String>, max_depth: i32) -> Self {
        Self {
            client,
            paths,
            max_depth,
        }
    }
}

impl<'a> super::Index<'a> {
    /// 按路径增量更新搜索索引。
    ///
    /// 对应 AList `POST /api/admin/index/update`；服务端在后台异步删除
    /// `paths` 中各路径的旧索引条目并按 `max_depth` 深度重建，成功时响应
    /// `data` 为 `null`。该端点要求当前索引实现支持增量更新（服务端
    /// `auto_update` 配置开启），已有构建任务进行中时分别返回
    /// `update is not supported for current index` / `index is running`
    /// 错误响应；进度经 [`progress`](super::Index::progress) 查询。
    /// 数据来源：AList 服务端路由定义的 admin `index` 分组与
    /// AList 服务端索引处理模块（实现为 `UpdateIndex`）。
    ///
    /// # Arguments
    ///
    /// * `paths` - 待增量更新的路径列表。
    /// * `max_depth` - 重新扫描的最大目录深度。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如索引实现不支持增量更新或已有构建任务进行中）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.7.1` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client
    ///     .admin()
    ///     .index()
    ///     .update(vec!["/movies".to_owned(), "/music".to_owned()], 20)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn update(&self, paths: Vec<String>, max_depth: i32) -> Request<'a> {
        Request::new(self.client, paths, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体断言（`{"paths":[...],"max_depth":N}`）。
    #[test]
    fn build_request_composes_method_url_and_json_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, vec!["/movies".to_owned(), "/music".to_owned()], 20)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/index/update"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");

        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        assert_eq!(
            std::str::from_utf8(body).unwrap(),
            r#"{"paths":["/movies","/music"],"max_depth":20}"#
        );
    }

    /// 收发路径：mock 服务器记录请求原文并解码 `data: null`。
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
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        client
            .admin()
            .index()
            .update(vec!["/movies".to_owned()], 5)
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/index/update "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""paths":["/movies"]"#),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""max_depth":5"#), "{}", recorded[0]);
    }
}
