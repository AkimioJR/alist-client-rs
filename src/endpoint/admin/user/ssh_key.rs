//! admin-user SSH 公钥管理子句柄。
//!
//! 覆盖 `/api/admin/user/sshkey` 下的用户 SSH 公钥管理端点：
//! 列出公钥与删除公钥。
//! 通过 [`User::ssh_key`](super::User::ssh_key) 获取句柄，推荐即建即用。

pub mod delete;
pub mod list;

/// admin-user SSH 公钥管理子句柄。
///
/// 作为用户 SSH 公钥端点方法的命名空间路由句柄，通过 [`User::ssh_key`](super::User::ssh_key) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct SshKey<'a> {
    client: &'a crate::Client,
}

impl<'a> SshKey<'a> {
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}
