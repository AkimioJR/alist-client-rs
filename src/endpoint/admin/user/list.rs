//! admin-user 端点：列出用户。
//!
//! 对应 `GET /api/admin/user/list`；响应 `data` 为
//! `{"content": [用户数组], "total": 总数}` 分页形态，复用
//! [`crate::schema::common::PageResp`]，元素类型为
//! [`crate::schema::admin::user::AdminUser`]。
//!
//! 说明：当前服务端实现（`examples/alist/server/handles/user.go` 的 `ListUsers`）
//! 会绑定可选的 `page`/`per_page` 查询参数（`model.PageReq`），但缺省
//! （不传参数，`per_page` 回退为 `MaxInt`）即返回全部用户，且 openapi 文档
//! 未列出这两个参数；因此本端点按文档语义不暴露分页参数。

use alist_client_derive::EndpointRequest;

use crate::schema::{admin::user::AdminUser, common::PageResp};

/// 列出用户请求构建器。
///
/// 通过 [`User::list`](super::User::list) 创建。直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/user/list", model = PageResp<AdminUser>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点没有必选或可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::User<'a> {
    /// 列出全部用户。
    ///
    /// 对应 AList `GET /api/admin/user/list`；成功时响应 `data` 为
    /// `{"content": [用户数组], "total": 总数}` 分页形态。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/user/list`、
    /// `docs/api/alistv3.md` 的 `# admin/user` 分组与
    /// `examples/alist/server/handles/user.go`（实现为 `ListUsers`）。
    ///
    /// 当前服务端实现另接受可选的 `page`/`per_page` 查询参数，但缺省即返回
    /// 全部用户（`internal/model/req.go` 的 `PageReq::Validate` 将 `per_page`
    /// 回退为 `MaxInt`），且 openapi 文档未列出这两个参数，故本客户端不提供。
    ///
    /// # Arguments
    ///
    /// 本端点无参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PageResp`](crate::schema::common::PageResp)`<`[`AdminUser`](crate::schema::admin::user::AdminUser)`>`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
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
    /// let page = client.admin().user().list().await?;
    /// for user in page.content {
    ///     println!("{} (id={})", user.username, user.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn list(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：GET /api/admin/user/list，无查询串、无请求体、无 JSON Content-Type。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/user/list"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应携带查询参数: {url}");
        assert!(built.body().is_none(), "GET 列表请求不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );
    }

    /// 收发路径：mock 服务器返回文档示例，验证分页形态与 `role` 单值展开的解码。
    #[tokio::test]
    async fn send_decodes_page_resp_of_admin_user() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"content":[{"id":1,"username":"admin","password":"","base_path":"/","role":2,"disabled":false,"permission":0,"sso_id":""},{"id":2,"username":"guest","password":"","base_path":"/","role":1,"disabled":true,"permission":0,"sso_id":""}],"total":2}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let page = Request::new(&client).send().await.unwrap();

        assert_eq!(page.total, 2);
        assert_eq!(page.content.len(), 2);
        assert_eq!(page.content[0].username, "admin");
        assert_eq!(page.content[0].role, vec![2]);
        assert!(page.content[1].disabled);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/admin/user/list"),
            "{}",
            recorded[0]
        );
    }
}
