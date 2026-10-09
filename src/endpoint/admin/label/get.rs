//! admin-label 端点：获取标签。
//!
//! 对应 `GET /api/label/get`（AList OpenAPI 规范未收录该分组，路由以
//! AList 服务端路由定义的 `_label` 为准；实现为 `handles.GetLabel`）。
//! 标签 ID 通过 URL 查询参数 `id` 传递（服务端读取查询参数 `id`），响应 `data` 为标签条目。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label::Label;

/// 获取标签请求构建器。
///
/// 通过 [`Label::get`](super::Label::get) 创建。本端点没有可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/label/get", model = Label)]
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
    /// 按 ID 获取单个标签。
    ///
    /// 对应 AList `GET /api/label/get`；标签 ID 以查询参数 `id` 传递
    /// （`handles.GetLabel` 通过查询参数 `id` 读取，非请求体）。
    /// 成功时响应 `data` 为标签条目，解码为 [`Label`]。
    /// 数据来源：AList 服务端路由定义的 `_label` 路由与
    /// AList 服务端 label 模块（实现为 `GetLabel`）；
    /// AList OpenAPI 规范未收录该分组。
    ///
    /// # Arguments
    ///
    /// * `id` - 标签 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Label`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如标签不存在时服务端返回 `500` 错误响应）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let label = client.admin().label().get(1).await?;
    /// println!("{}: {:?}", label.name, label.bg_color);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn get(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 标签条目示例 JSON（按 AList 服务端标签模型字段构造）。
    const LABEL_JSON: &str = r##"{"id":1,"type":2,"name":"电影","description":"电影相关文件","bg_color":"#FF0000","create_time":"2024-06-01T12:00:00Z"}"##;

    /// 1) 纯 URL/方法断言：build_request().build() 检查 method 与 URL（含查询参数）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 5).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/label/get"), "URL 应包含路径: {url}");
        assert!(url.contains("id=5"), "URL 应包含 id 查询参数: {url}");
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文，响应解码为 Label。
    #[tokio::test]
    async fn send_gets_label_by_id() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = format!(r#"{{"code":200,"message":"success","data":{LABEL_JSON}}}"#);
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let label = Request::new(&client, 1).send().await.unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.label_type, 2);
        assert_eq!(label.name, "电影");
        assert_eq!(label.bg_color.as_deref(), Some("#FF0000"));

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("GET /api/label/get?id=1 "),
            "{}",
            recorded[0]
        );
    }
}
