//! 单个任务控制端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/{action}?tid={tid}`（取消 `cancel`、删除 `delete`、重试 `retry`），
//! 成功时响应 `data: null`，返回 `()`。

use crate::schema::task::TaskCategory;

/// 单个任务控制请求构建器。
///
/// 用于取消、删除或重试指定的单个任务。
/// 对应 `POST /api/task/{category}/{action}?tid={tid}`，成功时响应 `data: null`。
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    /// 底层 HTTP 客户端引用。
    ///
    /// 用于发送请求与执行认证注入、限速控制及响应反序列化。
    client: &'a crate::Client,

    /// 目标任务类别。
    ///
    /// 对应路径中的 `{category}` 段（如 `upload`、`copy` 等）。
    category: TaskCategory,

    /// 动作类型。
    ///
    /// 对应路径中的 `{action}` 段（`"cancel"`、`"delete"` 或 `"retry"`）。
    action: &'static str,

    /// 目标任务 ID（必选参数）。
    ///
    /// 作为 URL 查询参数 `tid` 传递给服务端。
    tid: String,
}

impl<'a> Request<'a> {
    /// 创建单任务控制请求构建器。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        category: TaskCategory,
        action: &'static str,
        tid: impl Into<String>,
    ) -> Self {
        Self {
            client,
            category,
            action,
            tid: tid.into(),
        }
    }

    /// 构建底层 HTTP POST 请求。
    ///
    /// 目标任务 ID 作为查询参数 `tid` 携带；不含认证头（认证由 `Client::execute` 发送时注入）。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!("/api/task/{}/{}", self.category.as_str(), self.action);
        self.client
            .request(reqwest::Method::POST, &path)
            .query(&[("tid", &self.tid)])
    }

    /// 发送请求并确认成功完成。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    #[inline]
    pub async fn send(&self) -> crate::Result<()> {
        self.client.execute(self.build_request()).await
    }

    /// 发送请求并反序列化为调用方指定的原始类型 `T`。
    ///
    /// # Errors
    ///
    /// 语义与 [`Self::send`] 一致，仅目标反序列化类型不同。
    #[inline]
    pub async fn send_raw<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        self.client.execute(self.build_request()).await
    }
}

impl<'a> core::future::IntoFuture for Request<'a> {
    type Output = crate::Result<()>;
    type IntoFuture = core::pin::Pin<
        Box<dyn core::future::Future<Output = Self::Output> + core::marker::Send + 'a>,
    >;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.send().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_request_build() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(&client, TaskCategory::OfflineDownload, "cancel", "t_1");
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/offline_download/cancel?tid=t_1"
        );
        assert!(built.body().is_none());
    }
}
