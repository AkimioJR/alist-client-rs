//! admin-task 端点：未完成任务列表。
//!
//! 对应 `GET /api/admin/task/upload/undone`；响应 `data` 为任务数组
//! （[`crate::schema::admin::task::TaskInfoList`]，非分页包裹）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::task::TaskInfoList;

/// 获取未完成任务列表请求构建器。
///
/// 通过 [`Task::undone`](super::Task::undone) 创建。无业务参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/admin/task/upload/undone",
    model = TaskInfoList
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Task<'a> {
    /// 获取未完成任务列表。
    ///
    /// 对应 AList `GET /api/admin/task/upload/undone`；响应 `data` 为处于
    /// 待处理/运行中/取消中/出错/重试中等未完成状态的上传任务数组
    /// （数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/task/upload/undone`
    /// 与 `examples/alist/server/handles/task.go` 的 `taskRoute` `/undone` 分支，
    /// 实现为 `common.SuccessResp(c, getTaskInfos(...))`，非分页包裹）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回任务列表
    /// [`TaskInfoList`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// for task in client.admin().task().undone().await? {
    ///     println!("{}: {}", task.id, task.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn undone(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URL/方法断言：无参数 GET，路径完整。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.ends_with("/api/admin/task/upload/undone"),
            "URL 应以路径结尾: {url}"
        );
        assert!(
            built.url().query().is_none(),
            "本端点不应携带查询参数: {url}"
        );
    }

    /// 收发路径断言：mock 服务器返回任务数组，解码为 [`TaskInfoList`]。
    #[tokio::test]
    async fn send_decodes_task_array_response() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": [
                {
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
            ]
        }"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let tasks = Request::new(&client).send().await.unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "sdH2LbjyWRk");
        assert_eq!(tasks[0].progress, 50.0);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/task/upload/undone "),
            "{}",
            recorded[0]
        );
    }
}
