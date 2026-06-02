//! Serde data models grouped by AList API tags.
//!
//! The file layout follows the OpenAPI groups used by `alist-api`: auth, fs,
//! and public. Archive endpoints live under the fs API group in OpenAPI, but
//! are split into their own module because the server implements them in a
//! dedicated archive handler and their payloads are tree-shaped.

pub mod archive;
pub mod auth;
pub mod common;
pub mod fs;
pub mod public;

pub use archive::*;
pub use auth::*;
pub use common::*;
pub use fs::*;
pub use public::*;
