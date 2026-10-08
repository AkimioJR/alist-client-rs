//! admin-user 端点：删除用户。
//!
//! 对应 `POST /api/admin/user/delete`；以必选查询参数 `id` 指定用户，
//! 响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

/// 删除用户请求构建器。
///
/// 通过 [`User::delete`](super::User::delete) 创建。直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/user/delete", model = ())]
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
    /// 删除指定用户。
    ///
    /// 对应 AList `POST /api/admin/user/delete`；成功时响应 `data` 为 `null`。
    /// 数据来源：AList OpenAPI 规范的 `admin/user/delete` 与
    /// AList 服务端 user 模块（实现为 `DeleteUser`，读取 `id`
    /// 查询参数后按 ID 删除）。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标用户 ID。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
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
    /// client.admin().user().delete(3).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn delete(&self, id: i64) -> Request<'a> {
        Request::new(self.client, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST /api/admin/user/delete，查询串携带 id，无请求体。
    #[test]
    fn build_request_composes_method_url_and_id_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 7).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/user/delete"),
            "URL 应包含路径: {url}"
        );
        assert!(url.contains("id=7"), "URL 应包含查询参数 id: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );
    }
}
