//! Data models for `/api/admin/storage/*`.

use serde::{Deserialize, Serialize};

/// Storage payload returned by admin storage endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storage {
    /// Storage id.
    pub id: u64,
    /// Mount path.
    pub mount_path: String,
    /// Sort order.
    pub order: i32,
    /// Driver name.
    pub driver: String,
    /// Cache expiration in seconds.
    pub cache_expiration: i32,
    /// Storage status.
    pub status: String,
    /// Driver-specific addition JSON encoded as a string.
    pub addition: String,
    /// Remark name.
    pub remark: String,
    /// Last modified timestamp as returned by AList.
    pub modified: String,
    /// Whether this storage is disabled.
    pub disabled: bool,
    /// Whether signing is enabled.
    #[serde(default)]
    pub enable_sign: bool,
    /// Object ordering field.
    pub order_by: String,
    /// Object ordering direction.
    pub order_direction: String,
    /// Extract folder policy.
    pub extract_folder: String,
    /// Whether web proxy is enabled.
    pub web_proxy: bool,
    /// WebDAV policy.
    pub webdav_policy: String,
    /// Download proxy URL.
    pub down_proxy_url: String,
}

/// Request body for `/api/admin/storage/create` and `/api/admin/storage/update`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageReq {
    /// Storage id for updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Mount path.
    pub mount_path: String,
    /// Sort order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// Driver name.
    pub driver: String,
    /// Remark name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// Cache expiration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_expiration: Option<i32>,
    /// Storage status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Whether web proxy is enabled.
    pub web_proxy: bool,
    /// WebDAV policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webdav_policy: Option<String>,
    /// Download proxy URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub down_proxy_url: Option<String>,
    /// Object ordering field.
    pub order_by: String,
    /// Extract folder policy.
    pub extract_folder: String,
    /// Object ordering direction.
    pub order_direction: String,
    /// Driver-specific addition JSON encoded as a string.
    pub addition: String,
    /// Whether signing is enabled.
    pub enable_sign: bool,
}

/// Response payload for storage create and update endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCreateResp {
    /// Created or updated storage id.
    pub id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::{ApiResponse, PageResp};

    fn storage_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "mount_path": "/lll",
            "order": 0,
            "driver": "Local",
            "cache_expiration": 0,
            "status": "work",
            "addition": "{\"root_folder_path\":\"/root/www\"}",
            "remark": "",
            "modified": "2023-07-19T09:46:38.868739912+08:00",
            "disabled": false,
            "enable_sign": false,
            "order_by": "name",
            "order_direction": "asc",
            "extract_folder": "front",
            "web_proxy": false,
            "webdav_policy": "native_proxy",
            "down_proxy_url": ""
        })
    }

    #[test]
    fn openapi_admin_storage_examples_match_models() {
        let storage: Storage = serde_json::from_value(storage_json()).unwrap();
        assert_eq!(storage.mount_path, "/lll");
        assert_eq!(storage.driver, "Local");

        let list: ApiResponse<PageResp<Storage>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "content": [storage_json()],
                "total": 5
            }
        }))
        .unwrap();
        assert_eq!(list.data.total, 5);

        let req = StorageReq {
            id: None,
            mount_path: "/lll".to_string(),
            order: Some(0),
            driver: "Local".to_string(),
            remark: Some(String::new()),
            cache_expiration: Some(30),
            status: None,
            web_proxy: false,
            webdav_policy: Some("native_proxy".to_string()),
            down_proxy_url: Some(String::new()),
            order_by: "name".to_string(),
            extract_folder: "front".to_string(),
            order_direction: "asc".to_string(),
            addition: "{\"root_folder_path\":\"/\"}".to_string(),
            enable_sign: false,
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "mount_path": "/lll",
                "order": 0,
                "driver": "Local",
                "remark": "",
                "cache_expiration": 30,
                "web_proxy": false,
                "webdav_policy": "native_proxy",
                "down_proxy_url": "",
                "order_by": "name",
                "extract_folder": "front",
                "order_direction": "asc",
                "addition": "{\"root_folder_path\":\"/\"}",
                "enable_sign": false
            })
        );

        let created: ApiResponse<StorageCreateResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": { "id": 7 }
        }))
        .unwrap();
        assert_eq!(created.data.id, 7);
    }
}
