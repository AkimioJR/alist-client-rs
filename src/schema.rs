//! AList API 数据模型模块。
//!
//! 模块布局与 AList API 分组一一对应：[`common`] 为无条件编译的共享模型，
//! 其余子模块由对应的 `*-schema` feature 门控（端点 feature 会自动拉取镜像 schema feature，
//! 见根 `Cargo.toml` 的 `[features]` 与 `docs/design.md`）。

pub mod common;

#[cfg(feature = "auth-schema")]
pub mod auth;
#[cfg(not(feature = "auth-schema"))]
pub(crate) mod auth;

#[cfg(feature = "fs-schema")]
pub mod fs;

#[cfg(feature = "public-schema")]
pub mod public;

#[cfg(any(
    feature = "admin-meta-schema",
    feature = "admin-user-schema",
    feature = "admin-storage-schema",
    feature = "admin-driver-schema",
    feature = "admin-setting-schema",
    feature = "admin-task-schema",
    feature = "admin-role-schema",
    feature = "admin-label-schema",
    feature = "admin-label-file-binding-schema"
))]
pub mod admin;
