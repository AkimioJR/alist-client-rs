//! 单个任务控制端点请求构建器。
//!
//! 对应 `POST /api/task/{category}/{action}?tid={tid}`（取消 `cancel`、删除 `delete`、重试 `retry`；
//! 经管理员句柄 `client.admin().task()` 调用时路径前缀为 `/api/admin/task`），
//! 成功时响应 `data: null`，返回 `()`。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.11.0` 起，任务类别 `down`/`transfer` 更名为 `aria2_down`/`aria2_transfer`，
//!   并新增 `qbit_down`/`qbit_transfer`（`upload`/`copy` 不变）；老版本沿用旧类别名。
//! - 从 AList `v3.14.0` 起，新增 `retry` 动作；老版本无该动作。
//! - 从 AList `v3.29.0` 起，任务类别调整：新增 `offline_download` 与 `offline_download_transfer`，
//!   移除 `aria2_down`、`aria2_transfer`、`qbit_down`、`qbit_transfer`；同时任务 ID 统一为字符串形式
//!   （老版本 `upload`/`copy` 等类别的 `tid` 为数字 ID），且 `cancel`/`delete`/`retry` 对无效 `tid`
//!   由返回 400/500 错误改为一律返回成功响应。
//! - 从 AList `v3.39.1` 起，本接口同时挂载于用户级 `/api/task`（非游客认证即可访问，且仅能操作本人
//!   创建的任务：任务不存在或非本人任务一律返回 404），原管理员级 `/api/admin/task` 路径保留以兼容
//!   旧自动化脚本；老版本仅提供管理员级 `/api/admin/task` 且仅管理员可访问。
//! - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload` 任务类别；老版本不支持这两类任务。
//! - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别；老版本不支持该类任务。

use super::TaskScope;
use crate::schema::task::TaskCategory;

/// 单个任务控制请求构建器。
///
/// 用于取消、删除或重试指定的单个任务。
/// 对应 `POST /api/task/{category}/{action}?tid={tid}`（管理员作用域下前缀为
/// `/api/admin/task`，见 [`TaskScope`]），成功时响应 `data: null`。
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
        scope: TaskScope,
        category: TaskCategory,
        action: &'static str,
        tid: impl Into<String>,
    ) -> Self {
        Self {
            client,
            scope,
            category,
            action,
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
            "/api/{}/{}/{}",
            self.scope.as_str(),
            self.category.as_str(),
            self.action
        );
        self.client
            .request(reqwest::Method::POST, &path)
            .query(&[("tid", &self.tid)])
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
    /// - 从 AList `v3.11.0` 起，任务类别 `down`/`transfer` 更名为 `aria2_down`/`aria2_transfer`，
    ///   并新增 `qbit_down`/`qbit_transfer`（`upload`/`copy` 不变）；老版本沿用旧类别名。
    /// - 从 AList `v3.14.0` 起，新增 `retry` 动作；老版本无该动作。
    /// - 从 AList `v3.29.0` 起，任务类别调整：新增 `offline_download` 与 `offline_download_transfer`，
    ///   移除 `aria2_down`、`aria2_transfer`、`qbit_down`、`qbit_transfer`；同时任务 ID 统一为字符串形式
    ///   （老版本 `upload`/`copy` 等类别的 `tid` 为数字 ID），且 `cancel`/`delete`/`retry` 对无效 `tid`
    ///   由返回 400/500 错误改为一律返回成功响应。
    /// - 从 AList `v3.39.1` 起，本接口同时挂载于用户级 `/api/task`（非游客认证即可访问，且仅能操作本人
    ///   创建的任务：任务不存在或非本人任务一律返回 404），原管理员级 `/api/admin/task` 路径保留以兼容
    ///   旧自动化脚本；老版本仅提供管理员级 `/api/admin/task` 且仅管理员可访问。
    /// - 从 AList `v3.42.0` 起，新增 `decompress` 与 `decompress_upload` 任务类别；老版本不支持这两类任务。
    /// - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别；老版本不支持该类任务。
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
        let req = Request::new(
            &client,
            TaskScope::User,
            TaskCategory::OfflineDownload,
            "cancel",
            "t_1",
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/offline_download/cancel?tid=t_1"
        );
        assert!(built.body().is_none());
    }

    #[test]
    fn test_action_request_build_admin_scope() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(
            &client,
            TaskScope::Admin,
            TaskCategory::OfflineDownload,
            "retry",
            "t_1",
        );
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/admin/task/offline_download/retry?tid=t_1"
        );
        assert!(built.body().is_none());
    }
}
