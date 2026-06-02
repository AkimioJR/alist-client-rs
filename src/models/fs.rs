//! Data models for the `fs` OpenAPI group, excluding archive-specific payloads.

use crate::models::common::TaskInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Object metadata returned by AList fs endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjResp {
    /// Object id. Present in current `/fs/list`, absent in older mounted-client responses.
    #[serde(default)]
    pub id: Option<String>,
    /// Full server path. Present in current `/fs/list`.
    #[serde(default)]
    pub path: Option<String>,
    /// Virtual path. Present in current server responses when applicable.
    #[serde(default)]
    pub virtual_path: Option<String>,
    /// File or directory name.
    pub name: String,
    /// Size in bytes.
    pub size: i64,
    /// Whether this object is a directory.
    pub is_dir: bool,
    /// Last modified timestamp.
    pub modified: DateTime<Utc>,
    /// Created timestamp.
    pub created: DateTime<Utc>,
    /// Download/signature token. Empty for archive tree responses.
    pub sign: String,
    /// Thumbnail URL or token.
    pub thumb: String,
    /// AList object type code.
    #[serde(rename = "type")]
    pub obj_type: i32,
    /// Compact hash string used by the Go model.
    pub hashinfo: String,
    /// Structured hash data. OpenAPI examples show an object or null; current Go
    /// code uses pointer-like map keys internally, so keep the raw JSON value.
    #[serde(default)]
    pub hash_info: Option<Value>,
    /// Storage class appears on newer server builds.
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// Label metadata attached to a file in current `/fs/list` responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelResp {
    /// Label id.
    pub id: u64,
    /// Label type from AList.
    #[serde(default)]
    pub r#type: Option<i32>,
    /// Display name.
    pub name: String,
    /// Optional label color.
    #[serde(default)]
    pub color: Option<String>,
}

/// Object metadata plus optional labels returned by `/api/fs/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjLabelResp {
    /// Shared object fields.
    #[serde(flatten)]
    pub object: ObjResp,
    /// Current API may return `null` when no labels are attached.
    #[serde(default)]
    pub label_list: Option<Vec<LabelResp>>,
}

/// Request body for `/api/fs/list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FsListReq {
    /// Directory path to list.
    pub path: String,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
    /// Force refresh the storage cache.
    pub refresh: bool,
    /// One-based page number.
    pub page: i32,
    /// Page size. Current server accepts `0` for all items.
    pub per_page: i32,
}

impl FsListReq {
    /// Build an all-items list request with no meta password and no refresh.
    pub fn all(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            password: String::new(),
            refresh: false,
            page: 1,
            per_page: 0,
        }
    }
}

/// Response body for `/api/fs/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsListResp {
    /// Listed objects.
    pub content: Vec<ObjLabelResp>,
    /// Total visible rows.
    pub total: i64,
    /// Filtered total in current server versions.
    pub filtered_total: i64,
    /// Effective page returned by the server.
    pub page: i32,
    /// Effective page size returned by the server.
    pub per_page: i32,
    /// Whether more pages are available.
    pub has_more: bool,
    /// Total pages.
    pub pages_total: i32,
    /// Directory readme content.
    pub readme: String,
    /// Directory header content.
    pub header: String,
    /// Whether the current user can write to this directory.
    pub write: bool,
    /// Backing storage provider name.
    pub provider: String,
}

/// Request body for `/api/fs/get`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FsGetReq {
    /// Object path.
    pub path: String,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
    /// Some docs/examples include pagination fields for related items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Some docs/examples include pagination fields for related items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    /// Force refresh object metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
}

/// Response body for `/api/fs/get`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsGetResp {
    /// Shared object fields.
    #[serde(flatten)]
    pub object: ObjResp,
    /// Direct raw URL generated by the backing storage.
    pub raw_url: String,
    /// Readme inherited from meta.
    pub readme: String,
    /// Optional header field added by current server code.
    #[serde(default)]
    pub header: Option<String>,
    /// Backing storage provider name.
    pub provider: String,
    /// Related objects. OpenAPI documents `null`; older driver code expects an array.
    #[serde(default)]
    pub related: Option<Vec<ObjResp>>,
}

/// Request body for `/api/fs/dirs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirsReq {
    /// Parent directory path.
    pub path: String,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
    /// Admin-only flag to bypass the current user's base path.
    pub force_root: bool,
}

/// Directory entry returned by `/api/fs/dirs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirResp {
    /// Directory name.
    pub name: String,
    /// Last modified timestamp.
    pub modified: DateTime<Utc>,
}

/// Request body for `/api/fs/mkdir`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MkdirReq {
    /// New directory path.
    pub path: String,
}

/// Request body for `/api/fs/move` and `/api/fs/copy`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveCopyReq {
    /// Source directory.
    pub src_dir: String,
    /// Destination directory.
    pub dst_dir: String,
    /// File or directory names relative to `src_dir`.
    pub names: Vec<String>,
    /// Current server supports overwrite though the OpenAPI examples omit it.
    #[serde(default)]
    pub overwrite: bool,
}

/// Request body for `/api/fs/rename`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameReq {
    /// Source path.
    pub path: String,
    /// New name. Must not contain `/`.
    pub name: String,
    /// Current server supports overwrite though the OpenAPI examples omit it.
    #[serde(default)]
    pub overwrite: bool,
}

/// Request body for `/api/fs/remove`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveReq {
    /// Parent directory.
    pub dir: String,
    /// File or directory names relative to `dir`.
    pub names: Vec<String>,
}

/// Request body for `/api/fs/remove_empty_directory`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveEmptyDirectoryReq {
    /// Root directory to scan for empty directories.
    pub src_dir: String,
}

/// Request body for `/api/fs/search`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchReq {
    /// Parent path to search under.
    pub parent: String,
    /// Search keywords.
    pub keywords: String,
    /// Search scope: `0` all, `1` directories, `2` files.
    pub scope: i32,
    /// One-based page number.
    pub page: i32,
    /// Page size.
    pub per_page: i32,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
}

/// Search result item returned by `/api/fs/search`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResp {
    /// Parent path containing the result.
    pub parent: String,
    /// File or directory name.
    pub name: String,
    /// Whether this result is a directory.
    pub is_dir: bool,
    /// Size in bytes.
    pub size: i64,
    /// AList object type code.
    #[serde(rename = "type")]
    pub obj_type: i32,
}

/// Response payload used by copy/offline APIs when they create tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TasksResp {
    /// Created tasks.
    pub tasks: Vec<TaskInfo>,
}
