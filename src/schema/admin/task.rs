//! admin-task 管理员任务域数据模型。
//!
//! 管理员级任务管理与通用任务域（[`crate::schema::task`]）在 AList 服务端
//! 共享完全相同的底层任务实体与类别枚举：
//!
//! - 服务端统一调度 7 类后台任务管理器（上传、复制、离线下载、离线下载转存、
//!   S3 归档解冻、压缩包解压与解压内容转存上传）；
//! - 单任务状态详情（[`TaskInfo`]）、任务列表（[`TaskInfoList`]）及批量操作结果
//!   （[`BatchTaskResult`]）在管理员路由与通用任务路由下具有完全一致的 JSON 数据契约；
//! - 仅路由前缀与访问控制权限不同（管理员级接口要求管理员凭据并可查看/操作全部用户的任务）。
//!
//! 故本模块直接重导出 task 域模型，避免重复定义；
//! `admin-task-schema` feature 已声明拉取 `task-schema` 以保证依赖类型可用。

pub use crate::schema::task::{BatchTaskResult, TaskCategory, TaskInfo, TaskInfoList};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 验证任务类别枚举的构造、标识字符串转换与 Display/AsRef 实现。
    #[test]
    fn admin_task_category_construction_and_display() {
        let categories = [
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

        for (category, expected) in categories {
            assert_eq!(category.as_str(), expected);
            assert_eq!(category.as_ref(), expected);
            assert_eq!(category.to_string(), expected);
        }
    }

    /// 验证任务类别枚举的序列化与反序列化（snake_case 契约）。
    #[test]
    fn admin_task_category_serde_roundtrip() {
        let json = serde_json::to_string(&TaskCategory::OfflineDownload).unwrap();
        assert_eq!(json, "\"offline_download\"");

        let category: TaskCategory = serde_json::from_str("\"decompress_upload\"").unwrap();
        assert_eq!(category, TaskCategory::DecompressUpload);
    }

    /// 验证单个任务信息 [`TaskInfo`] 的手动构造与字段读取。
    #[test]
    fn admin_task_info_manual_construction() {
        let task = TaskInfo {
            id: "admin-task-1".to_owned(),
            name: "copy /src to /dst".to_owned(),
            creator: "admin".to_owned(),
            creator_role: vec![1, 2],
            state: 2,
            status: "running".to_owned(),
            progress: 45.5,
            start_time: None,
            end_time: None,
            total_bytes: 4096,
            error: String::new(),
        };

        assert_eq!(task.id, "admin-task-1");
        assert_eq!(task.name, "copy /src to /dst");
        assert_eq!(task.creator, "admin");
        assert_eq!(task.creator_role, vec![1, 2]);
        assert_eq!(task.state, 2);
        assert_eq!(task.status, "running");
        assert_eq!(task.progress, 45.5);
        assert_eq!(task.start_time, None);
        assert_eq!(task.end_time, None);
        assert_eq!(task.total_bytes, 4096);
        assert!(task.error.is_empty());
    }

    /// 正向钉扎：单任务详情响应解码为 [`TaskInfo`]。
    #[test]
    fn admin_task_info_decodes_single_task_response() {
        let resp: Response<TaskInfo> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "id": "admin-copy-1",
                "name": "copy file.mkv to /dest",
                "creator": "admin",
                "creator_role": [1],
                "state": 3,
                "status": "succeeded",
                "progress": 100.0,
                "start_time": "2024-01-01T00:00:00Z",
                "end_time": "2024-01-01T00:05:00Z",
                "total_bytes": 1073741824,
                "error": ""
            }
        }))
        .unwrap();

        assert_eq!(resp.code, 200);
        assert_eq!(resp.data.id, "admin-copy-1");
        assert_eq!(resp.data.state, 3);
        assert_eq!(resp.data.progress, 100.0);
        assert!(resp.data.start_time.is_some());
        assert!(resp.data.end_time.is_some());
        assert_eq!(resp.data.total_bytes, 1073741824);
    }

    /// 兼容钉扎：任务信息在缺失默认字段时的容错解码。
    #[test]
    fn admin_task_info_tolerates_missing_defaults() {
        let task: TaskInfo = serde_json::from_value(serde_json::json!({
            "id": "minimal-task",
            "name": "minimal task description",
            "state": 0,
            "status": "pending",
            "progress": 0,
            "error": ""
        }))
        .unwrap();

        assert_eq!(task.id, "minimal-task");
        assert_eq!(task.creator, "");
        assert!(task.creator_role.is_empty());
        assert_eq!(task.start_time, None);
        assert_eq!(task.end_time, None);
        assert_eq!(task.total_bytes, 0);
    }

    /// 正向钉扎：任务列表响应解码为 [`TaskInfoList`]。
    #[test]
    fn admin_task_info_list_decodes_list_response() {
        let resp: Response<TaskInfoList> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [
                {
                    "id": "task-undone-1",
                    "name": "upload part1.bin to /data",
                    "creator": "admin",
                    "creator_role": [1],
                    "state": 1,
                    "status": "uploading",
                    "progress": 50.0,
                    "start_time": "2024-01-01T00:00:00Z",
                    "end_time": null,
                    "total_bytes": 52428800,
                    "error": ""
                }
            ]
        }))
        .unwrap();

        assert_eq!(resp.code, 200);
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].id, "task-undone-1");
        assert_eq!(resp.data[0].status, "uploading");
        assert_eq!(resp.data[0].progress, 50.0);
        assert_eq!(resp.data[0].end_time, None);
    }

    /// 正向钉扎：批量任务控制结果 [`BatchTaskResult`] 解码。
    #[test]
    fn admin_batch_task_result_decodes_success_and_failure() {
        // 全成功：服务端返回空映射
        let success_resp: Response<BatchTaskResult> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {}
        }))
        .unwrap();
        assert!(success_resp.data.is_empty());

        // 部分失败：服务端返回失败任务映射
        let failure_resp: Response<BatchTaskResult> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "t-1": "task not found",
                "t-2": "cannot cancel completed task"
            }
        }))
        .unwrap();
        assert_eq!(failure_resp.data.len(), 2);
        assert_eq!(failure_resp.data.get("t-1").unwrap(), "task not found");
        assert_eq!(
            failure_resp.data.get("t-2").unwrap(),
            "cannot cancel completed task"
        );
    }
}
