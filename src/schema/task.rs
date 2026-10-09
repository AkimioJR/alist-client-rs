//! task 任务域数据模型。
//!
//! 覆盖 `/api/task` 下任务端点的响应模型：
//!
//! - 任务类别以 [`TaskCategory`] 表达（对应 AList 7 种服务端任务路由管理器分支）；
//! - 任务详情复用共享模型 [`TaskInfo`]（由 [`crate::schema::common`] 建模，此处 re-export 便于按域引用）；
//! - 未完成/已完成列表（`GET /undone`、`GET /done`）的响应 `data` 为任务数组
//!   （服务端返回任务切片，**非分页包裹**），以 [`TaskInfoList`] 表达；
//! - 单任务查询（`POST /info`）的响应 `data` 为单个任务对象；
//! - 批量任务控制端点（`cancel_some`、`delete_some`、`retry_some`）的响应 `data`
//!   以 [`BatchTaskResult`] 映射表达（失败任务 ID 到原因描述，全部成功返回空映射）；
//! - 单任务取消/删除/重试/清空等控制端点的 `data` 为 `null`，端点以 `()` 接收，无需专门模型。
//!
//! ## 字段形状来源
//!
//! 数据来源：AList OpenAPI 规范与 AList 服务端任务处理模块（Go `server/handles/task.go`）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use crate::schema::common::TaskInfo;

/// 任务类别枚举。
///
/// 对应 AList 服务端 7 种任务管理器路由分支（Go `handles.SetupTaskRoute`）：
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskCategory {
    /// 上传任务。
    Upload,
    /// 跨存储复制任务。
    Copy,
    /// 离线下载任务（第一阶段：远端拉取）。
    OfflineDownload,
    /// 离线下载转存任务（第二阶段：转存至目标存储）。
    OfflineDownloadTransfer,
    /// S3 存储归档/解冻转换任务。
    S3Transition,
    /// 压缩包下载与解压任务（第一阶段：下载与本地解包）。
    Decompress,
    /// 解压内容转存上传任务（第二阶段：解压内容上传回存储）。
    DecompressUpload,
}

impl TaskCategory {
    /// 获取任务类别在 API 路径中的英文标识字符串。
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Upload => "upload",
            Self::Copy => "copy",
            Self::OfflineDownload => "offline_download",
            Self::OfflineDownloadTransfer => "offline_download_transfer",
            Self::S3Transition => "s3_transition",
            Self::Decompress => "decompress",
            Self::DecompressUpload => "decompress_upload",
        }
    }
}

impl AsRef<str> for TaskCategory {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 批量任务操作执行结果映射。
///
/// 对应 AList 批量接口（`cancel_some`、`delete_some`、`retry_some`）响应的 `data`：
/// 键为操作失败的任务 ID，值为失败原因描述（全部成功时返回空映射 `{}`）。
pub type BatchTaskResult = HashMap<String, String>;

/// 任务列表。
///
/// 对应 `GET /api/task/upload/undone` 与 `GET /api/task/upload/done` 等端点
/// 的响应 `data`：任务数组（非分页，无 `content`/`total` 包裹）。
/// 字段形状以 AList 服务端源码为准。
pub type TaskInfoList = Vec<TaskInfo>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 验证全部 7 种任务类别的标识字符串及 Display/AsRef 实现。
    #[test]
    fn task_category_as_str_and_display() {
        let cases = [
            (TaskCategory::Upload, "upload"),
            (TaskCategory::Copy, "copy"),
            (TaskCategory::OfflineDownload, "offline_download"),
            (
                TaskCategory::OfflineDownloadTransfer,
                "offline_download_transfer",
            ),
            (TaskCategory::S3Transition, "s3_transition"),
            (TaskCategory::Decompress, "decompress"),
            (TaskCategory::DecompressUpload, "decompress_upload"),
        ];

        for (category, expected) in cases {
            assert_eq!(category.as_str(), expected);
            assert_eq!(category.as_ref(), expected);
            assert_eq!(category.to_string(), expected);
        }
    }

    /// 验证 `TaskCategory` 的 serde 序列化与反序列化（snake_case 钉扎）。
    #[test]
    fn task_category_serde_roundtrip() {
        let cases = [
            (TaskCategory::Upload, "\"upload\""),
            (TaskCategory::Copy, "\"copy\""),
            (TaskCategory::OfflineDownload, "\"offline_download\""),
            (
                TaskCategory::OfflineDownloadTransfer,
                "\"offline_download_transfer\"",
            ),
            (TaskCategory::S3Transition, "\"s3_transition\""),
            (TaskCategory::Decompress, "\"decompress\""),
            (TaskCategory::DecompressUpload, "\"decompress_upload\""),
        ];

        for (category, json_str) in cases {
            // 序列化验证
            let serialized = serde_json::to_string(&category).unwrap();
            assert_eq!(serialized, json_str);

            // 反序列化验证
            let deserialized: TaskCategory = serde_json::from_str(json_str).unwrap();
            assert_eq!(deserialized, category);
        }

        // 无效类别反序列化失败验证
        let err = serde_json::from_str::<TaskCategory>("\"non_existent_category\"");
        assert!(err.is_err());
    }

    /// 验证批量任务执行结果 `BatchTaskResult` 的反序列化（全成功与部分失败）。
    #[test]
    fn batch_task_result_decodes_empty_and_partial_failures() {
        // 全成功场景：服务端返回空映射 `{}`
        let success_resp: Response<BatchTaskResult> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {}
        }))
        .unwrap();
        assert_eq!(success_resp.code, 200);
        assert!(success_resp.data.is_empty());

        // 部分失败场景：包含失败任务 ID 与错误描述映射
        let failure_resp: Response<BatchTaskResult> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "tid_1": "task not found",
                "tid_2": "permission denied"
            }
        }))
        .unwrap();
        assert_eq!(failure_resp.code, 200);
        assert_eq!(failure_resp.data.len(), 2);
        assert_eq!(failure_resp.data.get("tid_1").unwrap(), "task not found");
        assert_eq!(failure_resp.data.get("tid_2").unwrap(), "permission denied");
    }

    /// 正向钉扎：`GET /api/task/upload/done` 的响应与任务数组形状。
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

    /// 正向钉扎：`POST /api/task/upload/info` 的 `data` 为**单个**任务对象。
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
