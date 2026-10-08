//! admin-user 端点：获取用户。
//!
//! 对应 `GET /api/admin/user/get`；以必选查询参数 `id` 指定用户，
//! 响应 `data` 为单个用户对象（[`AdminUser`]）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::user::AdminUser;

/// 获取用户请求构建器。
///
/// 通过 [`User::get`](super::User::get) 创建。直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/user/get", model = AdminUser)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标用户 ID（必选参数）。
    ///
    /// 作为 URL 查询参数 `id` 发送。
    #[query]
    id: i64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点没有可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, id: i64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::User<'a> {
    /// 按 ID 获取用户详情。
    ///
    /// 对应 AList `GET /api/admin/user/get`；成功时响应 `data` 为单个用户对象。
    /// 数据来源：AList OpenAPI 规范的 `admin/user/get` 与
    /// AList 服务端 user 模块（实现为 `GetUser`，读取 `id`
    /// 查询参数后按 ID 查库返回）。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标用户 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`AdminUser`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如用户不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let user = client.admin().user().get(1).await?;
    /// println!("{} 的根目录为 {}", user.username, user.base_path);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn get(&self, id: i64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET /api/admin/user/get，查询串携带 id，无请求体。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 1).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/admin/user/get"), "URL 应包含路径: {url}");
        assert!(url.contains("id=1"), "URL 应包含查询参数 id: {url}");
        assert!(built.body().is_none(), "GET 请求不应携带请求体");
    }

    /// 收发路径：mock 服务器返回文档示例，验证用户对象解码与查询串原文。
    #[tokio::test]
    async fn send_decodes_admin_user() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":1,"username":"admin","password":"","base_path":"/","role":2,"disabled":false,"permission":0,"sso_id":""}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let user = Request::new(&client, 1).send().await.unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.username, "admin");
        assert_eq!(user.role, vec![2]); // 文档单值形状展开为数组
        assert!(!user.disabled);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/user/get?id=1"),
            "{}",
            recorded[0]
        );
    }
}
