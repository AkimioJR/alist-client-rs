//! Data models for `/api/admin/driver/*`.

use serde_json::Value;
use std::collections::HashMap;

/// Driver template map keyed by driver name.
pub type DriverListResp = HashMap<String, Value>;

/// Driver names returned by `/api/admin/driver/names`.
pub type DriverNamesResp = Vec<String>;

/// Driver template returned by `/api/admin/driver/info`.
pub type DriverInfoResp = Value;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;

    #[test]
    fn openapi_admin_driver_examples_match_models() {
        let names: ApiResponse<DriverNamesResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": ["UC", "Local"]
        }))
        .unwrap();
        assert_eq!(names.data, vec!["UC", "Local"]);

        let info_json = serde_json::json!({
            "common": [{
                "name": "mount_path",
                "type": "string",
                "default": "",
                "options": "",
                "required": true,
                "help": "The path you want to mount to"
            }],
            "additional": [],
            "config": {
                "name": "UC",
                "local_sort": false,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        });
        let info: ApiResponse<DriverInfoResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": info_json
        }))
        .unwrap();
        assert_eq!(info.data["config"]["name"], "UC");

        let list: ApiResponse<DriverListResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": { "UC": info.data }
        }))
        .unwrap();
        assert!(list.data.contains_key("UC"));
    }
}
