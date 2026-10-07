//! admin-role 端点：获取角色。
//!
//! 对应 `GET /api/admin/role/get`（openapi 未收录该分组，路由见
//! `examples/alist/server/router.go:151`）。处理函数 `handles.GetRole`
//! 从查询串读取 `id`（缺失或非数字时返回 400 信封错误），角色不存在时
//! 返回 500 信封错误（见 `examples/alist/server/handles/role.go:30-43`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::role::Role;

/// 获取角色请求构建器。
///
/// 通过 [`Role::get`](super::Role::get) 创建。直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/role/get", model = Role)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标角色 ID（必选）。
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
    /// 按 ID 获取角色详情。
    ///
    /// 对应 AList `GET /api/admin/role/get`；响应 `data` 为角色对象。
    /// 数据来源：`examples/alist/server/router.go:151`（`handles.GetRole`）与
    /// `examples/alist/server/handles/role.go:30-43`；该分组不在 openapi 中，以 Go 源码为准。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标角色 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Role`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如角色不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let role = client.admin().role().get(2).await?;
    /// println!("{}: {}", role.name, role.description);
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

    /// 请求形状：GET 方法、路径与 `id` 查询参数。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/admin/role/get"), "URL 应包含路径: {url}");
        assert!(url.contains("id=3"), "URL 应包含 id 查询参数: {url}");
    }

    /// 收发路径：mock 服务器返回角色对象，断言请求行与解码结果。
    #[tokio::test]
    async fn send_gets_role_by_id_and_decodes_content() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 示例形状取自 internal/model/role.go 的 JSON 标签（GetRole 直接返回 model.Role）
        let body = r#"{"code":200,"message":"success","data":{"id":3,"name":"editor","description":"可编辑 /data","default":false,"permission_scopes":[{"path":"/data","permission":15}]}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let role = Request::new(&client, 3).send().await.unwrap();
        assert_eq!(role.id, 3);
        assert_eq!(role.name, "editor");
        assert_eq!(role.permission_scopes.len(), 1);
        assert_eq!(role.permission_scopes[0].path, "/data");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/role/get?id=3"),
            "{}",
            recorded[0]
        );
    }
}
