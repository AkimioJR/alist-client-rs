//! admin-label-file-binding 端点：按标签查询文件。
//!
//! 对应 `GET /api/label_file_binding/get_file_by_label`；响应 `data` 为带标签的
//! 文件条目数组，以 [`Vec<LabeledFsObject>`] 作为端点模型。
//! 该分组未收录进 AList OpenAPI 规范；路由见 AList 服务端路由定义
//! （挂载在用户认证路由组下，非 `/api/admin` 前缀），
//! 处理逻辑见 AList 服务端 label_file_binding 模块
//! （实现为 `handles.GetFileByLabel`）。
//!
//! # Version
//!
//! - 自 AList `v3.46.0` 起提供本接口。
//! - 从 AList `v3.50.0` 起，路由由管理后台路由组的 `/api/admin/label_file_binding/get_file_by_label` 迁移至用户认证路由组的 `/api/label_file_binding/get_file_by_label`，不再要求管理员权限（仅需用户认证）；老版本仅提供 `/api/admin/label_file_binding/get_file_by_label` 路径且仅管理员可访问。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label_file_binding::LabeledFsObject;

/// 按标签查询文件请求构建器。
///
/// 通过 [`LabelFileBinding::get_file_by_label`](super::LabelFileBinding::get_file_by_label)
/// 创建。直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = GET,
    path = "/api/label_file_binding/get_file_by_label",
    model = Vec<LabeledFsObject>
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 标签 ID（必选参数）。
    ///
    /// 支持逗号分隔多个标签，如 `"1,2"`。服务端在 `label_id` 为空时返回 400。
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
    /// 为 [`LabeledFsObject`] 数组。
    /// 数据来源：AList 服务端路由定义（路由注册）与
    /// AList 服务端 label_file_binding 模块
    /// （实现为 `handles.GetFileByLabel`）。
    ///
    /// # Arguments
    ///
    /// * `label_id` - 标签 ID；支持逗号分隔多个标签（如 `"1,2"`），返回命中任一标签的文件。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回带标签列表的文件条目
    /// 数组（[`Vec<LabeledFsObject>`]）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`label_id` 为空时服务端返回 400。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.46.0` 起提供本接口。
    /// - 从 AList `v3.50.0` 起，路由由管理后台路由组的 `/api/admin/label_file_binding/get_file_by_label` 迁移至用户认证路由组的 `/api/label_file_binding/get_file_by_label`，不再要求管理员权限（仅需用户认证）；老版本仅提供 `/api/admin/label_file_binding/get_file_by_label` 路径且仅管理员可访问。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let files = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .get_file_by_label("1,2") // 支持逗号分隔多个标签
    ///     .await?;
    /// for file in files {
    ///     println!("{} ({} 个标签)", file.name, file.label_list.len());
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

    use super::*;

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

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码响应内文件条目数组。
    #[tokio::test]
    async fn send_reads_query_and_decodes_files() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/label_file_binding/get_file_by_label")
            .match_query(mockito::Matcher::UrlEncoded("label_id".into(), "1,2".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":[{"id":"obj-1","path":"/data","name":"movie.mp4","size":1024,"is_dir":false,"modified":"2024-06-01T08:00:00Z","created":"2024-05-01T08:00:00Z","sign":"","thumb":"","type":2,"hashinfo":"","label_list":[{"id":1,"type":0,"name":"收藏","description":"","bg_color":"","create_time":"2024-06-01T08:00:00Z"}]}]}"#,
            )
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let files = Request::new(&client, "1,2").send().await.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "movie.mp4");
        assert_eq!(files[0].id, "obj-1");
        assert_eq!(files[0].label_list.len(), 1);

        mock.assert_async().await;
    }

    /// 空列表断言：无结果时解码为空数组。
    #[tokio::test]
    async fn send_decodes_empty_data() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/api/label_file_binding/get_file_by_label")
            .match_query(mockito::Matcher::UrlEncoded("label_id".into(), "7".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"code":200,"message":"success","data":[]}"#)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let files = Request::new(&client, "7").await.unwrap();
        assert_eq!(files, Vec::<LabeledFsObject>::new());

        mock.assert_async().await;
    }
}
