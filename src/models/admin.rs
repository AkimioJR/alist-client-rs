//! Data models for the `admin` OpenAPI groups.

pub mod driver;
pub mod meta;
pub mod setting;
pub mod storage;
pub mod user;

use serde::{Deserialize, Serialize};

/// Query parameters containing a numeric id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdQuery {
    /// Resource id.
    pub id: u64,
}

/// Query parameters containing a username.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsernameQuery {
    /// Username value.
    pub username: String,
}

/// Query parameters containing a driver name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriverQuery {
    /// Driver name.
    pub driver: String,
}

/// Optional pagination query parameters used by admin list endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AdminPageQuery {
    /// One-based page number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Page size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

impl AdminPageQuery {
    /// Create pagination query parameters.
    pub fn new(page: i32, per_page: i32) -> Self {
        Self {
            page: Some(page),
            per_page: Some(per_page),
        }
    }
}
