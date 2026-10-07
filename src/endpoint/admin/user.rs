//! admin-user 用户端点句柄。
//!
//! 覆盖 `/api/admin/user` 下的用户管理端点：
//! 列表、详情、创建、更新、取消两步验证、删除与缓存清理。
//! 通过 [`Admin::user`](super::Admin::user) 获取句柄，推荐即建即用。

pub mod cancel_2fa;
pub mod create;
pub mod del_cache;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

/// admin-user 用户句柄。
///
/// 作为用户端点方法的命名空间路由句柄，通过 [`Admin::user`](super::Admin::user) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct User<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> User<'a> {
    /// 创建用户句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取用户管理子句柄。
    #[inline]
    #[must_use]
    pub fn user(&self) -> User<'a> {
        User::new(self.client)
    }
}
