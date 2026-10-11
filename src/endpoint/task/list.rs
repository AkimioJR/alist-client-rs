//! 任务列表查询端点请求构建器。
//!
//! 对应 `GET /api/task/{category}/{action}`（查询未完成列表 `undone` 或已完成列表 `done`），
//! 响应数据反序列化为 [`TaskInfoList`]。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.11.0` 起，任务类别路径段调整：`down`、`transfer` 分别更名为
//!   `aria2_down`、`aria2_transfer`，并新增 `qbit_down`、`qbit_transfer`；
//!   老版本沿用 `down`、`transfer` 类别路径段。
//! - 从 AList `v3.29.0` 起，离线下载任务重构为统一的 `offline_download` 与
//!   `offline_download_transfer` 类别，并移除 `aria2_down`、`aria2_transfer`、
//!   `qbit_down`、`qbit_transfer`；老版本沿用上述已移除的类别路径段。
//! - 从 AList `v3.39.1` 起，任务路由由 `/api/admin/task`（仅管理员可用）
//!   迁往 `/api/task`（登录非访客用户可用，且仅能查询本人创建的任务），
//!   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
//! - 从 AList `v3.39.1` 起，响应新增 `creator` 与 `creator_role` 字段；
//!   老版本响应缺失这两个字段。
//! - 从 AList `v3.42.0` 起，响应新增 `start_time`、`end_time` 与 `total_bytes` 字段；
//!   老版本响应缺失这三个字段。
//! - 从 AList `v3.42.0` 起，新增 `decompress`、`decompress_upload` 任务类别；
//!   老版本不支持这两个类别路径段。
//! - 从 AList `v3.46.0` 起，响应 `creator_role` 由单个角色整数改为角色 ID 数组；
//!   老版本为单个整数。
//! - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别；
//!   老版本不支持该类别路径段。

use crate::schema::task::{TaskCategory, TaskInfoList};

/// 任务列表查询请求构建器。
///
/// 用于查询指定类别下的未完成（`undone`）或已完成（`done`）任务列表。
/// 对应 `GET /api/task/{category}/{action}`，服务端响应数据反序列化为 [`TaskInfoList`]。
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

    /// 列表查询动作类型。
    ///
    /// 对应路径中的 `{action}` 段（固定为 `"done"` 或 `"undone"`）。
    action: &'static str,
}

impl<'a> Request<'a> {
    /// 创建任务列表请求构建器。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        category: TaskCategory,
        action: &'static str,
    ) -> Self {
        Self {
            client,
            category,
            action,
        }
    }

    /// 构建底层 HTTP GET 请求。
    ///
    /// 不含认证头（认证由 `Client::execute` 发送时注入）。
    #[inline]
    pub(crate) fn build_request(&self) -> reqwest::RequestBuilder {
        let path = format!("/api/task/{}/{}", self.category.as_str(), self.action);
        self.client.request(reqwest::Method::GET, &path)
    }

    /// 发送请求并反序列化为任务列表 [`TaskInfoList`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败、AList 返回非成功状态码，或响应 `data` 无法反序列化为 [`TaskInfoList`] 时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.11.0` 起，任务类别路径段调整：`down`、`transfer` 分别更名为
    ///   `aria2_down`、`aria2_transfer`，并新增 `qbit_down`、`qbit_transfer`；
    ///   老版本沿用 `down`、`transfer` 类别路径段。
    /// - 从 AList `v3.29.0` 起，离线下载任务重构为统一的 `offline_download` 与
    ///   `offline_download_transfer` 类别，并移除 `aria2_down`、`aria2_transfer`、
    ///   `qbit_down`、`qbit_transfer`；老版本沿用上述已移除的类别路径段。
    /// - 从 AList `v3.39.1` 起，任务路由由 `/api/admin/task`（仅管理员可用）
    ///   迁往 `/api/task`（登录非访客用户可用，且仅能查询本人创建的任务），
    ///   并保留 `/api/admin/task` 作为兼容别名；老版本仅提供 `/api/admin/task`。
    /// - 从 AList `v3.39.1` 起，响应新增 `creator` 与 `creator_role` 字段；
    ///   老版本响应缺失这两个字段。
    /// - 从 AList `v3.42.0` 起，响应新增 `start_time`、`end_time` 与 `total_bytes` 字段；
    ///   老版本响应缺失这三个字段。
    /// - 从 AList `v3.42.0` 起，新增 `decompress`、`decompress_upload` 任务类别；
    ///   老版本不支持这两个类别路径段。
    /// - 从 AList `v3.46.0` 起，响应 `creator_role` 由单个角色整数改为角色 ID 数组；
    ///   老版本为单个整数。
    /// - 从 AList `v3.54.0` 起，新增 `s3_transition` 任务类别；
    ///   老版本不支持该类别路径段。
    #[inline]
    pub async fn send(&self) -> crate::Result<TaskInfoList> {
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
    type Output = crate::Result<TaskInfoList>;
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
    fn test_list_request_build() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let req = Request::new(&client, TaskCategory::Upload, "undone");
        let built = req.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        assert_eq!(
            built.url().as_str(),
            "https://alist.example/api/task/upload/undone"
        );
        assert!(built.url().query().is_none());
        assert!(built.body().is_none());
    }
}
