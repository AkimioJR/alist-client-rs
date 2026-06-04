//! Serde data models grouped by AList API tags.
//!
//! The file layout follows AList API paths such as `auth`, `fs`, and `public`.
//! Nested endpoints keep their models under the matching parent path.

pub mod auth;
pub mod common;
pub mod fs;
pub mod public;
