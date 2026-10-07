//! fs 归档子句柄。
//!
//! 对应 `examples/alist/server/router.go` 中 `/api/fs/archive` 下的
//! `meta`、`list`、`decompress` 三个路由，用于浏览压缩包内容与解压。

pub mod decompress;
pub mod list;
pub mod meta;

/// fs 归档句柄。
///
/// 通过 [`Fs::archive`](super::Fs::archive) 获取，例如
/// `client.fs().archive().meta(...)`。
pub struct Archive<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> Archive<'a> {
    /// 创建归档句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}
