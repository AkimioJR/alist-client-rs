//! AList API 数据模型模块。
//!
//! 模块布局与 AList API 分组一一对应：[`common`] 为无条件编译的共享模型，
//! 其余子模块由对应的 `*-schema` feature 门控（端点 feature 会自动拉取镜像 schema feature，
//! 见根 `Cargo.toml` 的 `[features]` 与项目设计文档）。
//!
//! ## 跨版本数据契约规范
//!
//! - **标量演进字段严谨建模**：对于随 AList 服务端版本演进新增或调整的标量字段
//!   （如分页元信息、对象物理路径与虚拟路径、索引开关、响应头等），统一使用 [`Option<T>`]
//!   建模，严格区分「老版本服务端未提供（[`None`]）」与「服务端提供且为有效零值（如 `Some(0)`、
//!   `Some(false)`）」，杜绝隐式默认零值造成的逻辑误判。
//! - **Git Tag 精确版本溯源**：所有演进字段的 rustdoc 中均明确记录了首次引入该特性的
//!   AList Git Tag（如 `v3.61.0`、`v3.58.0` 等）及演进背景。
//! - **集合类容器的人体工程学维持**：对于 [`Vec`] 与 [`HashMap`](std::collections::HashMap)
//!   等容器字段，继续采用 `null_to_default` 映射为空集合，兼顾日常遍历的便利性。

pub mod common;

#[cfg(feature = "auth-schema")]
pub mod auth;
#[cfg(not(feature = "auth-schema"))]
pub(crate) mod auth;

#[cfg(feature = "fs-schema")]
pub mod fs;

#[cfg(feature = "public-schema")]
pub mod public;

#[cfg(feature = "task-schema")]
pub mod task;

#[cfg(any(
    feature = "admin-meta-schema",
    feature = "admin-user-schema",
    feature = "admin-storage-schema",
    feature = "admin-driver-schema",
    feature = "admin-setting-schema",
    feature = "admin-role-schema",
    feature = "admin-label-schema",
    feature = "admin-label-file-binding-schema"
))]
pub mod admin;
