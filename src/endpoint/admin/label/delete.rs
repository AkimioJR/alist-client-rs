//! admin-label 端点：删除标签。
//!
//! 对应 `POST /api/admin/label/delete`（AList OpenAPI 规范未收录该分组，路由以
//! AList 服务端路由定义的 admin `label` 分组为准；实现为
//! `handles.DeleteLabel`）。标签 ID 通过 URL 查询参数 `id` 传递（服务端
//! 读取查询参数 `id`，无请求体）；成功时响应 `data` 为 `null`，以 `()` 解码。

use alist_client_derive::EndpointRequest;

/// 删除标签请求构建器。
///
/// 通过 [`Label::delete`](super::Label::delete) 创建。本端点没有可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/label/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 标签 ID（必选参数）。
    ///
    /// 以 URL 查询参数 `id` 传递。
    #[query]
    id: u64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点的标签 ID 必选。
    ///
    /// 返回类型 [`Request`] 已整体标记 `#[must_use]`，此处不再重复标注。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, id: u64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::Label<'a> {
    /// 删除标签。
    ///
    /// 对应 AList `POST /api/admin/label/delete`；标签 ID 以查询参数 `id` 传递
    /// （`handles.DeleteLabel` 通过查询参数 `id` 读取，非请求体）。
    /// 成功时响应 `data` 为 `null`，以 `()` 作为端点模型。
    /// 数据来源：AList 服务端路由定义的 admin `label` 路由与
    /// AList 服务端 label 模块（实现为 `DeleteLabel`）；
    /// AList OpenAPI 规范未收录该分组。
    ///
    /// # Arguments
    ///
    /// * `id` - 待删除标签的 ID。
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
    /// client.admin().label().delete(1).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn delete(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：build_request().build() 检查 method 与 URL（含查询参数）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=3"), "URL 应包含 id 查询参数: {url}");
        assert!(built.body().is_none(), "删除端点无请求体: {}", built.url());
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文，`data: null` 解码为 `()`。
    #[tokio::test]
    async fn send_deletes_label_by_id() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 3).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label/delete?id=3 "),
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
