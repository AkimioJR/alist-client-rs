//! Data models for `/api/admin/user/*`.

use serde::{Deserialize, Serialize};

/// User payload returned by admin user endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminUser {
    /// User id.
    pub id: u64,
    /// Username.
    pub username: String,
    /// Password field. List/get responses usually return an empty string.
    pub password: String,
    /// Base path for this user.
    pub base_path: String,
    /// Legacy role id.
    pub role: i32,
    /// Whether the user is disabled.
    pub disabled: bool,
    /// Legacy permission bitmask.
    pub permission: i32,
    /// SSO id.
    pub sso_id: String,
}

/// Request body for `/api/admin/user/create` and `/api/admin/user/update`.
pub type AdminUserReq = AdminUser;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::{ApiResponse, PageResp};

    fn user_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "username": "admin",
            "password": "",
            "base_path": "/",
            "role": 2,
            "disabled": false,
            "permission": 0,
            "sso_id": ""
        })
    }

    #[test]
    fn openapi_admin_user_examples_match_models() {
        let user: AdminUser = serde_json::from_value(user_json()).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "admin");

        let list: ApiResponse<PageResp<AdminUser>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "content": [user_json()],
                "total": 1
            }
        }))
        .unwrap();
        assert_eq!(list.data.content[0].role, 2);

        let req = AdminUserReq {
            id: 0,
            username: "a".to_string(),
            password: "123456".to_string(),
            base_path: "/".to_string(),
            role: 0,
            permission: 60,
            disabled: false,
            sso_id: String::new(),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "id": 0,
                "username": "a",
                "password": "123456",
                "base_path": "/",
                "role": 0,
                "disabled": false,
                "permission": 60,
                "sso_id": ""
            })
        );
    }
}
