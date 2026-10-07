//! AList v3 API 的异步 Rust 客户端。
//!
//! 本 crate 将 AList 的 JSON 信封与文件系统/管理端点建模为强类型 Rust API：
//! 数据模型按 API 分组放在 [`schema`] 下，端点请求构建器放在 [`endpoint`] 下，
//! 由 `alist-client-derive` 派生宏生成 `build_request`/`send`/`IntoFuture`/setter，
//! 约定详见 `docs/design.md`。
//!
//! # 快速上手
//!
//! ```no_run
//! use alist_client::{Authentication, Client};
//!
//! # async fn example() -> alist_client::Result<()> {
//! // 1. 创建客户端：传入站点地址（支持带路径前缀的反代部署）
//! let mut client = Client::new("https://alist.example.com")?;
//!
//! // 2. 配置凭据：用户名密码（401/403 时自动重新登录并重试一次）
//! client.set_authentication(Authentication::username_password("admin", "password", None));
//! // 或直接使用已有 token：client.set_authentication(Authentication::token("..."));
//!
//! // 3. 可选：启用客户端侧请求限速
//! client.set_api_request_interval(std::time::Duration::from_millis(250));
//! # Ok(())
//! # }
//! ```
//!
//! # 架构总览
//!
//! - `client`（私有）：[`Client`] 核心与 [`Authentication`] 凭据；
//! - [`endpoint`]：按功能域划分的无状态句柄（如 `client.fs().list()`），feature 门控；
//! - [`schema`]：API 数据模型，与端点 feature 一一对应（`X` → `X-schema`）；
//! - [`error`]：[`Error`]/[`Result`] 与 AList 状态码、内部错误分类。
//!
//! # 功能矩阵
//!
//! 默认启用全部功能（`default = ["all"]`）。可按需裁剪：
//! `auth`、`fs`、`public`、`admin`（聚合全部 admin 子域）、
//! `into-stream`（自动翻页流）与 `stream`（响应流式读取）。

pub(crate) mod client;
pub mod endpoint;
pub mod error;
pub mod schema;

#[cfg(test)]
pub(crate) mod test_support;

pub use client::{Authentication, Client};
pub use error::{ApiStatusCode, Error, InternalErrorKind, Result};
