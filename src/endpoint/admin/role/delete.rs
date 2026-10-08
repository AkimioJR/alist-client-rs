//! admin-role 端点：删除角色。
//!
//! 对应 `POST /api/admin/role/delete`（AList OpenAPI 规范未收录该分组，路由见
//! AList 服务端路由定义）。处理函数 `handles.DeleteRole`
//! 从查询串读取 `id`（缺失或非数字时返回 400），内置 `admin`/`guest`
//! 角色被服务端以 403 拒绝，成功时响应 `data: null`
//! （见 AList 服务端 role 模块）。

use alist_client_derive::EndpointRequest;

/// 删除角色请求构建器。
///
/// 通过 [`Role::delete`](super::Role::delete) 创建。直接 `.await` 执行请求，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/role/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标角色 ID（必选参数）。
    #[query]
    id: u64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, id: u64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::Role<'a> {
    /// 删除角色。
    ///
    /// 对应 AList `POST /api/admin/role/delete`；成功时响应 `data` 为 `null`。
    /// 内置 `admin`/`guest` 角色不可删除（服务端返回 403 错误响应）。
    /// 数据来源：AList 服务端路由定义（`handles.DeleteRole`）与
    /// AList 服务端 role 模块；该分组不在 AList OpenAPI 规范中，以 AList 服务端实现为准。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标角色 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如角色不存在或删除内置角色被拒绝）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.admin().role().delete(4).await?;
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

    /// 请求形状：POST 方法、路径与 `id` 查询参数；无 JSON 请求体。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 4).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/role/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=4"), "URL 应包含 id 查询参数: {url}");
        assert!(built.body().is_none(), "删除端点不应携带请求体");
    }

    /// 收发路径：`data: null` 解码为 `()`，请求不携带 JSON Content-Type。
    #[tokio::test]
    async fn send_posts_delete_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 4).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/role/delete?id=4 "),
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
