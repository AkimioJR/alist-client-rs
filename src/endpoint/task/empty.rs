//! 无参任务控制端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/{action}`（清空已完成 `clear_done`、
//! 清空已成功 `clear_succeeded`、重试所有失败 `retry_failed`；
//! 经管理员句柄 `client.admin().task()` 调用时路径前缀为 `/api/admin/task`），
//! 成功时响应 `data: null`，返回 `()`。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.14.0` 起，新增 `clear_succeeded` 动作；老版本不支持该动作。
//! - 从 AList `v3.29.0` 起，新增 `retry_failed` 动作；老版本不支持该动作。
//! - 从 AList `v3.29.0` 起，新增 `offline_download` 与 `offline_download_transfer`
//!   任务类别路由分支；老版本不支持这两个类别的任务控制操作。
//! - 从 AList `v3.39.1` 起，任务控制路由由 `/api/admin/task`（仅管理员可用）
//!   迁往 `/api/task`（登录非访客用户可用，且仅能操作本人创建的任务），
//!   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
//! - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload`
//!   任务类别路由分支；老版本不支持这两个类别的任务控制操作。
//! - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别路由分支；
//!   老版本不支持该类别的任务控制操作。

use super::TaskScope;
use crate::schema::task::TaskCategory;

/// 无参任务控制请求构建器。
///
/// 用于全局清空已完成/成功任务或重试全部失败任务。
/// 对应 `POST /api/task/{category}/{action}`（管理员作用域下前缀为 `/api/admin/task`，
/// 见 [`TaskScope`]），无请求体与查询参数，成功时响应 `data: null`。
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

    /// 动作类型。
    ///
    /// 对应路径中的 `{action}` 段（`"clear_done"`、`"clear_succeeded"` 或 `"retry_failed"`）。
    action: &'static str,
}

impl<'a> Request<'a> {
    /// 创建无参任务控制请求构建器。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        scope: TaskScope,
        category: TaskCategory,
        action: &'static str,
    ) -> Self {
        Self {
            client,
            scope,
            category,
            action,
        }
    }

    /// 构建底层 HTTP POST 请求。
    ///
    /// 路径形如 `/api/task/{category}/{action}`（管理员作用域下前缀为 `/api/admin/task`）；
    /// 无请求参数与请求体。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!(
            "/api/{}/{}/{}",
            self.scope.as_str(),
            self.category.as_str(),
            self.action
        );
        self.client.request(reqwest::Method::POST, &path)
    }

    /// 发送请求并确认成功完成。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.14.0` 起，新增 `clear_succeeded` 动作；老版本不支持该动作。
    /// - 从 AList `v3.29.0` 起，新增 `retry_failed` 动作；老版本不支持该动作。
    /// - 从 AList `v3.29.0` 起，新增 `offline_download` 与 `offline_download_transfer`
    ///   任务类别路由分支；老版本不支持这两个类别的任务控制操作。
    /// - 从 AList `v3.39.1` 起，任务控制路由由 `/api/admin/task`（仅管理员可用）
    ///   迁往 `/api/task`（登录非访客用户可用，且仅能操作本人创建的任务），
    ///   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
    /// - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload`
    ///   任务类别路由分支；老版本不支持这两个类别的任务控制操作。
    /// - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别路由分支；
    ///   老版本不支持该类别的任务控制操作。
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
    fn test_empty_request_build() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(
            &client,
            TaskScope::User,
            TaskCategory::S3Transition,
            "clear_done",
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/s3_transition/clear_done"
        );
        assert!(built.url().query().is_none());
        assert!(built.body().is_none());
    }

    #[test]
    fn test_empty_request_build_admin_scope() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(
            &client,
            TaskScope::Admin,
            TaskCategory::S3Transition,
            "clear_done",
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/admin/task/s3_transition/clear_done"
        );
        assert!(built.url().query().is_none());
        assert!(built.body().is_none());
    }
}
