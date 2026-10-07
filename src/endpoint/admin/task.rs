//! admin-task 上传任务端点句柄。
//!
//! 覆盖 `/api/admin/task/upload` 下的后台上传任务端点：
//! 任务详情、完成/未完成列表、删除、取消、重试与清空。
//! 通过 [`Admin::task`](super::Admin::task) 获取句柄，推荐即建即用。

pub mod cancel;
pub mod clear_done;
pub mod clear_succeeded;
pub mod delete;
pub mod done;
pub mod info;
pub mod retry;
pub mod undone;

/// admin-task 上传任务句柄。
///
/// 作为上传任务端点方法的命名空间路由句柄，通过 [`Admin::task`](super::Admin::task) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Task<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> Task<'a> {
    /// 创建上传任务句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取上传任务管理子句柄。
    #[inline]
    #[must_use]
    pub fn task(&self) -> Task<'a> {
        Task::new(self.client)
    }
}
