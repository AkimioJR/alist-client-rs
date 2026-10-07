//! admin-meta 端点：删除元信息。
//!
//! 对应 `POST /api/admin/meta/delete`；必选查询参数 `id`（元信息 ID），
//! 响应 `data: null`，以 `()` 作为端点模型。
//! 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/delete` 与
//! `examples/alist/server/handles/meta.go`（`DeleteMeta` 经 `c.Query("id")` 取参）。

use alist_client_derive::EndpointRequest;

/// 删除元信息请求构建器。
///
/// 通过 [`Meta::delete`](super::Meta::delete) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/meta/delete", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待删除的元信息 ID（必选，作为 `id` 查询参数发送）。
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
    /// 按 ID 删除元信息。
    ///
    /// 对应 AList `POST /api/admin/meta/delete?id=<id>`；成功时响应 `data` 为 `null`
    /// （ID 不存在时服务端返回 `code` 非 200 的信封错误）。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/delete` 与
    /// `examples/alist/server/handles/meta.go`（实现为 `DeleteMeta`，经 `op.DeleteMetaById` 删除）。
    ///
    /// # Arguments
    ///
    /// * `id` - 待删除的元信息 ID（来自 [`list`](super::Meta::list) 返回的条目 `id` 字段）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
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
    /// client.admin().meta().delete(1).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn delete(&self, id: u64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build_request().build() 断言：方法（POST）、路径与 `id` 查询参数。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/meta/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=3"), "URL 应包含 id 查询参数: {url}");
        assert!(
            built.body().is_none(),
            "删除端点无请求体字段，不应携带 body"
        );
    }

    /// 收发路径断言：`docs/api/alistv3.openapi.yaml` `/api/admin/meta/delete`
    /// 的 200 响应示例（`data: null`）应解码为 `()`。
    #[tokio::test]
    async fn send_decodes_null_data() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 3).send().await.unwrap();
    }
}
