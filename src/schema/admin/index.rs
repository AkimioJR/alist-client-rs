//! admin-index 搜索索引域数据模型。
//!
//! 建模 `/api/admin/index` 端点族的响应数据形状：
//!
//! - [`IndexProgress`]：`progress` 端点的响应模型（搜索索引构建进度），
//!   对应 AList 服务端搜索索引数据模型中的 `IndexProgress`。
//!
//! ## 字段形状来源
//!
//! 字段形状以 AList 服务端路由定义的 admin `index` 分组、索引处理器实现
//! （构建/停止/清空/进度查询，实现为 `BuildIndex`/`StopIndex`/`ClearIndex`/`GetProgress`）
//! 与 AList 服务端搜索索引数据模型 `IndexProgress` 为准；
//! AList OpenAPI 规范未收录该分组（OpenList 规范虽有收录，但其进度字段形状与
//! 服务端实现不一致，以服务端源码为准）。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 搜索索引构建进度。
///
/// 对应 AList 服务端搜索索引数据模型中的 `IndexProgress`
/// （`GET /api/admin/index/progress` 响应 `data`）。
/// 索引构建在服务端后台异步执行，进度由服务端持久化为私有设置项。
/// 从 AList `v3.6.0` 起提供。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexProgress {
    /// 已索引对象计数（累计扫描到的文件/目录数量）。
    ///
    /// 对应服务端 `IndexProgress.obj_count`。
    pub obj_count: u64,
    /// 索引构建是否已完成。
    ///
    /// 对应服务端 `IndexProgress.is_done`；构建进行中为 `false`。
    pub is_done: bool,
    /// 上次完成索引构建的时间。
    ///
    /// 对应服务端 `IndexProgress.last_done_time`（Go `*time.Time`，序列化
    /// 恒输出该键）；从未完成过构建或该次构建尚未结束时，服务端对 nil 指针
    /// 序列化为 `null`，解码为 [`None`]。
    #[serde(default)]
    pub last_done_time: Option<DateTime<Utc>>,
    /// 构建过程中发生的错误信息。
    ///
    /// 对应服务端 `IndexProgress.error`；构建正常时为空字符串。
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：索引构建已完成的服务端响应形状
    /// （字段与服务端 `IndexProgress` 的 JSON 键一一对应）。
    #[test]
    fn index_progress_decodes_completed_shape() {
        // 形状来源：AList 服务端索引处理器完成构建后写入的进度
        // （`obj_count`/`is_done`/`last_done_time`/`error` 四字段）
        let progress: IndexProgress = serde_json::from_value(serde_json::json!({
            "obj_count": 12345,
            "is_done": true,
            "last_done_time": "2024-06-01T08:00:00Z",
            "error": ""
        }))
        .unwrap();
        assert_eq!(progress.obj_count, 12345);
        assert!(progress.is_done);
        assert_eq!(
            progress.last_done_time.map(|time| time.to_rfc3339()),
            Some("2024-06-01T08:00:00+00:00".to_owned())
        );
        assert_eq!(progress.error, "");
    }

    /// 正向钉扎：索引构建进行中的服务端响应形状（`last_done_time` 为显式 `null`）。
    #[test]
    fn index_progress_decodes_in_progress_shape_with_null_last_done_time() {
        // 形状来源：AList 服务端索引处理器构建中写入的进度
        // （`last_done_time` 为 Go nil 指针，序列化为 `null`）
        let progress: IndexProgress = serde_json::from_value(serde_json::json!({
            "obj_count": 678,
            "is_done": false,
            "last_done_time": null,
            "error": ""
        }))
        .unwrap();
        assert_eq!(progress.obj_count, 678);
        assert!(!progress.is_done);
        assert_eq!(progress.last_done_time, None);
    }

    /// 兼容钉扎：`last_done_time` 字段缺失时由 `#[serde(default)]` 兜底解码
    /// 为 [`None`]，构建出错时 `error` 携带错误文本。
    #[test]
    fn index_progress_tolerates_missing_last_done_time() {
        let progress: IndexProgress = serde_json::from_value(serde_json::json!({
            "obj_count": 0,
            "is_done": true,
            "error": "some build error"
        }))
        .unwrap();
        assert_eq!(progress.obj_count, 0);
        assert!(progress.is_done);
        assert_eq!(progress.last_done_time, None);
        assert_eq!(progress.error, "some build error");
    }

    /// 序列化键名钉扎：序列化键与服务端 `IndexProgress` 的 JSON 键一致，
    /// `last_done_time` 为 [`None`] 时序列化为 `null`（对应 Go nil 指针）。
    #[test]
    fn index_progress_serializes_with_api_field_names() {
        let progress = IndexProgress {
            obj_count: 99,
            is_done: false,
            last_done_time: None,
            error: String::new(),
        };
        assert_eq!(
            serde_json::to_value(&progress).unwrap(),
            serde_json::json!({
                "obj_count": 99,
                "is_done": false,
                "last_done_time": null,
                "error": ""
            })
        );
    }
}
