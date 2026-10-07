//! admin-label-file-binding 端点：按标签查询文件。
//!
//! 对应 `GET /api/label_file_binding/get_file_by_label`；响应 `data` 为带标签的
//! 文件条目数组（无结果时为 `null`），因此以 `Option<Vec<ObjLabelResp>>` 作为端点模型。
//! 该分组未收录进 openapi 文档；路由见 `examples/alist/server/router.go:263`
//! （挂载在 `auth.Group("/label_file_binding")` 下，非 `/api/admin` 前缀），
//! 处理逻辑见 `examples/alist/server/handles/label_file_binding.go:106`
//! （实现为 `handles.GetFileByLabel`，返回 `op.GetFileByLabel` 的 `[]op.ObjLabelResp`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label_file_binding::ObjLabelResp;

/// 按标签查询文件请求构建器。
///
/// 通过 [`LabelFileBinding::get_file_by_label`](super::LabelFileBinding::get_file_by_label)
/// 创建。直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/label_file_binding/get_file_by_label",
    model = Option<Vec<ObjLabelResp>>
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 标签 ID（必选；支持逗号分隔多个标签，如 `"1,2"`）。
    ///
    /// 服务端在 `label_id` 为空时返回 400。
    #[query]
    label_id: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, label_id: impl Into<String>) -> Self {
        Self {
            client,
            label_id: label_id.into(),
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 按标签查询绑定的文件。
    ///
    /// 对应 AList `GET /api/label_file_binding/get_file_by_label`；成功时响应 `data`
    /// 为 [`ObjLabelResp`](crate::schema::admin::label_file_binding::ObjLabelResp) 数组，
    /// 无结果时为 `null`（解码为 `None`）。
    /// 数据来源：`examples/alist/server/router.go:263`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:106`
    /// （实现为 `handles.GetFileByLabel`）。
    ///
    /// # Arguments
    ///
    /// * `label_id` - 标签 ID；支持逗号分隔多个标签（如 `"1,2"`），返回命中任一标签的文件。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回带标签列表的文件条目
    /// 数组（无结果时为 `None`）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`label_id` 为空时服务端返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// if let Some(files) = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .get_file_by_label("1,2") // 支持逗号分隔多个标签
    ///     .await?
    /// {
    ///     for file in files {
    ///         println!("{} ({} 个标签)", file.name, file.label_list.len());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn get_file_by_label(&self, label_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, label_id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状断言：GET 方法、URL 路径与 `label_id` 查询参数。
    #[test]
    fn build_request_composes_method_url_and_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "42").build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/label_file_binding/get_file_by_label"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("label_id=42"),
            "URL 应包含 label_id 查询参数: {url}"
        );
        assert!(
            !url.contains("/api/admin/"),
            "该端点挂在 /api/label_file_binding 下，不在 /api/admin 前缀: {url}"
        );
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码信封内文件条目数组。
    #[tokio::test]
    async fn send_reads_query_and_decodes_files() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":[{"id":"obj-1","path":"/data","name":"movie.mp4","size":1024,"is_dir":false,"modified":"2024-06-01T08:00:00Z","created":"2024-05-01T08:00:00Z","sign":"","thumb":"","type":2,"hashinfo":"","label_list":[{"id":1,"type":0,"name":"收藏","description":"","bg_color":"","create_time":"2024-06-01T08:00:00Z"}]}]}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let files = Request::new(&client, "1,2").send().await.unwrap();
        let files = files.expect("预设响应包含一个文件条目");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "movie.mp4");
        assert_eq!(files[0].id, "obj-1");
        assert_eq!(files[0].label_list.len(), 1);

        let recorded = requests.lock().unwrap();
        // 逗号分隔的标签 ID 在查询串中按百分号编码传输
        assert!(
            recorded[0].contains("GET /api/label_file_binding/get_file_by_label?label_id=1%2C2"),
            "{}",
            recorded[0]
        );
    }

    /// `data: null` 兼容：无结果时解码为 `None`。
    #[tokio::test]
    async fn send_decodes_null_data_as_none() {
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let files = Request::new(&client, "7").await.unwrap();
        assert_eq!(files, None);
    }
}
