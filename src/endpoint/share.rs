//! share 分享端点句柄。
//!
//! 覆盖两组分享端点（AList OpenAPI 规范未收录该分组，路径以 AList 服务端
//! 路由定义为准）：
//!
//! - **公开分享**（无需用户认证，`/api/public/share` 前缀）：获取分享公开元信息、
//!   分享密码换取访问令牌、浏览分享内目录与获取分享内对象详情，
//!   通过 [`Share::public`] 子句柄访问；
//! - **分享管理**（需登录非访客，`/api/share` 前缀）：创建、更新、禁用、
//!   列出与删除当前用户创建的分享，由本句柄直接访问。
//!
//! ## 分享访问令牌与用户 token 的区别
//!
//! 受密码保护的分享使用独立的「分享访问令牌」：先通过
//! [`Share::public`] 的鉴权端点以分享密码换取，再随请求传给公开分享端点。
//! 服务端读取令牌的优先级为：请求体 `token` 字段（浏览/详情端点）、
//! 查询参数 `auth`（元信息端点）、请求头 `X-Share-Token`（本 crate 未使用）。
//! 它与用户登录 token（`Authorization` 请求头，由 [`Client`](crate::Client)
//! 自动注入）完全独立：公开分享端点不消费用户 token，分享管理端点不消费分享令牌。
//!
//! 通过 [`Client::share`](crate::Client::share) 获取句柄，推荐即建即用。
//!
//! # Version
//!
//! - 自 AList `v3.59.0` 起提供本接口（公开分享与分享管理端点同期引入）。

pub mod create;
pub mod delete;
pub mod disable;
pub mod list;
pub mod public;
pub mod update;

/// share 分享句柄。
///
/// 作为分享端点方法的命名空间路由句柄，通过 [`Client::share`](crate::Client::share)
/// 获取。本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用；
/// 公开分享端点经 [`Share::public`] 子句柄访问。
pub struct Share<'a> {
    client: &'a crate::Client,
}

impl crate::Client {
    /// 获取 share 分享句柄。
    ///
    /// 公开分享端点（[`Share::public`]）无需任何认证；分享管理端点
    /// （创建/更新/禁用/列表/删除）要求登录非访客用户的用户 token
    /// （服务端路由组挂载认证与非访客校验中间件）。
    #[inline]
    #[must_use]
    pub fn share(&self) -> Share<'_> {
        Share { client: self }
    }
}

impl<'a> Share<'a> {
    /// 获取公开分享子句柄。
    ///
    /// 对应 `/api/public/share` 前缀下的无需认证端点：
    /// 分享公开元信息、密码鉴权、分享目录浏览与分享对象详情。
    #[inline]
    #[must_use]
    pub fn public(&self) -> public::Public<'a> {
        public::Public::new(self.client)
    }
}
