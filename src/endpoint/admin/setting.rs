//! admin-setting 设置端点句柄。
//!
//! 覆盖 `/api/admin/setting` 下的站点设置管理端点：
//! 列表、查询、保存、删除、重置/设置令牌、aria2/qBittorrent/Transmission 快捷配置、
//! 各网盘离线下载临时目录配置（115/PikPak/迅雷/光雅盘/123 Open）
//! 与 FRP 内网穿透配置/停止/运行时查询。
//! 通过 [`Admin::setting`](super::Admin::setting) 获取句柄，推荐即建即用。

pub mod delete;
pub mod frp_runtime;
pub mod get;
pub mod get_by_keys;
pub mod list;
pub mod reset_token;
pub mod save;
pub mod set_115;
pub mod set_123_open;
pub mod set_aria2;
pub mod set_frp;
pub mod set_guangyapan;
pub mod set_pikpak;
pub mod set_qbit;
pub mod set_thunder;
pub mod set_token;
pub mod set_transmission;
pub mod stop_frp;

/// admin-setting 设置句柄。
///
/// 作为设置端点方法的命名空间路由句柄，通过 [`Admin::setting`](super::Admin::setting) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Setting<'a> {
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
