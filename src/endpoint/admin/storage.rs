//! admin-storage 存储端点句柄。
//!
//! 覆盖 `/api/admin/storage` 下的存储驱动实例管理端点：
//! 列表、详情、创建、更新、启用、禁用、删除与全量重载。
//! 通过 [`Admin::storage`](super::Admin::storage) 获取句柄，推荐即建即用。

pub mod create;
pub mod delete;
pub mod disable;
pub mod enable;
pub mod get;
pub mod list;
pub mod load_all;
pub mod update;

/// admin-storage 存储句柄。
///
/// 作为存储端点方法的命名空间路由句柄，通过 [`Admin::storage`](super::Admin::storage) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Storage<'a> {
    client: &'a crate::Client,
}

impl<'a> Storage<'a> {
    /// 创建存储句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取存储管理子句柄。
    #[inline]
    #[must_use]
    pub fn storage(&self) -> Storage<'a> {
        Storage::new(self.client)
    }
}
