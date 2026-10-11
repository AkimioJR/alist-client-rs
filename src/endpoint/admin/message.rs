//! admin-message 消息队列端点句柄。
//!
//! 覆盖 `/api/admin/message` 下的消息队列端点：
//! 取用待推送消息（[`get`]）与投递应答消息（[`send`]）。
//! 消息队列为服务端与前端之间的双向交互通道（例如部分驱动需要用户输入
//! 验证码时，先经队列请求消息，再由前端经 `send` 投递应答）；
//! 队列为单条容量，无消费者时投递失败、队列为空时取用返回错误响应。
//! 通过 [`Admin::message`](super::Admin::message) 获取句柄，推荐即建即用。

pub mod get;
pub mod send;

/// admin-message 消息队列句柄。
///
/// 作为消息队列端点方法的命名空间路由句柄，通过 [`Admin::message`](super::Admin::message)
/// 获取。本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Message<'a> {
    client: &'a crate::Client,
}

impl<'a> Message<'a> {
    /// 创建消息队列句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取消息队列管理子句柄。
    #[inline]
    #[must_use]
    pub fn message(&self) -> Message<'a> {
        Message::new(self.client)
    }
}
