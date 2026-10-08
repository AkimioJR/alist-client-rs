//! admin-task 端点：清空已完成任务。
//!
//! 对应 `POST /api/admin/task/upload/clear_done`；响应 `data: null`，
//! 以 `()` 作为端点模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

/// 清空已完成上传任务请求构建器。
///
/// 通过 [`Task::clear_done`](super::Task::clear_done) 创建。无业务参数，
/// 直接 `.await` 执行，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/task/upload/clear_done", model = ())]
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
    /// 清空已完成（已取消/已失败/已成功）的上传任务。
    ///
    /// 对应 AList `POST /api/admin/task/upload/clear_done`；服务端按状态
    /// 条件批量移除任务（数据来源：`docs/api/alistv3.openapi.yaml` 的
    /// `admin/task/upload/clear_done` 与 `examples/alist/server/handles/task.go:176-188`
    /// 的 `manager.RemoveByCondition(...)`），成功时响应 `data` 为 `null`。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
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
    /// client.admin().task().clear_done().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn clear_done(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URL/方法断言：无参数 POST，路径完整。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.ends_with("/api/admin/task/upload/clear_done"),
            "URL 应以路径结尾: {url}"
        );
        assert!(
            built.url().query().is_none(),
            "本端点不应携带查询参数: {url}"
        );
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径断言：`data: null` 以 `()` 解码，请求不带 JSON Content-Type。
    #[tokio::test]
    async fn send_posts_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/task/upload/clear_done "),
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
