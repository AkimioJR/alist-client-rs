//! admin-session 会话管理端点句柄。
//!
//! 覆盖 `/api/admin/session` 下的会话管理端点：
//! 管理员查询全部登录会话（[`list`]）与踢出任意会话（[`evict`]）。
//! 会话列表元素复用 auth 域的 [`SessionResponse`](crate::schema::admin::session::SessionResponse)
//! （服务端为同一响应结构体，管理员列表总会携带 `user_id`）。
//! 通过 [`Admin::session`](super::Admin::session) 获取句柄，推荐即建即用。

pub mod evict;
pub mod list;

/// admin-session 会话管理句柄。
///
/// 作为会话管理端点方法的命名空间路由句柄，通过 [`Admin::session`](super::Admin::session)
/// 获取。本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Session<'a> {
    client: &'a crate::Client,
}

impl<'a> Session<'a> {
    /// 创建会话管理句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取会话管理子句柄。
    #[inline]
    #[must_use]
    pub fn session(&self) -> Session<'a> {
        Session::new(self.client)
    }
}
