//! share 公开分享子句柄。
//!
//! 覆盖无需用户认证的公开分享端点（`/api/public/share` 前缀）：
//! 分享公开元信息（info）、分享密码换取访问令牌（auth）、浏览分享内目录
//! （list）与获取分享内对象详情（get）。
//!
//! 受密码保护的分享需先通过 [`Public::auth`] 以分享密码换取分享访问令牌；
//! 令牌随请求传递的方式遵循服务端读取优先级——浏览（list）与详情（get）
//! 通过请求体 `token` 字段，元信息（info）通过查询参数 `auth`。
//! 服务端还接受请求头 `X-Share-Token` 传递令牌，本 crate 未使用该通道。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口。

pub mod auth;
pub mod get;
pub mod info;
pub mod list;

/// share 公开分享句柄。
///
/// 作为公开分享端点方法的命名空间路由句柄，通过 [`Share::public`](super::Share::public)
/// 获取。本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Public<'a> {
    client: &'a crate::Client,
}

impl<'a> Public<'a> {
    /// 创建公开分享句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}
