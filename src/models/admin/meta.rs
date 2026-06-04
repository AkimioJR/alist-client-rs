//! Data models for `/api/admin/meta/*`.

use serde::{Deserialize, Serialize};

/// Metadata rule used by admin meta endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Meta {
    /// Metadata id.
    pub id: u64,
    /// Path matched by this metadata rule.
    pub path: String,
    /// Meta password.
    pub password: String,
    /// Whether the password applies to child folders.
    pub p_sub: bool,
    /// Whether writes are allowed.
    pub write: bool,
    /// Whether write permission applies to child folders.
    pub w_sub: bool,
    /// Hidden file pattern.
    pub hide: String,
    /// Whether hidden pattern applies to child folders.
    pub h_sub: bool,
    /// Readme content.
    pub readme: String,
    /// Whether readme content applies to child folders.
    pub r_sub: bool,
}

/// Request body for `/api/admin/meta/create` and `/api/admin/meta/update`.
pub type MetaReq = Meta;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::{ApiResponse, PageResp};

    fn meta_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "path": "/a",
            "password": "c",
            "p_sub": false,
            "write": false,
            "w_sub": false,
            "hide": "",
            "h_sub": false,
            "readme": "",
            "r_sub": false
        })
    }

    #[test]
    fn openapi_admin_meta_examples_match_models() {
        let meta: Meta = serde_json::from_value(meta_json()).unwrap();
        assert_eq!(meta.id, 1);
        assert_eq!(meta.path, "/a");

        let req = MetaReq {
            id: 0,
            path: "/a".to_string(),
            password: "c".to_string(),
            p_sub: false,
            write: false,
            w_sub: false,
            hide: String::new(),
            h_sub: false,
            readme: String::new(),
            r_sub: false,
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "id": 0,
                "path": "/a",
                "password": "c",
                "p_sub": false,
                "write": false,
                "w_sub": false,
                "hide": "",
                "h_sub": false,
                "readme": "",
                "r_sub": false
            })
        );

        let list: ApiResponse<PageResp<Meta>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "content": [meta_json()],
                "total": 1
            }
        }))
        .unwrap();
        assert_eq!(list.data.total, 1);
        assert_eq!(list.data.content[0].password, "c");
    }
}
