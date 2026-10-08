//! admin 管理端点句柄。
//!
//! 按子域划分为元信息（[`meta`]）、用户（[`user`]）、存储（[`storage`]）、
//! 驱动（[`driver`]）、设置（[`setting`]）、上传任务（[`task`]）、角色（[`role`]）、
//! 标签（[`label`]）与标签绑定（[`label_file_binding`]），
//! API 路径见 AList OpenAPI 规范的 `admin` 分组与
//! AList 服务端路由定义的 `admin` 路由。
//!
//! 通过 [`Client::admin`](crate::Client::admin) 获取句柄，推荐即建即用：
//!
//! ```no_run
//! use alist_client::{Authentication, Client};
//!
//! # async fn example() -> alist_client::Result<()> {
//! let client = Client::new("https://alist.example.com")?
//!     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
//! let admin = client.admin();
//! # let _ = admin;
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "admin-meta")]
pub mod meta;

#[cfg(feature = "admin-user")]
pub mod user;

#[cfg(feature = "admin-storage")]
pub mod storage;

#[cfg(feature = "admin-driver")]
pub mod driver;

#[cfg(feature = "admin-setting")]
pub mod setting;

#[cfg(feature = "admin-task")]
pub mod task;

#[cfg(feature = "admin-role")]
pub mod role;

#[cfg(feature = "admin-label")]
pub mod label;

#[cfg(feature = "admin-label-file-binding")]
pub mod label_file_binding;

/// admin 管理句柄。
///
/// 作为管理端点方法的命名空间路由句柄，通过 [`Client::admin`](crate::Client::admin) 获取。
/// 本身不包含业务状态；各子域访问器随对应的 `admin-*` feature 启用而存在。
pub struct Admin<'a> {
    client: &'a crate::Client,
}

impl crate::Client {
    /// 获取 admin 管理句柄。
    ///
    /// 返回用于访问管理端点的 [`Admin`] 句柄，推荐直接链式调用端点方法。
    #[inline]
    #[must_use]
    pub fn admin(&self) -> Admin<'_> {
        Admin { client: self }
    }
}
