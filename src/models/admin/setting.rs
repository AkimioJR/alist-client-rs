//! Data models for `/api/admin/setting/*`.

use serde::{Deserialize, Serialize};

/// Setting item returned and saved by admin setting endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
    /// Setting key.
    pub key: String,
    /// Setting value.
    pub value: String,
    /// Help text.
    pub help: String,
    /// Value type.
    #[serde(rename = "type")]
    pub value_type: String,
    /// Option list encoded as a string.
    pub options: String,
    /// Setting group id.
    pub group: i32,
    /// Setting visibility flag.
    pub flag: i32,
}

/// Query parameters for `/api/admin/setting/list`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SettingListQuery {
    /// Multiple groups such as `5,0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<String>,
    /// Single group id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// Query parameters for `/api/admin/setting/get`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SettingGetQuery {
    /// Multiple setting keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<String>,
    /// Single setting key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Query parameters for `/api/admin/setting/delete`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingKeyQuery {
    /// Setting key.
    pub key: String,
}

/// Request body for `/api/admin/setting/set_aria2`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetAria2Req {
    /// aria2 JSON-RPC URI.
    pub uri: String,
    /// aria2 secret.
    pub secret: String,
}

/// Request body for `/api/admin/setting/set_qbit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetQbitReq {
    /// qBittorrent URL.
    pub url: String,
    /// Seed time value.
    pub seedtime: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;

    fn setting_json() -> serde_json::Value {
        serde_json::json!({
            "key": "aria2_uri",
            "value": "http://localhost:6800/jsonrpc",
            "help": "",
            "type": "string",
            "options": "",
            "group": 5,
            "flag": 1
        })
    }

    #[test]
    fn openapi_admin_setting_examples_match_models() {
        let list: ApiResponse<Vec<Setting>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [setting_json()]
        }))
        .unwrap();
        assert_eq!(list.data[0].key, "aria2_uri");
        assert_eq!(list.data[0].value_type, "string");

        let one: ApiResponse<Setting> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": setting_json()
        }))
        .unwrap();
        assert_eq!(one.data.group, 5);

        let aria2 = SetAria2Req {
            uri: "string".to_string(),
            secret: "string".to_string(),
        };
        assert_eq!(
            serde_json::to_value(&aria2).unwrap(),
            serde_json::json!({ "uri": "string", "secret": "string" })
        );

        let qbit = SetQbitReq {
            url: "string".to_string(),
            seedtime: "string".to_string(),
        };
        assert_eq!(
            serde_json::to_value(&qbit).unwrap(),
            serde_json::json!({ "url": "string", "seedtime": "string" })
        );
    }
}
