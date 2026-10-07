//! AList 共享数据模型（无条件编译）。
//!
//! 本文件建模所有端点共用的信封与分页/任务结构，字段与
//! `examples/alist/server/common/resp.go`、`internal/model/req.go`、`server/handles/task.go`
//! 及 `docs/api/alistv3.openapi.yaml` 的示例保持一致。
//!
//! ## 信封语义
//!
//! AList 对绝大多数 API 错误返回 HTTP 200，真正状态在信封 `code` 中：
//!
//! ```json
//! { "code": 200, "message": "success", "data": { "...": "..." } }
//! { "code": 403, "message": "permission denied", "data": null }
//! ```
//!
//! [`Client`](crate::Client) 统一负责信封解码与状态检查。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AList 标准 JSON 响应信封。
///
/// 对应 `examples/alist/server/common/resp.go` 的 `Resp[T]`。
/// [`Client`](crate::Client) 用 [`serde_json::Value`] 解出 `data` 后再二次反序列化为端点模型，
/// 因此 `data: null` 可以自然解码为 `()` 或 `Option<T>`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope<T> {
    /// AList 逻辑状态码，例如 `200`、`403`、`500`。
    pub code: i32,
    /// 服务端消息。
    pub message: String,
    /// 端点特定数据；错误响应通常为 `null`。
    pub data: T,
}

/// 分页请求参数。
///
/// 对应 `examples/alist/internal/model/req.go` 的 `PageReq`（JSON/form 键为 `page`/`per_page`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PageReq {
    /// 页码，从 1 开始。
    pub page: i32,
    /// 每页条数；部分 fs 端点接受 `0` 表示全部。
    pub per_page: i32,
}

impl PageReq {
    /// 以第 1 页、每页 10 条构造分页参数。
    #[must_use]
    pub const fn new(page: i32, per_page: i32) -> Self {
        Self { page, per_page }
    }

    /// 请求全部数据（第 1 页、每页 0 条）。
    ///
    /// 支持 `per_page = 0` 的端点会返回所有条目。
    #[must_use]
    pub const fn all() -> Self {
        Self {
            page: 1,
            per_page: 0,
        }
    }
}

/// 通用分页响应。
///
/// 对应 `examples/alist/server/common/resp.go` 的 `PageResp`
/// 以及 admin 列表端点返回的 `{ "content": [...], "total": ... }` 结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageResp<T> {
    /// 当前页内容。
    pub content: Vec<T>,
    /// 服务端报告的条目总数。
    pub total: i64,
}

/// 后台任务信息。
///
/// 对应 `examples/alist/server/handles/task.go` 的 `TaskInfo`，
/// 由上传/离线下载等长耗时操作返回。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务 ID。
    pub id: String,
    /// 服务端生成的任务名称。
    pub name: String,
    /// 创建者用户名；匿名任务为空字符串。
    #[serde(default)]
    pub creator: String,
    /// 创建者角色 ID 列表（Go 侧为 `model.Roles []int`）。
    #[serde(default)]
    pub creator_role: Vec<i32>,
    /// 任务状态数值（tache 状态机的枚举值）。
    pub state: i32,
    /// 人类可读的状态文本。
    pub status: String,
    /// 进度百分比；服务端对 NaN 会回退为 `100`。
    pub progress: f64,
    /// 开始时间；未开始为 `null`。
    #[serde(default)]
    pub start_time: Option<DateTime<Utc>>,
    /// 结束时间；未结束为 `null`。
    #[serde(default)]
    pub end_time: Option<DateTime<Utc>>,
    /// 任务涉及的总字节数。
    #[serde(default)]
    pub total_bytes: i64,
    /// 失败原因；未失败为空字符串。
    #[serde(default)]
    pub error: String,
}

/// 上传端点的响应数据。
///
/// 对应 `examples/alist/server/handles/fsup.go` 中 `FsStream`/`FsForm` 以
/// `gin.H{"task": getTaskInfo(t)}` 返回的结构。
/// 直传成功（未启用 `As-Task`）时端点 `data` 为 `null`，
/// 因此调用方应以 `Option<UploadResp>` 作为端点模型。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadResp {
    /// 后台上传任务详情。
    pub task: TaskInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 示例 JSON 钉扎测试：钉住 openapi/Go 源码中的响应结构，防止模型漂移。
    #[test]
    fn envelope_decodes_success_and_error_examples() {
        let ok: Envelope<serde_json::Value> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": { "token": "abcd" }
        }))
        .unwrap();
        assert_eq!(ok.code, 200);
        assert_eq!(ok.message, "success");
        assert_eq!(ok.data["token"], "abcd");

        let err: Envelope<serde_json::Value> = serde_json::from_value(serde_json::json!({
            "code": 403,
            "message": "permission denied",
            "data": null
        }))
        .unwrap();
        assert_eq!(err.code, 403);
        assert!(err.data.is_null());
    }

    #[test]
    fn envelope_null_data_decodes_to_unit_and_option() {
        let unit: Envelope<()> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": null
        }))
        .unwrap();
        assert_eq!(unit.data, ());

        let none: Envelope<Option<PageReq>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": null
        }))
        .unwrap();
        assert_eq!(none.data, None);
    }

    #[test]
    fn page_req_serializes_with_api_field_names() {
        let req = PageReq::new(2, 30);
        assert_eq!(
            serde_json::to_value(req).unwrap(),
            serde_json::json!({ "page": 2, "per_page": 30 })
        );
        assert_eq!(
            PageReq::all(),
            PageReq {
                page: 1,
                per_page: 0
            }
        );
    }

    #[test]
    fn page_resp_decodes_admin_list_example() {
        let resp: PageResp<i64> = serde_json::from_value(serde_json::json!({
            "content": [1, 2, 3],
            "total": 3
        }))
        .unwrap();
        assert_eq!(resp.content, vec![1, 2, 3]);
        assert_eq!(resp.total, 3);
    }

    /// 钉扎 `examples/alist/server/handles/task.go` TaskInfo 的 JSON 形状。
    #[test]
    fn task_info_decodes_upload_task_example() {
        let task: TaskInfo = serde_json::from_value(serde_json::json!({
            "id": "sdH2LbjyWRk",
            "name": "upload animated_zoom.gif to [/data](/alist)",
            "creator": "admin",
            "creator_role": [2],
            "state": 0,
            "status": "uploading",
            "progress": 50.0,
            "start_time": "2024-01-01T00:00:00Z",
            "end_time": null,
            "total_bytes": 1024,
            "error": ""
        }))
        .unwrap();
        assert_eq!(task.id, "sdH2LbjyWRk");
        assert_eq!(task.creator, "admin");
        assert_eq!(task.creator_role, vec![2]);
        assert_eq!(task.state, 0);
        assert_eq!(task.progress, 50.0);
        assert!(task.start_time.is_some());
        assert_eq!(task.end_time, None);
        assert_eq!(task.total_bytes, 1024);
        assert_eq!(task.error, "");
    }

    #[test]
    fn task_info_tolerates_missing_optional_fields() {
        let task: TaskInfo = serde_json::from_value(serde_json::json!({
            "id": "abc",
            "name": "upload demo.txt",
            "state": 2,
            "status": "succeeded",
            "progress": 100,
            "error": ""
        }))
        .unwrap();
        assert_eq!(task.creator, "");
        assert!(task.creator_role.is_empty());
        assert_eq!(task.start_time, None);
        assert_eq!(task.total_bytes, 0);
    }

    #[test]
    fn upload_resp_decodes_fs_stream_task_example() {
        let resp: Envelope<UploadResp> = serde_json::from_value(serde_json::json!({
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
    }
}
