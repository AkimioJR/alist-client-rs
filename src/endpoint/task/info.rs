//! 单个任务详情查询端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/info?tid={tid}`，
//! 响应数据反序列化为 [`TaskInfo`]。

use crate::schema::task::{TaskCategory, TaskInfo};

/// 单个任务详情查询请求构建器。
///
/// 对应 `POST /api/task/{category}/info?tid={tid}`，服务端响应数据反序列化为 [`TaskInfo`]。
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

    /// 目标任务 ID（必选参数）。
    ///
    /// 作为 URL 查询参数 `tid` 传递给服务端。
    tid: String,
}

impl<'a> Request<'a> {
    /// 创建单任务查询请求构建器。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        category: TaskCategory,
        tid: impl Into<String>,
    ) -> Self {
        Self {
            client,
            category,
            tid: tid.into(),
        }
    }

    /// 构建底层 HTTP POST 请求。
    ///
    /// 目标任务 ID 作为查询参数 `tid` 携带；不含认证头（认证由 `Client::execute` 发送时注入）。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!("/api/task/{}/info", self.category.as_str());
        self.client
            .request(reqwest::Method::POST, &path)
            .query(&[("tid", &self.tid)])
    }

    /// 发送请求并反序列化为任务详情 [`TaskInfo`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败、AList 返回非成功状态码，或响应 `data` 无法反序列化为 [`TaskInfo`] 时，
    /// 返回 [`crate::Error`]。
    #[inline]
    pub async fn send(&self) -> crate::Result<TaskInfo> {
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
    type Output = crate::Result<TaskInfo>;
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
    fn test_info_request_build() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(&client, TaskCategory::Copy, "task_123");
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/copy/info?tid=task_123"
        );
        assert!(built.body().is_none());
    }
}
