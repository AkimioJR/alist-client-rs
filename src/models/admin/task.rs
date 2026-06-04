//! Data models for `/api/admin/task/*`.

use serde::{Deserialize, Serialize};

/// Upload task info returned by admin upload task endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadTaskInfo {
    /// Task id.
    pub id: String,
    /// Task name.
    pub name: String,
    /// Task state.
    pub state: String,
    /// Status text.
    pub status: String,
    /// Progress value.
    pub progress: i32,
    /// Error text.
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::ApiResponse;

    #[test]
    fn openapi_admin_upload_task_examples_match_models() {
        let resp: ApiResponse<Vec<UploadTaskInfo>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [{
                "id": "1",
                "name": "upload 1.png to [/s](/test)",
                "state": "succeeded",
                "status": "",
                "progress": 100,
                "error": ""
            }]
        }))
        .unwrap();

        assert_eq!(resp.data[0].id, "1");
        assert_eq!(resp.data[0].state, "succeeded");
        assert_eq!(resp.data[0].progress, 100);
    }
}
