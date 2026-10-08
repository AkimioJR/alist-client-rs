//! admin-label-file-binding 端点：按文件名查询绑定标签。
//!
//! 对应 `GET /api/label_file_binding/get`；响应 `data` 为标签数组
//! （文件未绑定任何标签时为 `null`），因此以 `Option<Vec<Label>>` 作为端点模型。
//! 该分组未收录进 openapi 文档；路由见 `examples/alist/server/router.go:262`
//! （挂载在 `auth.Group("/label_file_binding")` 下，非 `/api/admin` 前缀），
//! 处理逻辑见 `examples/alist/server/handles/label_file_binding.go:32`
//! （实现为 `handles.GetLabelByFileName`，返回 `op.GetLabelByFileName` 的 `[]model.Label`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label_file_binding::Label;

/// 按文件名查询绑定标签请求构建器。
///
/// 通过 [`LabelFileBinding::get`](super::LabelFileBinding::get) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/label_file_binding/get",
    model = Option<Vec<Label>>
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件名（必选）。
    ///
    /// 服务端在 `file_name` 为空时返回 400。
    #[query]
    file_name: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, file_name: impl Into<String>) -> Self {
        Self {
            client,
            file_name: file_name.into(),
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 按文件名查询绑定的标签。
    ///
    /// 对应 AList `GET /api/label_file_binding/get`；成功时响应 `data` 为
    /// [`Label`] 数组，
    /// 文件未绑定任何标签时为 `null`（解码为 `None`）。
    /// 数据来源：`examples/alist/server/router.go:262`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:32`
    /// （实现为 `handles.GetLabelByFileName`）。
    ///
    /// # Arguments
    ///
    /// * `file_name` - 目标文件名（服务端要求非空）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回标签数组
    /// （无绑定时为 `None`）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`file_name` 为空时服务端返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// if let Some(labels) = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .get("movie.mp4")
    ///     .await?
    /// {
    ///     for label in labels {
    ///         println!("{} (id={})", label.name, label.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn get(&self, file_name: impl Into<String>) -> Request<'a> {
        Request::new(self.client, file_name)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状断言：GET 方法、URL 路径与 `file_name` 查询参数。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "movie.mp4")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/label_file_binding/get"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("file_name=movie.mp4"),
            "URL 应包含 file_name 查询参数: {url}"
        );
        assert!(
            !url.contains("/api/admin/"),
            "该端点挂在 /api/label_file_binding 下，不在 /api/admin 前缀: {url}"
        );
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码响应内标签数组。
    #[tokio::test]
    async fn send_reads_query_and_decodes_labels() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"id":1,"type":0,"name":"收藏","description":"","bg_color":"","create_time":"2024-06-01T08:00:00Z"}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let labels = Request::new(&client, "movie.mp4").send().await.unwrap();
        let labels = labels.expect("预设响应包含一个标签");
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].id, 1);
        assert_eq!(labels[0].name, "收藏");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/label_file_binding/get?file_name=movie.mp4"),
            "{}",
            recorded[0]
        );
    }

    /// `data: null` 兼容：文件未绑定标签时解码为 `None`。
    #[tokio::test]
    async fn send_decodes_null_data_as_none() {
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let labels = Request::new(&client, "empty.txt").await.unwrap();
        assert_eq!(labels, None);
    }
}
