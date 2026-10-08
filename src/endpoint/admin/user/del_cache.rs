//! admin-user 端点：清理用户缓存。
//!
//! 对应 `POST /api/admin/user/del_cache`；以必选查询参数 **`username`**（用户名，
//! 而非用户 ID）指定目标，响应 `data: null`，以 `()` 作为端点模型。
//!
//! 注意：openapi 与 Go 源码（`examples/alist/server/handles/user.go` 的
//! `DelUserCache`，读取 `username` 查询参数）均以用户名定位，
//! 与同组其余端点的 `id` 参数不同。

use alist_client_derive::EndpointRequest;

/// 清理用户缓存请求构建器。
///
/// 通过 [`User::del_cache`](super::User::del_cache) 创建。直接 `.await`
/// 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/user/del_cache", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标用户名（必选，作为 URL 查询参数 `username` 发送；非用户 ID）。
    #[query]
    username: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点没有可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, username: impl Into<String>) -> Self {
        Self {
            client,
            username: username.into(),
        }
    }
}

impl<'a> super::User<'a> {
    /// 清理指定用户的缓存。
    ///
    /// 对应 AList `POST /api/admin/user/del_cache`；成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/user/del_cache`、
    /// `docs/api/alistv3.md` 的 `# admin/user` 分组与
    /// `examples/alist/server/handles/user.go`（实现为 `DelUserCache`，
    /// 读取 `username` 查询参数后清除该用户的内存缓存）。
    ///
    /// 注意：本端点以**用户名**（而非用户 ID）定位目标。
    ///
    /// # Arguments
    ///
    /// * `username` - 目标用户名。
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
    /// client.admin().user().del_cache("alice").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn del_cache(&self, username: impl Into<String>) -> Request<'a> {
        Request::new(self.client, username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST /api/admin/user/del_cache，查询串携带 username，无请求体。
    #[test]
    fn build_request_composes_method_url_and_username_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "guest")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/user/del_cache"),
            "URL 应包含路径: {url}"
        );
        assert!(
            url.contains("username=guest"),
            "URL 应包含查询参数 username: {url}"
        );
        assert!(
            !url.contains("id="),
            "本端点按用户名而非用户 ID 定位: {url}"
        );
        assert!(built.body().is_none(), "本端点不应携带请求体");
        assert!(
            built.headers().get(reqwest::header::CONTENT_TYPE).is_none(),
            "无请求体的请求不应携带 JSON Content-Type"
        );
    }
}
