//! Common response envelopes and pagination payloads shared by AList APIs.

use serde::{Deserialize, Serialize};

/// Standard AList JSON response envelope.
///
/// AList normally returns HTTP 200 for API errors and puts the actual status in
/// this `code` field, so client code must inspect the envelope instead of only
/// the HTTP status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// AList logical status code, for example `200`, `403`, or `500`.
    pub code: i32,
    /// Human-readable server message.
    pub message: String,
    /// Endpoint-specific payload. Error responses commonly use `null`.
    pub data: T,
}

/// Pagination request used by list/search-style APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PageReq {
    /// One-based page number.
    pub page: i32,
    /// Page size. AList accepts `0` for all items on several fs endpoints.
    pub per_page: i32,
}

impl PageReq {
    /// Request all rows for endpoints that support `per_page = 0`.
    pub fn all() -> Self {
        Self {
            page: 1,
            per_page: 0,
        }
    }
}

/// Generic paginated response used by search and admin-like endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageResp<T> {
    /// Page content.
    pub content: Vec<T>,
    /// Total number of rows reported by the server.
    pub total: i64,
}

/// Task info returned when AList accepts a long-running operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskInfo {
    /// Task identifier.
    pub id: String,
    /// Server-generated task name.
    pub name: String,
    /// Numeric task state from AList internals.
    pub state: i32,
    /// Human-readable status text.
    pub status: String,
    /// Task progress percentage or server-defined progress value.
    pub progress: i32,
    /// Error text, empty when the task has not failed.
    pub error: String,
}

/// Response payload for upload endpoints that may either complete immediately
/// or return a background task when `As-Task` is enabled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadResp {
    /// Background task details. Some successful direct uploads return `null`
    /// data instead of this object, so callers should handle both shapes.
    pub task: TaskInfo,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::auth::LoginResp;
    use serde_json::Value;

    #[test]
    fn api_response_deserializes_success_and_error_envelopes() {
        let ok: ApiResponse<LoginResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": { "token": "abc", "device_key": "dev" }
        }))
        .unwrap();
        assert_eq!(ok.data.token, "abc");
        assert_eq!(ok.data.device_key.as_deref(), Some("dev"));

        let err: ApiResponse<Value> = serde_json::from_value(serde_json::json!({
            "code": 403,
            "message": "permission denied",
            "data": null
        }))
        .unwrap();
        assert_eq!(err.code, 403);
        assert_eq!(err.message, "permission denied");
        assert!(err.data.is_null());
    }

    #[test]
    fn openapi_null_response_examples_match_envelope_model() {
        let success: ApiResponse<Option<Value>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": null
        }))
        .unwrap();
        assert_eq!(success.code, 200);
        assert!(success.data.is_none());

        let forbidden: ApiResponse<Option<Value>> = serde_json::from_value(serde_json::json!({
            "code": 403,
            "message": "registration is disabled",
            "data": null
        }))
        .unwrap();
        assert_eq!(forbidden.code, 403);
        assert_eq!(forbidden.message, "registration is disabled");
        assert!(forbidden.data.is_none());
    }

    #[test]
    fn openapi_upload_task_examples_match_models() {
        let resp: ApiResponse<UploadResp> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "task": {
                    "id": "sdH2LbjyWRk",
                    "name": "upload animated_zoom.gif to [/data](/alist)",
                    "state": 0,
                    "status": "uploading",
                    "progress": 0,
                    "error": ""
                }
            }
        }))
        .unwrap();

        assert_eq!(resp.data.task.id, "sdH2LbjyWRk");
        assert_eq!(resp.data.task.status, "uploading");
        assert_eq!(resp.data.task.progress, 0);
    }
}
