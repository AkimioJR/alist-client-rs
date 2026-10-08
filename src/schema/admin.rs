//! admin 管理域数据模型。
//!
//! 子模块与 `admin/*-schema` feature 一一对应：
//! 端点 feature（如 `admin-meta`）会自动拉取镜像的 `admin-meta-schema`。
//! 模块布局与 AList 服务端路由的 admin 分组保持一致。

#[cfg(feature = "admin-meta-schema")]
pub mod meta;

#[cfg(feature = "admin-user-schema")]
pub mod user;

#[cfg(feature = "admin-storage-schema")]
pub mod storage;

#[cfg(feature = "admin-driver-schema")]
pub mod driver;

#[cfg(feature = "admin-setting-schema")]
pub mod setting;

#[cfg(feature = "admin-task-schema")]
pub mod task;

#[cfg(feature = "admin-role-schema")]
pub mod role;

#[cfg(feature = "admin-label-schema")]
pub mod label;

#[cfg(feature = "admin-label-file-binding-schema")]
pub mod label_file_binding;
