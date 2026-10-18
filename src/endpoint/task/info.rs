//! 单个任务详情查询端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/info?tid={tid}`（经管理员句柄
//! `client.admin().task()` 调用时路径前缀为 `/api/admin/task`），
//! 响应数据反序列化为 [`TaskInfo`]。
//!
//! # Version
//!
//! - 自 AList `v3.30.0` 起提供本接口。
//! - 从 AList `v3.39.1` 起，任务路由由 `/api/admin/task`（仅管理员可用）
//!   迁往 `/api/task`（登录非访客用户可用，且仅能查询本人创建的任务），
//!   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
//! - 从 AList `v3.39.1` 起，响应新增 `creator` 与 `creator_role` 字段；
//!   老版本响应缺失这两个字段。
//! - 从 AList `v3.42.0` 起，响应新增 `start_time`、`end_time` 与 `total_bytes` 字段；
//!   老版本响应缺失这三个字段。
//! - 从 AList `v3.46.0` 起，响应 `creator_role` 由单个角色整数改为角色 ID 数组；
//!   老版本为单个整数。

use super::TaskScope;
use crate::schema::task::{TaskCategory, TaskInfo};

/// 单个任务详情查询请求构建器。
///
/// 对应 `POST /api/task/{category}/info?tid={tid}`（管理员作用域下前缀为
/// `/api/admin/task`，见 [`TaskScope`]），服务端响应数据反序列化为 [`TaskInfo`]。
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
        scope: TaskScope,
        category: TaskCategory,
        tid: impl Into<String>,
    ) -> Self {
        Self {
            client,
            scope,
            category,
            tid: tid.into(),
        }
    }

    /// 构建底层 HTTP POST 请求。
    ///
    /// 路径前缀随作用域为 `/api/task` 或 `/api/admin/task`；
    /// 目标任务 ID 作为查询参数 `tid` 携带；不含认证头（认证由 `Client::execute` 发送时注入）。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!(
            "/api/{}/{}/info",
            self.scope.as_str(),
            self.category.as_str()
        );
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
    ///
    /// # Version
    ///
    /// - 自 AList `v3.30.0` 起提供本接口。
    /// - 从 AList `v3.39.1` 起，任务路由由 `/api/admin/task`（仅管理员可用）
    ///   迁往 `/api/task`（登录非访客用户可用，且仅能查询本人创建的任务），
    ///   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
    /// - 从 AList `v3.39.1` 起，响应新增 `creator` 与 `creator_role` 字段；
    ///   老版本响应缺失这两个字段。
    /// - 从 AList `v3.42.0` 起，响应新增 `start_time`、`end_time` 与 `total_bytes` 字段；
    ///   老版本响应缺失这三个字段。
    /// - 从 AList `v3.46.0` 起，响应 `creator_role` 由单个角色整数改为角色 ID 数组；
    ///   老版本为单个整数。
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
        let req = Request::new(&client, TaskScope::User, TaskCategory::Copy, "task_123");
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/copy/info?tid=task_123"
        );
        assert!(built.body().is_none());
    }

    #[test]
    fn test_info_request_build_admin_scope() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(&client, TaskScope::Admin, TaskCategory::Copy, "task_123");
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/admin/task/copy/info?tid=task_123"
        );
        assert!(built.body().is_none());
    }
}
