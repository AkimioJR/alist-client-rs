//! Data models for `/api/admin/label/*`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Label returned by admin label endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// Label id.
    pub id: u64,
    /// Reserved label type.
    #[serde(rename = "type")]
    pub label_type: i32,
    /// Label name.
    pub name: String,
    /// Label description.
    #[serde(default)]
    pub description: Option<String>,
    /// Label background color.
    #[serde(default)]
    pub bg_color: Option<String>,
    /// Creation time.
    #[serde(default)]
    pub create_time: Option<String>,
}

/// Request body for `/api/admin/label/create`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelCreateReq {
    /// Label name.
    pub name: String,
    /// Reserved label type.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub label_type: Option<Value>,
    /// Label description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label background color.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,
}

/// Request body for `/api/admin/label/update`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelUpdateReq {
    /// Label id.
    pub id: u64,
    /// Label name.
    pub name: String,
    /// Reserved label type.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub label_type: Option<Value>,
    /// Label description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Label background color.
    pub bg_color: String,
}

/// Request body for `/api/admin/label/delete`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelDeleteReq {
    /// Label id encoded as documented by OpenAPI.
    pub id: String,
}

/// Response payload containing a created label id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelIdResp {
    /// Label id.
    pub id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::{ApiResponse, PageResp};

    fn label_json() -> serde_json::Value {
        serde_json::json!({
            "id": 0,
            "type": 0,
            "name": "string",
            "description": "string",
            "bg_color": "string",
            "create_time": "string"
        })
    }

    #[test]
    fn openapi_admin_label_examples_match_models() {
        let list: ApiResponse<PageResp<Label>> = serde_json::from_value(serde_json::json!({
            "code": 0,
            "message": "string",
            "data": {
                "content": [label_json()],
                "total": 0
            }
        }))
        .unwrap();
        assert_eq!(list.data.content[0].name, "string");

        let create = LabelCreateReq {
            name: "string".to_string(),
            label_type: Some(serde_json::json!(0)),
            description: Some("string".to_string()),
            bg_color: Some("string".to_string()),
        };
        assert_eq!(
            serde_json::to_value(&create).unwrap(),
            serde_json::json!({
                "name": "string",
                "type": 0,
                "description": "string",
                "bg_color": "string"
            })
        );

        let id: ApiResponse<LabelIdResp> = serde_json::from_value(serde_json::json!({
            "code": 0,
            "message": "string",
            "data": { "id": 0 }
        }))
        .unwrap();
        assert_eq!(id.data.id, 0);
    }
}
