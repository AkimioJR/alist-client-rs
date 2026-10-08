//! admin-task 任务域数据模型。
//!
//! 覆盖 `/api/admin/task/upload` 下上传任务端点的响应模型：
//!
//! - 任务详情复用共享模型 [`TaskInfo`]（由 `schema/common.rs` 建模，此处 re-export 便于按域引用）；
//! - 未完成/已完成列表（`GET /undone`、`GET /done`）的响应 `data` 为任务数组
//!   （服务端返回任务切片，**非分页包裹**），以 [`TaskInfoList`] 表达；
//! - 单任务查询（`POST /info`）的响应 `data` 为单个任务对象；
//! - 取消/删除/重试/清空端点的 `data` 为 `null`，端点以 `()` 接收，无需专门模型。
//!
//! ## 字段形状来源
//!
//! 数据来源：AList OpenAPI 规范的 `admin/task/upload` 分组与 AList 服务端任务处理模块。

pub use crate::schema::common::TaskInfo;

/// 上传任务列表。
///
/// `GET /api/admin/task/upload/undone` 与 `GET /api/admin/task/upload/done`
/// 的响应 `data`：任务数组（非分页，无 `content`/`total` 包裹）。
/// 字段形状以 AList 服务端源码为准。
pub type TaskInfoList = Vec<TaskInfo>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 正向钉扎：`GET /api/admin/task/upload/done` 的响应与任务数组形状。
    ///
    /// 响应 `data` 为数组；任务元素字段以 AList 服务端 `TaskInfo` 结构体为准。
    #[test]
    fn task_info_list_decodes_done_endpoint_response() {
        let resp: Response<TaskInfoList> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [
                {
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
                }
            ]
        }))
        .unwrap();
        assert_eq!(resp.code, 200);
        assert_eq!(resp.data.len(), 1);
        let task = &resp.data[0];
        assert_eq!(task.id, "sdH2LbjyWRk");
        assert_eq!(task.name, "upload animated_zoom.gif to [/data](/alist)");
        assert_eq!(task.state, 0);
        assert_eq!(task.progress, 50.0);
        assert!(task.start_time.is_some());
        assert_eq!(task.end_time, None); // Option 字段：显式 null → None
        assert_eq!(task.total_bytes, 1024);
    }

    /// 正向钉扎：`POST /api/admin/task/upload/info` 的 `data` 为**单个**任务对象。
    ///
    /// 服务端源码返回单个任务对象，以此为准钉住。
    #[test]
    fn info_endpoint_decodes_single_task_object() {
        let resp: Response<TaskInfo> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "id": "sdH2Lbjfs38yGMbHqyWRk",
                "name": "upload 1.png to [/s](/test)",
                "creator": "admin",
                "creator_role": [2],
                "state": 3,
                "status": "succeeded",
                "progress": 100,
                "start_time": null,
                "end_time": "2024-01-01T00:01:00Z",
                "total_bytes": 2048,
                "error": ""
            }
        }))
        .unwrap();
        let task = &resp.data;
        assert_eq!(task.id, "sdH2Lbjfs38yGMbHqyWRk");
        assert_eq!(task.state, 3);
        assert_eq!(task.progress, 100.0); // 整数字面量按 f64 解出
        assert_eq!(task.start_time, None);
        assert!(task.end_time.is_some());
    }

    /// 兼容钉扎：服务端只返回精简字段集时也能解出
    /// （缺失字段走 `#[serde(default)]`）。
    ///
    /// 注意：老文档示例把 `state` 写作字符串（`"succeeded"`），与 Go 侧
    /// `tache.State`（整型）冲突，以 Go 为准建模为 `i32`，故此处用整型钉住。
    #[test]
    fn task_info_list_tolerates_minimal_task_fields() {
        let resp: Response<TaskInfoList> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "1",
                    "name": "upload 1.png to [/s](/test)",
                    "state": 3,
                    "status": "",
                    "progress": 100,
                    "error": ""
                }
            ]
        }))
        .unwrap();
        let task = &resp.data[0];
        assert_eq!(task.id, "1");
        assert_eq!(task.creator, ""); // 缺失 → default 空串
        assert!(task.creator_role.is_empty()); // 缺失 → default 空 Vec
        assert_eq!(task.start_time, None); // 缺失 → default None
        assert_eq!(task.total_bytes, 0); // 缺失 → default 0
    }
}
