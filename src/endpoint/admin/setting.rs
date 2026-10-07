//! admin-setting 设置端点句柄。
//!
//! 覆盖 `/api/admin/setting` 下的站点设置管理端点：
//! 列表、查询、保存、删除、重置 token 与 aria2/qBittorrent 快捷配置。
//! 通过 [`Admin::setting`](super::Admin::setting) 获取句柄，推荐即建即用。

pub mod delete;
pub mod get;
pub mod list;
pub mod reset_token;
pub mod save;
pub mod set_aria2;
pub mod set_qbit;

/// admin-setting 设置句柄。
///
/// 作为设置端点方法的命名空间路由句柄，通过 [`Admin::setting`](super::Admin::setting) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Setting<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> Setting<'a> {
    /// 创建设置句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取设置管理子句柄。
    #[inline]
    #[must_use]
    pub fn setting(&self) -> Setting<'a> {
        Setting::new(self.client)
    }
}
