//! admin-meta 端点：获取元信息。
//!
//! 对应 `GET /api/admin/meta/get`；必选查询参数 `id`（元信息 ID），
//! 响应 `data` 为单个 [`Meta`] 对象。
//! 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/get` 与
//! `examples/alist/server/handles/meta.go`（`GetMeta` 经 `c.Query("id")` 取参）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::meta::Meta;

/// 获取元信息请求构建器。
///
/// 通过 [`Meta::get`](super::Meta::get) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/meta/get", model = Meta)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 元信息 ID（必选，作为 `id` 查询参数发送）。
    #[query]
    id: u64,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数（`id` 为该端点唯一必选查询参数）。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, id: u64) -> Self {
        Self { client, id }
    }
}

impl<'a> super::Meta<'a> {
    /// 按 ID 获取单个元信息。
    ///
    /// 对应 AList `GET /api/admin/meta/get?id=<id>`；成功时响应 `data` 为
    /// [`Meta`] 对象（ID 不存在时服务端返回 `code` 非 200 的错误响应）。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/get` 与
    /// `examples/alist/server/handles/meta.go`（实现为 `GetMeta`，经 `op.GetMetaById` 查询）。
    ///
    /// # Arguments
    ///
    /// * `id` - 元信息 ID（来自 [`list`](super::Meta::list) 返回的条目 `id` 字段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 [`Meta`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如 ID 不存在）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let meta = client.admin().meta().get(1).await?;
    /// println!("路径 {} 的密码规则: {:?}", meta.path, meta.password);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build_request().build() 断言：方法、路径与 `id` 查询参数。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 1).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        let url = built.url().as_str();
        assert!(url.contains("/api/admin/meta/get"), "URL 应包含路径: {url}");
        assert!(url.contains("id=1"), "URL 应包含 id 查询参数: {url}");
    }

    /// 收发路径断言：`docs/api/alistv3.openapi.yaml` `/api/admin/meta/get`
    /// 的 200 响应示例应能解码为 [`Meta`]。
    #[tokio::test]
    async fn send_decodes_openapi_get_example() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let body = r#"{"code":200,"message":"success","data":{"id":1,"path":"/a","password":"c","p_sub":false,"write":false,"w_sub":false,"hide":"","h_sub":false,"readme":"","r_sub":false}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = crate::Client::new(base_url).unwrap();

        let meta = Request::new(&client, 1).send().await.unwrap();
        assert_eq!(meta.id, 1);
        assert_eq!(meta.path, "/a");
        assert_eq!(meta.password, "c");
        assert!(!meta.p_sub);
    }
}
