//! Data models for `/api/admin/label_file_binding/*`.

use crate::models::admin::label::Label;
use serde::{Deserialize, Serialize};

/// Request body for `/api/admin/label_file_binding/create`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingCreateReq {
    /// Comma-separated label ids.
    pub label_ids: String,
    /// File name.
    pub name: String,
    /// File id.
    pub id: String,
    /// File path.
    pub path: String,
    /// File size.
    pub size: i64,
    /// Whether the file is a directory.
    pub is_dir: bool,
    /// Modified timestamp.
    pub modified: String,
    /// Created timestamp.
    pub created: String,
    /// File sign.
    pub sign: String,
    /// Thumbnail value.
    pub thumb: String,
    /// AList object type.
    #[serde(rename = "type")]
    pub obj_type: i32,
    /// Hash info string.
    pub hashinfo: String,
}

/// Response payload for `/api/admin/label_file_binding/create`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingCreateResp {
    /// Result message.
    pub msg: String,
}

/// Request body for `/api/admin/label_file_binding/delete`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingDeleteReq {
    /// Label id.
    pub label_id: String,
    /// File name.
    pub file_name: String,
}

/// File returned by `/api/admin/label_file_binding/get_file_by_label`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabeledFile {
    /// File id.
    pub id: String,
    /// File path.
    pub path: String,
    /// File name.
    pub name: String,
    /// File size.
    pub size: i64,
    /// Whether the file is a directory.
    pub is_dir: bool,
    /// Modified timestamp.
    pub modified: String,
    /// Created timestamp.
    pub created: String,
    /// File sign.
    pub sign: String,
    /// Thumbnail value.
    pub thumb: String,
    /// AList object type.
    #[serde(rename = "type")]
    pub obj_type: i32,
    /// Hash info string.
    pub hashinfo: String,
    /// Labels attached to this file.
    pub label_list: Vec<Label>,
}

/// Request body for `/api/admin/label_file_binding/create_batch`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingBatchReq {
    /// Batch binding items.
    pub items: Vec<LabelFileBindingBatchItem>,
}

/// One batch binding item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingBatchItem {
    /// File path.
    pub path: String,
    /// File name.
    pub name: String,
    /// Whether this item is a directory.
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    /// Label ids.
    #[serde(rename = "labelIdList")]
    pub label_id_list: Vec<u64>,
    /// File size.
    pub size: i64,
    /// AList object type.
    #[serde(rename = "type")]
    pub obj_type: i32,
    /// Modified timestamp.
    pub modified: String,
    /// Created timestamp.
    pub created: String,
    /// File sign.
    pub sign: String,
    /// Thumbnail value.
    pub thumb: String,
    /// Hash info string.
    #[serde(rename = "hashInfoStr")]
    pub hash_info_str: String,
}

/// Batch binding response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingBatchResp {
    /// Failed count.
    pub failed: i64,
    /// Per-file results.
    pub results: Vec<LabelFileBindingBatchResult>,
    /// Succeeded count.
    pub succeed: i64,
    /// Total count.
    pub total: i64,
}

/// One batch binding result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBindingBatchResult {
    /// File name.
    pub name: String,
    /// Whether this binding succeeded.
    pub ok: bool,
}

/// Query parameters containing a file name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileNameQuery {
    /// File name.
    pub file_name: String,
}

/// Query parameters containing a label id string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelIdQuery {
    /// Label ids, often comma-separated.
    pub label_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;

    #[test]
    fn openapi_admin_label_file_binding_examples_match_models() {
        let create = LabelFileBindingCreateReq {
            label_ids: "string".to_string(),
            name: "string".to_string(),
            id: "string".to_string(),
            path: "string".to_string(),
            size: 0,
            is_dir: true,
            modified: "string".to_string(),
            created: "string".to_string(),
            sign: "string".to_string(),
            thumb: "string".to_string(),
            obj_type: 0,
            hashinfo: "string".to_string(),
        };
        assert_eq!(serde_json::to_value(&create).unwrap()["type"], 0);

        let labels: ApiResponse<Vec<Label>> = serde_json::from_value(serde_json::json!({
            "code": 0,
            "message": "string",
            "data": [{
                "id": 0,
                "type": 0,
                "name": "string",
                "create_time": "string"
            }]
        }))
        .unwrap();
        assert_eq!(labels.data[0].name, "string");

        let batch: ApiResponse<LabelFileBindingBatchResp> =
            serde_json::from_value(serde_json::json!({
                "code": 200,
                "message": "success",
                "data": {
                    "failed": 0,
                    "results": [{ "name": "20221226_210943.jpg", "ok": true }],
                    "succeed": 1,
                    "total": 1
                }
            }))
            .unwrap();
        assert!(batch.data.results[0].ok);
    }
}
