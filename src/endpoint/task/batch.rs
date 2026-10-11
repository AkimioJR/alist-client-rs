//! 批量任务控制端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/{action}`（批量取消 `cancel_some`、
//! 批量删除 `delete_some`、批量重试 `retry_some`；
//! 经管理员句柄 [`crate::Client::admin_task`] 调用时路径前缀为 `/api/admin/task`），
//! 请求体为 JSON 数组字符串，响应数据反序列化为 [`BatchTaskResult`]。
//!
//! # Version
//!
//! - 自 AList `v3.41.0` 起提供本接口。
//! - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload` 任务类别路由分支；
//!   老版本不支持这两个类别的批量任务操作。
//! - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别路由分支；
//!   老版本不支持该类别的批量任务操作。

use super::TaskScope;
use crate::schema::task::{BatchTaskResult, TaskCategory};

/// 批量任务控制请求构建器。
///
/// 用于向指定类别批量发送任务取消、删除或重试请求。
/// 对应 `POST /api/task/{category}/{action}`（管理员作用域下前缀为 `/api/admin/task`，
/// 见 [`TaskScope`]），服务端响应数据反序列化为 [`BatchTaskResult`]。
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    /// 底层 HTTP 客户端引用。
    ///
    /// 用于发送请求与执行认证注入、限速控制及响应反序列化。
    client: &'a crate::Client,

    /// 路由前缀作用域（必选参数）。
    ///
    /// 决定路径前缀：[`TaskScope::User`] 为 `/api/task`，[`TaskScope::Admin`] 为
    /// `/api/admin/task`；访问前提说明见 [`TaskScope`]。
    scope: TaskScope,

    /// 目标任务类别。
    ///
    /// 对应路径中的 `{category}` 段（如 `upload`、`copy` 等）。
    category: TaskCategory,

    /// 批量动作类型。
    ///
    /// 对应路径中的 `{action}` 段（`"cancel_some"`、`"delete_some"` 或 `"retry_some"`）。
    action: &'static str,

    /// 目标任务 ID 列表（必选参数）。
    ///
    /// 作为 JSON 数组请求体发送给服务端。
    tids: Vec<String>,
}

impl<'a> Request<'a> {
    /// 创建批量任务操作请求构建器。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        scope: TaskScope,
        category: TaskCategory,
        action: &'static str,
        tids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            client,
            scope,
            category,
            action,
            tids: tids.into_iter().map(Into::into).collect(),
        }
    }

    /// 构建底层 HTTP POST 请求。
    ///
    /// 路径形如 `/api/task/{category}/{action}`（管理员作用域下前缀为 `/api/admin/task`），
    /// 请求体携带 JSON 数组 `tids`。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!(
            "/api/{}/{}/{}",
            self.scope.as_str(),
            self.category.as_str(),
            self.action
        );
        self.client
            .request(reqwest::Method::POST, &path)
            .json(&self.tids)
    }

    /// 发送请求并反序列化为批量操作结果映射 [`BatchTaskResult`]。
    ///
    /// 成功处理的任务不会出现在返回的映射中；未成功执行的任务其 ID 将映射到失败原因说明。
    ///
    /// # Errors
    ///
    /// 当网络请求失败、AList 返回非成功状态码，或响应 `data` 无法反序列化为 [`BatchTaskResult`] 时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.41.0` 起提供本接口。
    /// - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload` 任务类别路由分支；
    ///   老版本不支持这两个类别的批量任务操作。
    /// - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别路由分支；
    ///   老版本不支持该类别的批量任务操作。
    #[inline]
    pub async fn send(&self) -> crate::Result<BatchTaskResult> {
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
    type Output = crate::Result<BatchTaskResult>;
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
    fn test_batch_request_build() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(
            &client,
            TaskScope::User,
            TaskCategory::Decompress,
            "retry_some",
            ["a", "b"],
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/decompress/retry_some"
        );
        assert!(built.body().is_some());
    }

    #[test]
    fn test_batch_request_build_admin_scope() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(
            &client,
            TaskScope::Admin,
            TaskCategory::Decompress,
            "retry_some",
            ["a", "b"],
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/admin/task/decompress/retry_some"
        );
        assert!(built.body().is_some());
    }
}
