//! admin-label 标签域数据模型。
//!
//! 覆盖 `/api/label/list`、`/api/label/get`（读取）与
//! `/api/admin/label/create`、`/api/admin/label/update`、`/api/admin/label/delete`（写入）
//! 涉及的数据形状：标签条目 [`Label`] 与创建响应 [`CreateLabelResponse`]。
//!
//! ## 字段形状来源
//!
//! 该路由分组未被 AList OpenAPI 规范收录，字段形状以 AList 服务端源码为准：
//! 条目字段来自 model.Label 数据模型，
//! 响应包装来自通用响应模块，
//! 各操作的请求/响应形状来自服务端标签处理模块的
//! `ListLabel`/`GetLabel`/`CreateLabel`/`UpdateLabel`/`DeleteLabel`。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 将显式 `null` 归约为 `T` 的默认值（老版本服务端可能对新字段返回 `null`）。
#[allow(dead_code)]
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// 标签条目。
///
/// 对应 AList 服务端 model.Label 数据模型；
/// 由 `/api/label/list`（分页包装内）、`/api/label/get` 与 `/api/admin/label/update` 返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// 标签 ID。
    ///
    /// 对应服务端主键 ID。
    pub id: u64,
    /// 标签类型。
    ///
    /// JSON 键为保留字 `type`，在 Rust 中重命名为 `label_type`。
    #[serde(rename = "type")]
    pub label_type: i32,
    /// 标签名称。
    ///
    /// 服务端按名称查重，创建时必填。
    pub name: String,
    /// 标签描述。
    ///
    /// 从 AList `v3.46.0` 起新增标签描述；老版本缺失时为 [`None`]。
    #[serde(default)]
    pub description: Option<String>,
    /// 标签背景色。
    ///
    /// 从 AList `v3.46.0` 起新增标签背景色；老版本缺失时为 [`None`]。
    #[serde(default)]
    pub bg_color: Option<String>,
    /// 创建时间。
    ///
    /// 对应服务端创建时间戳，JSON 中永不缺失或为 `null`。
    pub create_time: DateTime<Utc>,
}

/// 创建标签的响应数据。
///
/// 对应 AList 服务端标签创建处理函数：成功时以 `gin.H{"id": id}` 返回新建标签的数据库 ID。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateLabelResponse {
    /// 新建标签的 ID。
    pub id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：按 AList 服务端 model.Label 数据模型的 JSON 字段
    /// 构造 Go 服务端 `ListLabel`/`GetLabel` 的实际返回形状（`time.Time` 以
    /// RFC3339 纳秒精度序列化），逐字段断言。
    #[test]
    fn label_decodes_go_server_shape() {
        let label: Label = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": 2,
            "name": "电影",
            "description": "电影相关文件",
            "bg_color": "#FF0000",
            "create_time": "2024-06-01T12:00:00.123456789+08:00"
        }))
        .unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.label_type, 2);
        assert_eq!(label.name, "电影");
        assert_eq!(label.description.as_deref(), Some("电影相关文件"));
        assert_eq!(label.bg_color.as_deref(), Some("#FF0000"));
        assert_eq!(
            label.create_time.to_rfc3339(),
            "2024-06-01T04:00:00.123456789+00:00"
        );
    }

    /// 兼容钉扎：老版本服务端省略 `description`/`bg_color` 时，缺失字段归约
    /// 为默认值而不是反序列化失败。`create_time` 对应 Go 非指针 `time.Time`，
    /// 服务端总是携带，不属于可缺失字段（测试 JSON 中必须提供）。
    #[test]
    fn label_tolerates_missing_optional_fields() {
        let label: Label = serde_json::from_value(serde_json::json!({
            "id": 3,
            "type": 0,
            "name": "音乐",
            "create_time": "2024-06-01T12:00:00Z"
        }))
        .unwrap();
        assert_eq!(label.description, None);
        assert_eq!(label.bg_color, None);
        assert_eq!(label.create_time.to_rfc3339(), "2024-06-01T12:00:00+00:00");
    }

    /// 显式 `null` 兼容：集合/标量辅助字段为 `null` 时归约为默认值。
    /// `create_time` 对应 Go 非指针 `time.Time`（服务端永不返回 `null`），
    /// 不做 `null` 兼容。
    #[test]
    fn label_tolerates_null_collection_fields() {
        let label: Label = serde_json::from_value(serde_json::json!({
            "id": 3,
            "type": 0,
            "name": "音乐",
            "description": null,
            "bg_color": null,
            "create_time": "2024-06-01T12:00:00Z"
        }))
        .unwrap();
        assert_eq!(label.description, None);
        assert_eq!(label.bg_color, None);
        assert_eq!(label.create_time.to_rfc3339(), "2024-06-01T12:00:00+00:00");
    }

    /// 分页钉扎：`/api/label/list` 的 `PageResponse<Label>` 包装形态。
    #[test]
    fn label_list_wraps_into_page_response() {
        let page: crate::schema::common::PageResponse<Label> =
            serde_json::from_value(serde_json::json!({
                "content": [{
                    "id": 7,
                    "type": 1,
                    "name": "学习",
                    "description": "",
                    "bg_color": "",
                    "create_time": "2024-06-01T12:00:00Z"
                }],
                "total": 1
            }))
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].id, 7);
        assert_eq!(page.content[0].name, "学习");
    }

    /// 序列化键名钉扎：`type` 为 JSON 保留字，`rename` 后必须以
    /// `type` 而非 `label_type` 落盘（对应 Go `model.Label.Type` 的 JSON tag）。
    #[test]
    fn label_serializes_with_go_field_names() {
        let label = Label {
            id: 1,
            label_type: 2,
            name: "电影".to_owned(),
            description: Some("电影相关文件".to_owned()),
            bg_color: Some("#FF0000".to_owned()),
            create_time: "2024-06-01T12:00:00Z".parse().unwrap(),
        };
        let value = serde_json::to_value(&label).unwrap();
        assert_eq!(value["type"], 2, "JSON 键应为 type: {value}");
        assert!(value.get("label_type").is_none(), "{value}");
        assert_eq!(value["id"], 1);
        assert_eq!(value["name"], "电影");
        assert_eq!(value["bg_color"], "#FF0000");
        assert!(value["create_time"].is_string(), "{value}");
    }

    /// 创建响应钉扎：服务端标签创建成功时返回 `gin.H{"id": id}`。
    #[test]
    fn create_label_response_decodes_handler_shape() {
        let resp: crate::schema::common::Response<CreateLabelResponse> =
            serde_json::from_value(serde_json::json!({
                "code": 200,
                "message": "success",
                "data": { "id": 42 }
            }))
            .unwrap();
        assert_eq!(resp.data.id, 42);
    }
}
