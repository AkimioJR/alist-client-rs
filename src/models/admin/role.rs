//! Data models for `/api/admin/role/*`.

use serde::{Deserialize, Serialize};

/// Permission bitmask scoped to a path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolePermissionEntry {
    /// Path prefix.
    pub path: String,
    /// Permission bitmask.
    pub permission: i32,
}

/// Role payload used by admin role endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    /// Role id.
    pub id: u64,
    /// Role name.
    pub name: String,
    /// Role description.
    pub description: String,
    /// Path-scoped permissions.
    pub permission_scopes: Vec<RolePermissionEntry>,
    /// Raw permission JSON string accepted by some server versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_permission: Option<String>,
}

/// Request body for `/api/admin/role/create` and `/api/admin/role/update`.
pub type RoleReq = Role;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::{ApiResponse, PageResp};

    fn role_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "name": "guest",
            "description": "Guest",
            "permission_scopes": [{ "path": "/", "permission": 0 }]
        })
    }

    #[test]
    fn openapi_admin_role_examples_match_models() {
        let list: ApiResponse<PageResp<Role>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "content": [role_json()],
                "total": 1
            }
        }))
        .unwrap();
        assert_eq!(list.data.content[0].name, "guest");

        let req = RoleReq {
            id: 1,
            name: "admin".to_string(),
            description: "Administrator role".to_string(),
            permission_scopes: vec![RolePermissionEntry {
                path: "/admin".to_string(),
                permission: 7,
            }],
            raw_permission: Some(r#"[{"path":"/admin","permission":7}]"#.to_string()),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "id": 1,
                "name": "admin",
                "description": "Administrator role",
                "permission_scopes": [{ "path": "/admin", "permission": 7 }],
                "raw_permission": "[{\"path\":\"/admin\",\"permission\":7}]"
            })
        );
    }
}
