//! admin-role 角色端点句柄。
//!
//! 覆盖 `/api/admin/role` 下的角色管理端点：
//! 列表、详情、创建、更新与删除。
//! 路由定义见 `examples/alist/server/router.go` 的 `role` 路由组
//! （openapi 文档未覆盖该分组，以 Go 源码为准）。
//! 通过 [`Admin::role`](super::Admin::role) 获取句柄，推荐即建即用。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

/// admin-role 角色句柄。
///
/// 作为角色端点方法的命名空间路由句柄，通过 [`Admin::role`](super::Admin::role) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Role<'a> {
    client: &'a crate::Client,
}

impl<'a> Role<'a> {
    /// 创建角色句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取角色管理子句柄。
    #[inline]
    #[must_use]
    pub fn role(&self) -> Role<'a> {
        Role::new(self.client)
    }
}
