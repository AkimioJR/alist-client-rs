//! admin-task 端点：取消任务。
//!
//! 对应 `POST /api/admin/task/upload/cancel`；目标任务经查询参数 `tid` 指定，
//! 响应 `data: null`，以 `()` 作为端点模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径与参数位置以 `docs/api/alistv3.openapi.yaml` 与
//! `examples/alist/server/handles/task.go` 为准。

use alist_client_derive::EndpointRequest;

/// 取消上传任务请求构建器。
///
/// 通过 [`Task::cancel`](super::Task::cancel) 创建。`tid` 为必选参数（经
/// `Request::new` 传入，进 URL 查询串）；直接 `.await` 执行，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/task/upload/cancel", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标任务 ID（必选；进 URL 查询串）。
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
    /// 取消指定上传任务。
    ///
    /// 对应 AList `POST /api/admin/task/upload/cancel`；目标任务经**查询参数**
    /// `tid` 指定（数据来源：`docs/api/alistv3.openapi.yaml` 的
    /// `admin/task/upload/cancel` 与 `examples/alist/server/handles/task.go:86`
    /// 的 `manager.GetByID(c.Query("tid"))`），成功时响应 `data` 为 `null`
    /// （`task.go:155-158` 的 `manager.Cancel(...)` + `common.SuccessResp(c)`）。
    /// 任务不存在时服务端返回 404 语义错误。
    ///
    /// # Arguments
    ///
    /// * `tid` - 目标任务 ID，可传 `&str` 或任何 `Into<String>` 的值。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
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
    /// client.admin().task().cancel("sdH2LbjyWRk").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn cancel(&self, tid: impl Into<String>) -> Request<'a> {
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
        let built = Request::new(&client, "abc123")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/task/upload/cancel"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("tid=abc123"), "tid 应进查询串: {url}");
        assert!(
            built.body().is_none(),
            "本端点不应携带请求体（tid 经查询串传递）"
        );
    }

    /// 收发路径断言：`data: null` 以 `()` 解码，请求带 tid 查询参数且无 JSON 体。
    #[tokio::test]
    async fn send_posts_tid_query_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "abc123").send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/task/upload/cancel?tid=abc123 "),
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
