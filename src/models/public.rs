//! Public unauthenticated API models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Site settings returned by `/api/public/settings`.
///
/// The public settings map is intentionally open-ended: AList exposes many
/// string-valued feature flags and site customization values, and the exact key
/// set changes across releases.
pub type PublicSettings = HashMap<String, String>;

/// Known public setting keys documented by the OpenAPI file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownPublicSettings {
    /// Whether indexing is allowed.
    #[serde(default)]
    pub allow_indexed: Option<String>,
    /// Whether remote mounting is allowed.
    #[serde(default)]
    pub allow_mounted: Option<String>,
    /// Site announcement.
    #[serde(default)]
    pub announcement: Option<String>,
    /// Default page size as a string setting.
    #[serde(default)]
    pub default_page_size: Option<String>,
    /// Favicon URL.
    #[serde(default)]
    pub favicon: Option<String>,
    /// Site title.
    #[serde(default)]
    pub site_title: Option<String>,
}
