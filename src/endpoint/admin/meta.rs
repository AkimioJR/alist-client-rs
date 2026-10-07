//! admin-meta 元信息端点句柄。
//!
//! 覆盖 `/api/admin/meta` 下的目录元信息（密码/隐藏/说明等）管理端点：
//! 列表、详情、创建、更新与删除。
//! 通过 [`Admin::meta`](super::Admin::meta) 获取句柄，推荐即建即用。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

/// admin-meta 元信息句柄。
///
/// 作为元信息端点方法的命名空间路由句柄，通过 [`Admin::meta`](super::Admin::meta) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Meta<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> Meta<'a> {
    /// 创建元信息句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取元信息管理子句柄。
    #[inline]
    #[must_use]
    pub fn meta(&self) -> Meta<'a> {
        Meta::new(self.client)
    }
}
