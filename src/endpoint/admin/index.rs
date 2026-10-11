//! admin-index 搜索索引端点句柄。
//!
//! 覆盖 `/api/admin/index` 下的搜索索引管理端点：
//! 全量构建（[`build`]）、按路径增量更新（[`update`]）、停止（[`stop`]）、
//! 清空（[`clear`]）与进度查询（[`progress`]）。
//! 服务端将上述端点置于 admin 路由组并额外挂载搜索索引中间件
//! （未启用全文搜索时服务端拒绝访问）；构建/更新在服务端后台异步执行，
//! 进程重启后进行中的构建任务中断，最后一次进度快照仍持久化于服务端
//! 设置项中，可继续经 [`progress`] 查询。
//! 通过 [`Admin::index`](super::Admin::index) 获取句柄，
//! 推荐即建即用。

pub mod build;
pub mod clear;
pub mod progress;
pub mod stop;
pub mod update;

/// admin-index 搜索索引句柄。
///
/// 作为搜索索引端点方法的命名空间路由句柄，通过 [`Admin::index`](super::Admin::index)
/// 获取。本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Index<'a> {
    client: &'a crate::Client,
}

impl<'a> Index<'a> {
    /// 创建搜索索引句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取搜索索引管理子句柄。
    #[inline]
    #[must_use]
    pub fn index(&self) -> Index<'a> {
        Index::new(self.client)
    }
}
