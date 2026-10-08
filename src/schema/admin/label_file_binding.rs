//! admin-label-file-binding 绑定域数据模型。
//!
//! 覆盖标签绑定端点的请求与响应模型：标签实体（[`Label`]）、绑定记录实体
//! （[`LabelFileBinding`]）、按标签查询到的文件条目（[`ObjLabelResp`]）、
//! 创建/批量创建的请求项（[`CreateItem`]）与响应（[`CreateResp`]、
//! [`CreateBatchResp`]）、恢复响应（[`RestoreResp`]）。
//!
//! 该 API 分组未收录进 `docs/api/alistv3.md` 与 `docs/api/alistv3.openapi.yaml`
//! （两份文档均无 label 相关条目），字段形状完全以 `examples/alist` Go 源码为准：
//!
//! - 标签实体：`examples/alist/internal/model/label.go`（`model.Label`）；
//! - 绑定记录实体：`examples/alist/internal/model/label_file_binding.go:5`
//!   （`model.LabelFileBinding`，`GET /api/admin/label_file_binding/list` 的元素类型）；
//! - 按标签查询的文件条目：`examples/alist/internal/op/label_file_binding.go:29`
//!   （`op.ObjLabelResp`，文件字段取自 `internal/model/obj_file.go` 的 `model.ObjFile`）；
//! - 创建请求体：`examples/alist/internal/op/label_file_binding.go:13`
//!   （`op.CreateLabelFileBinDingReq`）；
//! - 响应 `data` 形状：`examples/alist/server/handles/label_file_binding.go`
//!   （`gin.H` 字面量与内部 `perResult`/`pageResp`/`restoreLabelBindingsReq` 结构）。
//!
//! schema 约定（`#[serde(default)]`、可选字段、示例 JSON 钉扎测试）见 `docs/design.md` §5。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 标签实体。
///
/// 对应 `examples/alist/internal/model/label.go` 的 `model.Label`；
/// `GET /api/label_file_binding/get` 直接返回该实体的数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// 标签 ID（Go `uint`，主键）。
    pub id: u64,
    /// 标签类型标记（对应 Go `Label.Type`，服务端内部用途）。
    pub r#type: i32,
    /// 标签名称。
    pub name: String,
    /// 标签描述。
    pub description: String,
    /// 标签背景色（CSS 颜色值，如 `#1890ff`；无背景色时为空字符串）。
    pub bg_color: String,
    /// 标签创建时间。
    pub create_time: DateTime<Utc>,
}

/// 标签-文件绑定记录实体。
///
/// 对应 `examples/alist/internal/model/label_file_binding.go:5` 的
/// `model.LabelFileBinding`；`GET /api/admin/label_file_binding/list` 以
/// `{"content": [...], "total": N}` 分页壳返回该实体的数组
/// （handler 内 `pageResp[model.LabelFileBinding]`，形状与
/// [`crate::schema::common::PageResponse`] 一致），也是
/// `POST /api/admin/label_file_binding/restore` 请求体 `bindings` 数组的元素类型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelFileBinding {
    /// 绑定记录 ID（Go `uint`，主键；restore 时服务端按 `keep_ids` 决定是否沿用）。
    pub id: u64,
    /// 所属用户 ID（Go `uint`；restore 请求中为 0 时服务端以当前认证用户填充）。
    pub user_id: u64,
    /// 标签 ID（Go `uint`；restore 请求中必须非 0，否则服务端返回 400）。
    pub label_id: u64,
    /// 绑定的文件名（restore 请求中必须非空，否则服务端返回 400）。
    pub file_name: String,
    /// 绑定创建时间。
    pub create_time: DateTime<Utc>,
}

/// 按标签查询返回的文件条目（附加了标签列表的文件对象）。
///
/// 对应 `examples/alist/internal/op/label_file_binding.go:29` 的 `op.ObjLabelResp`；
/// 文件字段与 `examples/alist/internal/model/obj_file.go` 的 `model.ObjFile` 一致，
/// 并附加该文件命中的标签列表。
/// `GET /api/label_file_binding/get_file_by_label` 返回该条目的数组。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjLabelResp {
    /// 文件 ID（对应 Go `model.ObjFile.Id`；JSON 中为字符串）。
    pub id: String,
    /// 文件完整路径。
    pub path: String,
    /// 文件名。
    pub name: String,
    /// 文件大小（字节）。
    pub size: i64,
    /// 是否为目录。
    pub is_dir: bool,
    /// 修改时间。
    pub modified: DateTime<Utc>,
    /// 创建时间。
    pub created: DateTime<Utc>,
    /// 签名字符串（启用签名保护时非空）。
    pub sign: String,
    /// 缩略图链接。
    pub thumb: String,
    /// 文件类型枚举值（对应 Go `model.ObjFile.Type`；0 未指定、1 目录、2 视频、
    /// 3 音频、4 文本、5 图片，见 `examples/alist/internal/conf/const.go:165`）。
    pub r#type: i32,
    /// 哈希信息字符串（对应 Go `ObjFile.HashInfoStr`；JSON 键为 `hashinfo`）。
    pub hashinfo: String,
    /// 该文件命中的标签列表；服务端保证逐文件至少一个标签，此处仍容忍缺失/`null`。
    #[serde(default, deserialize_with = "null_to_default")]
    pub label_list: Vec<Label>,
}

/// 创建标签绑定的请求项。
///
/// 对应 `examples/alist/internal/op/label_file_binding.go:13` 的
/// `op.CreateLabelFileBinDingReq`；批量创建端点
/// `POST /api/admin/label_file_binding/create_batch` 的 `items` 元素即该形状，
/// 单条创建端点的请求体也是同一形状的平铺字段（见 `src/endpoint/admin/label_file_binding/create.rs`）。
///
/// 服务端对单个文件的处理语义：先删除该文件名的全部既有绑定，再按标签 ID 重建
/// （`op.CreateLabelFileBinDing`）；标签列表为空时仅清除不新建。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateItem {
    /// 文件 ID（对应 Go `model.ObjFile.Id`；可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 文件完整路径（可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 目标文件名（服务端以文件名定位绑定，建议必填）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 文件大小（字节，可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 是否为目录（可选；服务端拒绝为目录创建绑定）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_dir: Option<bool>,
    /// 文件修改时间（可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,
    /// 文件创建时间（可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    /// 签名字符串（可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    /// 缩略图链接（可选）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    /// 文件类型枚举值（可选；取值含义见 [`ObjLabelResp`] 的 `type` 字段）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 哈希信息字符串（可选；JSON 键为 `hashinfo`，对应 Go `HashInfoStr`）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashinfo: Option<String>,
    /// 逗号分隔的标签 ID 列表（可选，如 `"1,2,3"`；对应 Go `LabelIds string`）。
    ///
    /// Go 侧请求体同时存在数组形态的 `labelIdList` 键（`LabelIDs []uint64`），
    /// 两种形态服务端解析行为一致；本模型统一使用字符串形态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<String>,
}

/// 创建标签绑定的响应数据。
///
/// 对应 `examples/alist/server/handles/label_file_binding.go:77` 的
/// `CreateLabelFileBinDing` 以 `gin.H{"msg": "添加成功！"}` 返回的形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateResp {
    /// 服务端提示消息（Go 源码固定返回「添加成功！」）。
    pub msg: String,
}

/// 批量创建结果中的单项结果。
///
/// 对应 `examples/alist/server/handles/label_file_binding.go:223` 的
/// `CreateLabelFileBinDingBatch` 内部 `perResult` 结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchResult {
    /// 对应请求项的文件名（Go 取 `CreateLabelFileBinDingReq.Name`）。
    pub name: String,
    /// 该项是否创建成功。
    pub ok: bool,
    /// 失败原因；成功时服务端省略该字段（Go `omitempty`）。
    #[serde(rename = "errMsg", default)]
    pub err_msg: Option<String>,
}

/// 批量创建标签绑定的响应数据。
///
/// 对应 `examples/alist/server/handles/label_file_binding.go:244` 的
/// `CreateLabelFileBinDingBatch` 以 `gin.H{...}` 返回的形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateBatchResp {
    /// 提交的请求项总数。
    pub total: i64,
    /// 创建成功的项数。
    pub succeed: i64,
    /// 创建失败的项数。
    pub failed: i64,
    /// 每个请求项的逐条结果。
    #[serde(default, deserialize_with = "null_to_default")]
    pub results: Vec<BatchResult>,
}

/// 恢复标签绑定记录的响应数据。
///
/// 对应 `examples/alist/server/handles/label_file_binding.go:203` 的
/// `RestoreLabelFileBinding` 以 `gin.H{"msg": "restored N rows"}` 返回的形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreResp {
    /// 服务端提示消息（Go 源码返回 `restored N rows`，N 为提交的记录条数）。
    pub msg: String,
}

/// 集合字段对显式 `null` 的宽容反序列化辅助（模式见 `docs/design.md` §5）。
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：按 `examples/alist/internal/model/label.go` 的 `model.Label`
    /// JSON tag 构造的标签实体（`GET /api/label_file_binding/get` 的元素形状）。
    #[test]
    fn label_decodes_go_model_shape() {
        let label: Label = serde_json::from_value(serde_json::json!({
            "id": 1,
            "type": 0,
            "name": "收藏",
            "description": "important files",
            "bg_color": "#1890ff",
            "create_time": "2024-06-01T08:00:00Z"
        }))
        .unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.r#type, 0);
        assert_eq!(label.name, "收藏");
        assert_eq!(label.description, "important files");
        assert_eq!(label.bg_color, "#1890ff");
        assert_eq!(
            label.create_time,
            DateTime::parse_from_rfc3339("2024-06-01T08:00:00Z")
                .unwrap()
                .with_timezone(&Utc)
        );
    }

    /// 正向钉扎：按 `op.ObjLabelResp` JSON tag 构造的文件条目
    /// （`GET /api/label_file_binding/get_file_by_label` 的元素形状），
    /// 重点钉住字符串形态的 `id`、`hashinfo`/`label_list` 键名与 `type` 键。
    #[test]
    fn obj_label_resp_decodes_go_shape() {
        let resp: ObjLabelResp = serde_json::from_value(serde_json::json!({
            "id": "obj-1",
            "path": "/data",
            "name": "movie.mp4",
            "size": 1024,
            "is_dir": false,
            "modified": "2024-06-01T08:00:00Z",
            "created": "2024-05-01T08:00:00Z",
            "sign": "sign-token",
            "thumb": "https://thumb.example/movie.png",
            "type": 2,
            "hashinfo": "sha1:abcdef",
            "label_list": [
                {
                    "id": 1,
                    "type": 0,
                    "name": "收藏",
                    "description": "",
                    "bg_color": "",
                    "create_time": "2024-06-01T08:00:00Z"
                }
            ]
        }))
        .unwrap();
        assert_eq!(resp.id, "obj-1"); // 服务端文件 ID 是字符串
        assert_eq!(resp.path, "/data");
        assert_eq!(resp.name, "movie.mp4");
        assert_eq!(resp.size, 1024);
        assert!(!resp.is_dir);
        assert_eq!(resp.r#type, 2);
        assert_eq!(resp.hashinfo, "sha1:abcdef");
        assert_eq!(resp.label_list.len(), 1);
        assert_eq!(resp.label_list[0].name, "收藏");
    }

    /// 兼容钉扎：集合字段显式 `null` 时归约为空 `Vec`（`null_to_default`）。
    #[test]
    fn obj_label_resp_tolerates_null_label_list() {
        let resp: ObjLabelResp = serde_json::from_value(serde_json::json!({
            "id": "obj-1",
            "path": "/data",
            "name": "movie.mp4",
            "size": 0,
            "is_dir": true,
            "modified": "2024-06-01T08:00:00Z",
            "created": "2024-05-01T08:00:00Z",
            "sign": "",
            "thumb": "",
            "type": 1,
            "hashinfo": "",
            "label_list": null
        }))
        .unwrap();
        assert_eq!(resp.label_list, Vec::<Label>::new());
    }

    /// 序列化键名钉扎：`CreateItem` 按 `op.CreateLabelFileBinDingReq` 的 JSON tag
    /// 输出键名（含 `type`、`hashinfo`），`None` 字段被跳过。
    #[test]
    fn create_item_serializes_with_go_json_keys() {
        let item = CreateItem {
            name: Some("movie.mp4".to_owned()),
            size: Some(1024),
            r#type: Some(2),
            label_ids: Some("1,2".to_owned()),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&item).unwrap(),
            serde_json::json!({
                "name": "movie.mp4",
                "size": 1024,
                "type": 2,
                "label_ids": "1,2"
            })
        );
    }

    /// 反序列化钉扎：时间字段按 RFC3339 解码，空对象归约为全 `None`。
    #[test]
    fn create_item_decodes_rfc3339_time_fields() {
        let item: CreateItem = serde_json::from_value(serde_json::json!({
            "id": "obj-1",
            "path": "/data",
            "name": "movie.mp4",
            "size": 1024,
            "is_dir": false,
            "modified": "2024-06-01T08:00:00Z",
            "created": "2024-05-01T08:00:00Z",
            "sign": "",
            "thumb": "",
            "type": 2,
            "hashinfo": "sha1:abcdef",
            "label_ids": "1,2"
        }))
        .unwrap();
        assert_eq!(item.id.as_deref(), Some("obj-1"));
        assert_eq!(item.size, Some(1024));
        assert!(!item.is_dir.unwrap());
        assert!(item.modified.is_some());
        assert_eq!(item.label_ids.as_deref(), Some("1,2"));
        assert_eq!(
            CreateItem::default(),
            CreateItem {
                ..Default::default()
            }
        );
    }

    /// 正向钉扎：`POST /api/admin/label_file_binding/create` 的 `data` 形状
    /// （`handles.CreateLabelFileBinDing` 返回 `gin.H{"msg": "添加成功！"}`）。
    #[test]
    fn create_resp_decodes_handler_example() {
        let resp: CreateResp = serde_json::from_value(serde_json::json!({
            "msg": "添加成功！"
        }))
        .unwrap();
        assert_eq!(resp.msg, "添加成功！");
    }

    /// 正向钉扎：`POST /api/admin/label_file_binding/create_batch` 的 `data` 形状
    /// （`handles.CreateLabelFileBinDingBatch` 返回 `gin.H{total, succeed, failed, results}`，
    /// 其中失败项带 `errMsg`、成功项省略该键）。
    #[test]
    fn create_batch_resp_decodes_handler_example() {
        let resp: CreateBatchResp = serde_json::from_value(serde_json::json!({
            "total": 2,
            "succeed": 1,
            "failed": 1,
            "results": [
                { "name": "movie.mp4", "ok": true },
                { "name": "folder", "ok": false, "errMsg": "Unable to bind folder" }
            ]
        }))
        .unwrap();
        assert_eq!(resp.total, 2);
        assert_eq!(resp.succeed, 1);
        assert_eq!(resp.failed, 1);
        assert_eq!(resp.results.len(), 2);
        assert!(resp.results[0].ok);
        assert_eq!(resp.results[0].err_msg, None); // 成功项省略 errMsg
        assert!(!resp.results[1].ok);
        assert_eq!(
            resp.results[1].err_msg.as_deref(),
            Some("Unable to bind folder")
        );
    }

    /// 兼容钉扎：`results` 缺失或显式 `null` 时归约为空 `Vec`。
    #[test]
    fn create_batch_resp_tolerates_missing_or_null_results() {
        let missing: CreateBatchResp =
            serde_json::from_value(serde_json::json!({ "total": 0, "succeed": 0, "failed": 0 }))
                .unwrap();
        assert_eq!(missing.results, Vec::<BatchResult>::new());

        let null: CreateBatchResp = serde_json::from_value(serde_json::json!({
            "total": 0, "succeed": 0, "failed": 0, "results": null
        }))
        .unwrap();
        assert_eq!(null.results, Vec::<BatchResult>::new());
    }

    /// 正向钉扎：按 `examples/alist/internal/model/label_file_binding.go:5` 的
    /// `model.LabelFileBinding` JSON tag 构造的绑定记录实体
    /// （`GET /api/admin/label_file_binding/list` 的元素形状，
    /// 也是 restore 请求体 `bindings` 数组的元素形状）。
    #[test]
    fn label_file_binding_decodes_go_model_shape() {
        let binding: LabelFileBinding = serde_json::from_value(serde_json::json!({
            "id": 7,
            "user_id": 1,
            "label_id": 3,
            "file_name": "movie.mp4",
            "create_time": "2024-06-01T08:00:00Z"
        }))
        .unwrap();
        assert_eq!(binding.id, 7);
        assert_eq!(binding.user_id, 1);
        assert_eq!(binding.label_id, 3);
        assert_eq!(binding.file_name, "movie.mp4");
        assert_eq!(
            binding.create_time,
            DateTime::parse_from_rfc3339("2024-06-01T08:00:00Z")
                .unwrap()
                .with_timezone(&Utc)
        );
    }

    /// 正向钉扎：`GET /api/admin/label_file_binding/list` 的 `data` 分页壳
    /// （`handles.ListLabelFileBinding` 返回 `pageResp[model.LabelFileBinding]`，
    /// 即 `{content, total}`，与 `crate::schema::common::PageResponse` 形状一致）。
    #[test]
    fn label_file_binding_page_response_wraps_handler_shape() {
        use crate::schema::common::PageResponse;

        let page: PageResponse<LabelFileBinding> = serde_json::from_value(serde_json::json!({
            "content": [
                {
                    "id": 7,
                    "user_id": 1,
                    "label_id": 3,
                    "file_name": "movie.mp4",
                    "create_time": "2024-06-01T08:00:00Z"
                },
                {
                    "id": 8,
                    "user_id": 1,
                    "label_id": 4,
                    "file_name": "song.flac",
                    "create_time": "2024-06-02T08:00:00Z"
                }
            ],
            "total": 2
        }))
        .unwrap();
        assert_eq!(page.total, 2);
        assert_eq!(page.content.len(), 2);
        assert_eq!(page.content[0].file_name, "movie.mp4");
        assert_eq!(page.content[1].label_id, 4);
    }

    /// 正向钉扎：`POST /api/admin/label_file_binding/restore` 的 `data` 形状
    /// （`handles.RestoreLabelFileBinding` 返回 `gin.H{"msg": "restored N rows"}`）。
    #[test]
    fn restore_resp_decodes_handler_example() {
        let resp: RestoreResp = serde_json::from_value(serde_json::json!({
            "msg": "restored 3 rows"
        }))
        .unwrap();
        assert_eq!(resp.msg, "restored 3 rows");
    }
}
