//! fs 上传子句柄。
//!
//! 对应 `examples/alist/server/router.go` 中 `PUT /api/fs/put`（流式上传）与
//! `PUT /api/fs/form`（表单上传）两个路由；上传行为细节见
//! `examples/alist/server/handles/fsup.go`（`File-Path`/`As-Task`/`Overwrite`
//! 等请求头与哈希校验头）。

pub mod form;
pub mod put;

/// fs 上传句柄。
///
/// 通过 [`Fs::upload`](super::Fs::upload) 获取，例如
/// `client.fs().upload().put(...)`。
pub struct Upload<'a> {
    client: &'a crate::Client,
}

impl<'a> Upload<'a> {
    /// 创建上传句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}
