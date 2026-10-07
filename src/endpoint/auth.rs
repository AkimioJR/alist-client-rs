//! auth 认证端点句柄。
//!
//! 覆盖登录（明文/哈希）、注册、两步验证与当前用户信息等端点，
//! API 路径见 `docs/api/alistv3.openapi.yaml` 的 `auth` 分组。
//!
//! 通过 [`Client::auth`](crate::Client::auth) 获取句柄，推荐即建即用：
//!
//! ```no_run
//! use alist_client::{Authentication, Client};
//!
//! # async fn example() -> alist_client::Result<()> {
//! let client = Client::new("https://alist.example.com")?
//!     .with_authentication(Authentication::token("TOKEN".to_owned()));
//! // 即建即用：直接链式调用端点方法（如 client.auth().me()）
//! let auth = client.auth();
//! # let _ = auth;
//! # Ok(())
//! # }
//! ```

pub mod generate_2fa;
pub mod login;
pub mod login_hash;
pub mod me;
pub mod register;
pub mod verify_2fa;

/// auth 认证句柄。
///
/// 作为认证端点方法的命名空间路由句柄，通过 [`Client::auth`](crate::Client::auth) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Auth<'a> {
    client: &'a crate::Client,
}

impl crate::Client {
    /// 获取 auth 认证句柄。
    ///
    /// 返回用于访问认证端点的 [`Auth`] 句柄，推荐直接链式调用端点方法。
    #[inline]
    #[must_use]
    pub fn auth(&self) -> Auth<'_> {
        Auth { client: self }
    }
}
