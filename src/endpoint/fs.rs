//! fs 文件系统端点句柄。
//!
//! 覆盖目录列表、文件信息、目录树、搜索、目录/重命名/移动/复制/删除、
//! 离线下载以及归档（[`archive`]）与上传（[`upload`]）子句柄，
//! API 路径见 AList OpenAPI 规范的 `fs` 分组与
//! AList 服务端路由定义的 `_fs` 路由。
//!
//! 通过 [`Client::fs`](crate::Client::fs) 获取句柄，推荐即建即用。

pub mod add_offline_download;
pub mod archive;
pub mod batch_rename;
pub mod copy;
pub mod dirs;
pub mod get;
pub mod list;
pub mod mkdir;
pub mod move_items;
pub mod recursive_move;
pub mod regex_rename;
pub mod remove;
pub mod remove_empty_directory;
pub mod rename;
pub mod search;
pub mod upload;

/// fs 文件系统句柄。
///
/// 作为文件系统端点方法的命名空间路由句柄，通过 [`Client::fs`](crate::Client::fs) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Fs<'a> {
    client: &'a crate::Client,
}

impl crate::Client {
    /// 获取 fs 文件系统句柄。
    ///
    /// 返回用于访问文件系统端点的 [`Fs`] 句柄，推荐直接链式调用端点方法。
    #[inline]
    #[must_use]
    pub fn fs(&self) -> Fs<'_> {
        Fs { client: self }
    }
}

impl<'a> Fs<'a> {
    /// 获取归档（压缩包浏览/解压）子句柄。
    #[inline]
    #[must_use]
    pub fn archive(&self) -> archive::Archive<'a> {
        archive::Archive::new(self.client)
    }

    /// 获取上传子句柄。
    #[inline]
    #[must_use]
    pub fn upload(&self) -> upload::Upload<'a> {
        upload::Upload::new(self.client)
    }
}
