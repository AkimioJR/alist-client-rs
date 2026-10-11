//! admin-index 端点：查询搜索索引构建进度。
//!
//! 对应 `GET /api/admin/index/progress`；无需请求体，响应 `data` 为
//! [`IndexProgress`]。
//!
//! # Version
//!
//! - 自 AList `v3.6.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::index::IndexProgress;

/// 查询搜索索引构建进度请求构建器。
///
/// 通过 [`Index::progress`](super::Index::progress) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/index/progress",
    model = IndexProgress
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Index<'a> {
    /// 查询搜索索引构建进度。
    ///
    /// 对应 AList `GET /api/admin/index/progress`；成功时响应 `data` 为
    /// [`IndexProgress`]（`obj_count` 已索引对象计数、`is_done` 是否完成、
    /// `last_done_time` 上次完成时间、`error` 错误信息）。进度由服务端
    /// 持久化，重启后保留上次构建结果（进行中的构建任务会随重启中断）。
    /// 数据来源：AList 服务端路由定义的 admin `index` 分组与
    /// AList 服务端索引处理模块（实现为 `GetProgress`，响应模型对应
    /// AList 服务端搜索索引数据模型的 `IndexProgress`）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`IndexProgress`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.6.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let progress = client.admin().index().progress().await?;
    /// println!("已索引 {} 个对象，完成: {}", progress.obj_count, progress.is_done);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn progress(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET 方法与路径断言；本端点不应携带请求体与查询参数。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/index/progress"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回进度 JSON，断言 GET 方法与正确解码。
    #[tokio::test]
    async fn send_gets_progress_and_decodes_index_progress() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"obj_count":12345,"is_done":true,"last_done_time":"2024-06-01T08:00:00Z","error":""}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        let progress = client.admin().index().progress().send().await.unwrap();
        assert_eq!(progress.obj_count, 12345);
        assert!(progress.is_done);
        assert!(progress.last_done_time.is_some());
        assert_eq!(progress.error, "");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/admin/index/progress "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: admin-token"),
            "应注入认证头: {}",
            recorded[0]
        );
    }
}
