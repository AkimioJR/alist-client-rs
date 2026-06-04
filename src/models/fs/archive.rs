//! Archive payloads for `/api/fs/archive/*` endpoints.
//!
//! OpenAPI groups these endpoints under `fs`, but the server implements them in
//! `handles/archive.go`; keeping these models separate makes the recursive tree
//! response easier to browse.

use super::ObjResp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request body for `/api/fs/archive/meta`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveMetaReq {
    /// Archive file path.
    pub path: String,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
    /// Refresh cached archive metadata.
    pub refresh: bool,
    /// Password for encrypted archives.
    pub archive_pass: String,
}

/// Recursive archive tree entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveContentResp {
    /// Shared object fields. Archive entries intentionally have empty sign/thumb.
    #[serde(flatten)]
    pub object: ObjResp,
    /// Child entries when this archive item is a directory.
    #[serde(default)]
    pub children: Vec<ArchiveContentResp>,
}

/// Response body for `/api/fs/archive/meta`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveMetaResp {
    /// Archive comment.
    pub comment: String,
    /// Whether the archive is encrypted.
    pub encrypted: bool,
    /// Archive root tree.
    pub content: Vec<ArchiveContentResp>,
    /// Sort metadata added by newer server versions.
    #[serde(default)]
    pub sort: Option<Value>,
    /// Download/extract URL. Driver-forwarded responses may omit this.
    #[serde(default)]
    pub raw_url: Option<String>,
    /// Signature for raw archive access.
    #[serde(default)]
    pub sign: Option<String>,
}

/// Request body for `/api/fs/archive/list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveListReq {
    /// Archive file path.
    pub path: String,
    /// Meta password. Use an empty string when no password is needed.
    pub password: String,
    /// Refresh cached archive metadata.
    pub refresh: bool,
    /// Password for encrypted archives.
    pub archive_pass: String,
    /// One-based page number.
    pub page: i32,
    /// Page size. AList accepts `0` for all items.
    pub per_page: i32,
    /// Path inside the archive.
    pub inner_path: String,
}

/// Response body for `/api/fs/archive/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveListResp {
    /// Archive entries in the requested inner path.
    pub content: Vec<ObjResp>,
    /// Total entries.
    pub total: i64,
}

/// Request body for `/api/fs/archive/decompress`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveDecompressReq {
    /// Source directory containing the archive.
    pub src_dir: String,
    /// Destination directory for extracted files.
    pub dst_dir: String,
    /// Archive file names. The Go server accepts a string or array; this client
    /// sends the array form for deterministic JSON.
    pub name: Vec<String>,
    /// Password for encrypted archives.
    pub archive_pass: String,
    /// Path inside the archive to extract.
    pub inner_path: String,
    /// Whether to cache the full archive before extracting.
    pub cache_full: bool,
    /// Whether to put extracted files into a new directory.
    pub put_into_new_dir: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;
    use crate::models::fs::tests::object_with;
    use serde_json::{Map, Value};

    #[test]
    fn archive_meta_resp_deserializes_recursive_tree() {
        let child = object_with(Map::from_iter([(
            "children".to_string(),
            Value::Array(Vec::new()),
        )]));
        let root = object_with(Map::from_iter([(
            "children".to_string(),
            Value::Array(vec![child]),
        )]));
        let resp: ApiResponse<ArchiveMetaResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "comment": "archive comment",
                "encrypted": false,
                "content": [root],
                "sort": { "order_by": "name" },
                "raw_url": "https://example.test/ad/demo.zip",
                "sign": "archive-sign"
            }
        }))
        .unwrap();

        assert_eq!(resp.data.comment, "archive comment");
        assert_eq!(resp.data.content[0].children.len(), 1);
        assert_eq!(resp.data.sign.as_deref(), Some("archive-sign"));
    }
}
