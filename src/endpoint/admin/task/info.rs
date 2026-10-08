//! admin-task 端点：查询单个任务。
//!
//! 对应 `POST /api/admin/task/upload/info`；目标任务经查询参数 `tid` 指定，
//! 响应 `data` 为单个任务对象（[`crate::schema::admin::task::TaskInfo`]）。
//! API 路径与参数位置以 AList OpenAPI 规范与
//! AList 服务端 task 模块为准。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::task::TaskInfo;

/// 查询单个上传任务请求构建器。
///
/// 通过 [`Task::info`](super::Task::info) 创建。`tid` 为必选参数（经
/// `Request::new` 传入，进 URL 查询串）；直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/task/upload/info", model = TaskInfo)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标任务 ID（必选参数）。
    ///
    /// 作为 URL 查询参数传递。
    #[query]
    tid: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, tid: impl Into<String>) -> Self {
        Self {
            client,
            tid: tid.into(),
        }
    }
}

impl<'a> super::Task<'a> {
    /// 按 `tid` 查询单个上传任务。
    ///
    /// 对应 AList `POST /api/admin/task/upload/info`；目标任务经**查询参数**
    /// `tid` 指定（数据来源：AList OpenAPI 规范的
    /// `admin/task/upload/info` 与 AList 服务端 task 模块），响应 `data` 为单个任务对象
    /// （老版本文档示例误标为数组，以服务端实际返回为准）。任务不存在时服务端返回 404 语义错误。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`TaskInfo`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如 `tid` 不存在时）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let task = client.admin().task().info("sdH2LbjyWRk").await?;
    /// println!("{}: {}", task.name, task.status);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn info(&self, tid: impl Into<String>) -> Request<'a> {
        Request::new(self.client, tid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URL/方法断言：POST + 必选查询参数 tid。
    #[test]
    fn build_request_composes_method_url_and_tid_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "sdH2LbjyWRk")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/task/upload/info"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("tid=sdH2LbjyWRk"), "tid 应进查询串: {url}");
        assert!(
            built.body().is_none(),
            "本端点不应携带请求体（tid 经查询串传递）"
        );
    }

    /// 收发路径断言：mock 服务器返回单个任务对象，请求带 tid 查询参数且无 JSON 体。
    #[tokio::test]
    async fn send_posts_tid_query_and_decodes_single_task() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": {
                "id": "sdH2LbjyWRk",
                "name": "upload animated_zoom.gif to [/data](/alist)",
                "creator": "admin",
                "creator_role": [2],
                "state": 0,
                "status": "uploading",
                "progress": 50.0,
                "start_time": "2024-01-01T00:00:00Z",
                "end_time": null,
                "total_bytes": 1024,
                "error": ""
            }
        }"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let task = Request::new(&client, "sdH2LbjyWRk").send().await.unwrap();

        assert_eq!(task.id, "sdH2LbjyWRk");
        assert_eq!(task.status, "uploading");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/task/upload/info?tid=sdH2LbjyWRk "),
            "{}",
            recorded[0]
        );
        assert!(
            !recorded[0]
                .to_ascii_lowercase()
                .contains("content-type: application/json"),
            "无请求体字段的请求不应携带 JSON Content-Type: {}",
            recorded[0]
        );
    }
}
