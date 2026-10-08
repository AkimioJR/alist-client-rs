//! admin-task 端点：已完成任务列表。
//!
//! 对应 `GET /api/admin/task/upload/done`；响应 `data` 为任务数组
//! （[`crate::schema::admin::task::TaskInfoList`]，非分页包裹）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::task::TaskInfoList;

/// 获取已完成任务列表请求构建器。
///
/// 通过 [`Task::done`](super::Task::done) 创建。无业务参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/task/upload/done", model = TaskInfoList)]
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
    /// 获取已完成任务列表。
    ///
    /// 对应 AList `GET /api/admin/task/upload/done`；响应 `data` 为处于
    /// 已取消/已失败/已成功状态的上传任务数组（数据来源：
    /// `docs/api/alistv3.openapi.yaml` 的 `admin/task/upload/done` 与
    /// `examples/alist/server/handles/task.go` 的 `taskRoute` `/done` 分支，
    /// 实现为 `common.SuccessResp(c, getTaskInfos(...))`，非分页包裹）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回任务列表
    /// [`TaskInfoList`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
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
    /// for task in client.admin().task().done().await? {
    ///     println!("{}: {}", task.name, task.error);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn done(&self) -> Request<'a> {
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
            url.ends_with("/api/admin/task/upload/done"),
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
        // 任务元素字段取自 examples/alist/server/handles/task.go 的 TaskInfo（Go 为准）
        let body = r#"{
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "1",
                    "name": "upload 1.png to [/s](/test)",
                    "state": 3,
                    "status": "succeeded",
                    "progress": 100,
                    "error": ""
                }
            ]
        }"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let tasks = Request::new(&client).send().await.unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "upload 1.png to [/s](/test)");
        assert_eq!(tasks[0].creator, ""); // 缺失字段走 default

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/task/upload/done "),
            "{}",
            recorded[0]
        );
    }
}
