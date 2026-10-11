//! admin-index 端点：清空搜索索引。
//!
//! 对应 `POST /api/admin/index/clear`；无请求体。服务端删除全部索引数据
//! 并将进度重置为「已完成 0 个对象」，成功时响应 `data` 为 `null`，
//! 以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.7.1` 起提供本接口。

use alist_client_derive::EndpointRequest;

/// 清空搜索索引请求构建器。
///
/// 通过 [`Index::clear`](super::Index::clear) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/index/clear", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无业务参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Index<'a> {
    /// 清空搜索索引。
    ///
    /// 对应 AList `POST /api/admin/index/clear`；服务端删除全部索引数据，
    /// 并将构建进度重置为「已完成、0 个对象、无错误」，成功时响应 `data`
    /// 为 `null`。已有构建任务进行中时服务端返回 `index is running`
    /// 错误响应；重置后的进度经 [`progress`](super::Index::progress) 查询。
    /// 数据来源：AList 服务端路由定义的 admin `index` 分组与
    /// AList 服务端索引处理模块（实现为 `ClearIndex`）。
    ///
    /// # Arguments
    ///
    /// 本端点无业务参数。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如已有构建任务进行中时返回 `index is running`）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.7.1` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().index().clear().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn clear(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径断言；本端点不应携带请求体。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/index/clear"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "本端点不应有查询参数: {url}");
        assert!(built.body().is_none(), "本端点不应携带请求体");
    }

    /// 收发路径：mock 服务器返回 `data: null`，以 `()` 解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        client.admin().index().clear().send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/index/clear "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("authorization: admin-token"),
            "应注入认证头: {}",
            recorded[0]
        );
    }
}
