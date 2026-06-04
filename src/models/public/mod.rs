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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;

    #[test]
    fn openapi_public_settings_example_matches_models() {
        let resp: ApiResponse<PublicSettings> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "allow_indexed": "false",
                "allow_mounted": "false",
                "announcement": "",
                "audio_autoplay": "true",
                "audio_cover": "https://jsd.nn.ci/gh/alist-org/logo@main/logo.svg",
                "auto_update_index": "false",
                "default_page_size": "30",
                "external_previews": "{}",
                "favicon": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
                "filename_char_mapping": "{\"/\": \"|\"}",
                "forward_direct_link_params": "false",
                "hide_files": "/\\/README.md/i",
                "home_container": "hope_container",
                "home_icon": "🏠",
                "iframe_previews": "{\n\t\"doc,docx,xls,xlsx,ppt,pptx\": {\n\t\t\"Microsoft\":\"https://view.officeapps.live.com/op/view.aspx?src=$e_url\",\n\t\t\"Google\":\"https://docs.google.com/gview?url=$e_url&embedded=true\"\n\t},\n\t\"pdf\": {\n\t\t\"PDF.js\":\"https://alist-org.github.io/pdf.js/web/viewer.html?file=$e_url\"\n\t},\n\t\"epub\": {\n\t\t\"EPUB.js\":\"https://alist-org.github.io/static/epub.js/viewer.html?url=$e_url\"\n\t}\n}",
                "logo": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
                "main_color": "#1890ff",
                "ocr_api": "https://api.nn.ci/ocr/file/json",
                "package_download": "true",
                "pagination_type": "all",
                "robots_txt": "User-agent: *\nAllow: /",
                "search_index": "none",
                "settings_layout": "responsive",
                "site_title": "AList",
                "sso_login_enabled": "false",
                "sso_login_platform": "",
                "version": "v3.25.1",
                "video_autoplay": "true"
            }
        }))
        .unwrap();

        assert_eq!(resp.data["allow_mounted"], "false");
        assert_eq!(resp.data["default_page_size"], "30");
        assert_eq!(resp.data["site_title"], "AList");

        let known: KnownPublicSettings =
            serde_json::from_value(serde_json::to_value(&resp.data).unwrap()).unwrap();
        assert_eq!(known.allow_indexed.as_deref(), Some("false"));
        assert_eq!(
            known.favicon.as_deref(),
            Some("https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg")
        );
        assert_eq!(known.site_title.as_deref(), Some("AList"));
    }
}
