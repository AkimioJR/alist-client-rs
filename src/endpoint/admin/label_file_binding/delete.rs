//! admin-label-file-binding 端点：删除标签绑定。
//!
//! 对应 `POST /api/admin/label_file_binding/delete`；成功时响应 `data` 为 `null`，
//! 以 `()` 作为端点模型。该分组未收录进 openapi 文档；路由见
//! `examples/alist/server/router.go:213`，处理逻辑见
//! `examples/alist/server/handles/label_file_binding.go:83`
//! （实现为 `handles.DelLabelByFileName`，请求体为文件内
//! `DelLabelFileBinDingReq{file_name, label_id}`，其中 `label_id` 是字符串形态的标签 ID）。

use alist_client_derive::EndpointRequest;

/// 删除标签绑定请求构建器。
///
/// 通过 [`LabelFileBinding::delete`](super::LabelFileBinding::delete) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/label_file_binding/delete",
    model = ()
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件名（必选；与标签 ID 共同定位要删除的绑定记录）。
    file_name: String,
    /// 标签 ID（必选；JSON 中为字符串，服务端按 64 位无符号整数解析）。
    label_id: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(
        client: &'a crate::Client,
        file_name: impl Into<String>,
        label_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            file_name: file_name.into(),
            label_id: label_id.into(),
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 删除标签绑定（按文件名 + 标签 ID 定位）。
    ///
    /// 对应 AList `POST /api/admin/label_file_binding/delete`；成功时响应 `data`
    /// 为 `null`。
    /// 数据来源：`examples/alist/server/router.go:213`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:83`
    /// （实现为 `handles.DelLabelByFileName`）。
    ///
    /// # Arguments
    ///
    /// * `file_name` - 目标文件名。
    /// * `label_id` - 要解绑的标签 ID（JSON 中以字符串传输，可传 `&str` 或数字的字符串形式）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`label_id` 不是合法无符号整数时服务端返回 500。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client
    ///     .admin()
    ///     .label_file_binding()
    ///     .delete("movie.mp4", "1")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn delete(&self, file_name: impl Into<String>, label_id: impl Into<String>) -> Request<'a> {
        Request::new(self.client, file_name, label_id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状断言：方法/URL 与 JSON 请求体键名（`label_id` 为字符串形态）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "movie.mp4", "1")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label_file_binding/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains('?'),
            "本端点无查询参数，URL 不应携带查询串: {url}"
        );

        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"file_name\":\"movie.mp4\""), "{body}");
        // Go 侧 DelLabelFileBinDingReq.LabelId 为 string，序列化必须保持字符串形态
        assert!(body.contains("\"label_id\":\"1\""), "{body}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，`data: null` 解码为 `()`。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        Request::new(&client, "movie.mp4", "1")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label_file_binding/delete "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"file_name\":\"movie.mp4\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"label_id\":\"1\""),
            "{}",
            recorded[0]
        );
    }
}
