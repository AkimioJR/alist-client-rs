//! Async Rust client for the AList v3 API.
//!
//! This crate models AList's JSON envelopes and core file-system endpoints.
//! API data models are grouped by OpenAPI tag, while high-level async methods
//! live on [`Client`].

pub mod client;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests;

pub use client::{Authentication, Client};
pub use error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
