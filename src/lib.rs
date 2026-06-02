//! Async Rust client for the AList v3 API.
//!
//! This crate models AList's JSON envelopes and core file-system endpoints.
//! API data models are grouped by OpenAPI tag.

pub mod error;
pub mod models;

pub use error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
pub use models::*;
