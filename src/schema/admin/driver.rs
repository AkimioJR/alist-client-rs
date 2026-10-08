//! admin-driver 驱动域数据模型。
//!
//! 覆盖 `/api/admin/driver` 下的三个端点：
//! - `GET /api/admin/driver/list`：全部驱动配置模板映射 [`DriverListResponse`]（宽松原始 JSON 模型）；
//! - `GET /api/admin/driver/names`：驱动名列表 [`DriverNamesResponse`]；
//! - `GET /api/admin/driver/info`：单个驱动配置模板 [`DriverInfo`]。
//!
//! 字段形状来源：`docs/api/alistv3.openapi.yaml` 的 `admin/driver` 分组、
//! `docs/api/alistv3.md` 的 `# admin/driver` 返回示例，以及
//! `examples/alist/internal/driver/item.go`（`driver.Item`/`driver.Info`）、
//! `examples/alist/internal/driver/config.go`（`driver.Config`）、
//! `examples/alist/internal/op/driver.go`（模板组装逻辑）与
//! `examples/alist/server/handles/driver.go`（处理函数）。
//!
//! ## 宽松模型说明
//!
//! list 端点的 `data` 是「驱动名 → 驱动模板」映射：键为任意驱动名（如 `Local`、
//! `115 Cloud`），模板内容随驱动种类与 AList 版本演进（例如配置项陆续新增
//! `proxy_range`、`disable_index`、`show_when` 等），因此按仓库约定保持
//! `HashMap<String, serde_json::Value>` 的宽松形状；每个条目的实际形状与
//! [`DriverInfo`]（`common`/`additional`/`config`）一致，需要强类型视图时可对
//! 单个条目执行 `serde_json::from_value::<DriverInfo>` 转换。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// `GET /api/admin/driver/list` 的 `data`：全部驱动配置模板，以驱动名为键。
///
/// 宽松模型（`HashMap<String, serde_json::Value>`）：值为驱动模板的原始 JSON，
/// 形状与 [`DriverInfo`] 一致，可用 `serde_json::from_value` 按需转为强类型。
pub type DriverListResponse = HashMap<String, serde_json::Value>;

/// `GET /api/admin/driver/names` 的 `data`：已注册驱动的名称列表。
pub type DriverNamesResponse = Vec<String>;

/// 单个驱动的配置模板，`GET /api/admin/driver/info` 的 `data`。
///
/// 对应 `examples/alist/internal/driver/item.go` 的 `driver.Info`：
/// [`common`](DriverInfo::common) 为通用配置项（挂载路径、缓存策略等，由
/// `internal/op/driver.go` 的 `getMainItems` 生成），[`additional`](DriverInfo::additional)
/// 为驱动专有配置项（由各驱动 `Addition` 结构体反射生成），
/// [`config`](DriverInfo::config) 为驱动行为开关。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverInfo {
    /// 通用配置项列表（对应 Go `driver.Info.Common`）。
    #[serde(default, deserialize_with = "null_to_default")]
    pub common: Vec<DriverItem>,
    /// 驱动专有配置项列表（对应 Go `driver.Info.Additional`）；
    /// 驱动无专有配置时服务端可能返回 `null` 或空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub additional: Vec<DriverItem>,
    /// 驱动行为配置（对应 Go `driver.Info.Config`）。
    #[serde(default)]
    pub config: DriverConfig,
}

/// 驱动配置项描述，即存储表单中的一个字段。
///
/// 对应 `examples/alist/internal/driver/item.go` 的 `driver.Item`；
/// [`default`](DriverItem::default)/[`options`](DriverItem::options) 等均为
/// 字符串形式，由前端按 [`value_type`](DriverItem::value_type) 解释。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverItem {
    /// 配置项名，如 `mount_path`、`root_folder_id`（对应 Go `Item.Name`）。
    pub name: String,
    /// 值类型：`string`/`number`/`bool`/`text`/`select` 等。
    ///
    /// JSON 键为保留字 `type`（对应 Go `Item.Type`），故重命名为 `value_type`。
    #[serde(rename = "type")]
    pub value_type: String,
    /// 默认值（字符串形式，如 `"30"`、`"302_redirect"`）。
    #[serde(default)]
    pub default: String,
    /// `select` 类型的候选项，逗号分隔（如 `front,back`）。
    #[serde(default)]
    pub options: String,
    /// 存储表单中是否必填。
    #[serde(default)]
    pub required: bool,
    /// 帮助文本。
    #[serde(default)]
    pub help: String,
    /// 表单展示条件，如 `auth_mode=token`；为空或缺失表示总是展示。
    ///
    /// 服务端新增字段（对应 Go `Item.ShowWhen`，带 `omitempty`，老版本不返回）。
    #[serde(default)]
    pub show_when: Option<String>,
}

/// 驱动行为配置开关。
///
/// 对应 `examples/alist/internal/driver/config.go` 的 `driver.Config`；
/// `CheckStatus`/`NoOverwriteUpload`/`ProxyRangeOption` 三个字段标记 `json:"-"`，
/// 不会出现在响应中，故不建模。
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DriverConfig {
    /// 驱动名，与模板映射的键一致。
    #[serde(default)]
    pub name: String,
    /// 是否本地排序。
    #[serde(default)]
    pub local_sort: bool,
    /// 是否仅本地访问（不走代理）。
    #[serde(default)]
    pub only_local: bool,
    /// 是否仅代理访问。
    #[serde(default)]
    pub only_proxy: bool,
    /// 是否禁用缓存。
    #[serde(default)]
    pub no_cache: bool,
    /// 是否禁止上传。
    #[serde(default)]
    pub no_upload: bool,
    /// 是否需要用户交互消息（如验证码）。
    #[serde(default)]
    pub need_ms: bool,
    /// 默认根路径。
    #[serde(default)]
    pub default_root: String,
    /// 警告信息，取值如 `info`/`success`/`warning`/`danger`。
    #[serde(default)]
    pub alert: String,
}

/// 集合字段显式 `null` 归约为默认值（服务端对空/新增集合字段可能返回 `null`）。
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

    /// 正向钉扎：`docs/api/alistv3.md` `GET 列出特定驱动信息` 的 UC 驱动返回示例（节选，
    /// 字段值逐一取自原文），逐字段断言 [`DriverInfo`] 解码形状。
    #[test]
    fn driver_info_decodes_openapi_uc_example() {
        let info: DriverInfo = serde_json::from_value(serde_json::json!({
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "UC",
                "local_sort": false,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        }))
        .unwrap();
        assert_eq!(info.common[0].name, "mount_path");
        assert_eq!(info.common[0].value_type, "string");
        assert!(info.common[0].required);
        assert_eq!(
            info.common[0].help,
            "The path you want to mount to, it is unique and cannot be repeated"
        );
        assert_eq!(info.common[1].value_type, "select");
        assert_eq!(info.common[1].options, "use_proxy_url,native_proxy");
        assert_eq!(info.additional[0].name, "root_folder_id");
        assert_eq!(info.additional[0].default, "0");
        assert_eq!(info.config.name, "UC");
        assert!(info.config.only_local);
        assert!(!info.config.no_upload);
        assert_eq!(info.config.default_root, "0");
        assert_eq!(info.config.alert, "");
    }

    /// 兼容钉扎：`additional` 显式 `null`（Go `getAdditionalItems` 对无配置项驱动返回
    /// nil 切片，序列化为 `null`）、`config` 缺失字段、`show_when` 缺失时均不失败。
    #[test]
    fn driver_info_tolerates_null_and_missing_fields() {
        let info: DriverInfo = serde_json::from_value(serde_json::json!({
            "common": [{
                "name": "mount_path",
                "type": "string",
                "default": "",
                "options": "",
                "required": true,
                "help": ""
            }],
            "additional": null,
            "config": { "name": "Local" }
        }))
        .unwrap();
        assert_eq!(info.additional, Vec::<DriverItem>::new());
        assert_eq!(info.config.name, "Local");
        assert!(!info.config.only_local);
        assert_eq!(info.config.default_root, "");
        assert_eq!(info.common[0].show_when, None);
    }

    /// 新增字段钉扎：`show_when` 来自 `examples/alist/internal/driver/item.go`
    /// 的 `Item.ShowWhen`（文档 openapi 示例尚未收录该字段），非空时应解出。
    #[test]
    fn driver_item_decodes_show_when_from_go_source() {
        let item: DriverItem = serde_json::from_value(serde_json::json!({
            "name": "refresh_token",
            "type": "string",
            "default": "",
            "options": "",
            "required": true,
            "help": "",
            "show_when": "auth_mode=token"
        }))
        .unwrap();
        assert_eq!(item.show_when.as_deref(), Some("auth_mode=token"));
    }

    /// 序列化键名钉扎：`type` 为保留字，序列化必须还原为 JSON 键 `type`。
    #[test]
    fn driver_item_serializes_with_api_field_names() {
        let item = DriverItem {
            name: "webdav_policy".to_owned(),
            value_type: "select".to_owned(),
            default: "302_redirect".to_owned(),
            options: "302_redirect,use_proxy_url,native_proxy".to_owned(),
            required: true,
            help: String::new(),
            show_when: None,
        };
        let value = serde_json::to_value(&item).unwrap();
        assert_eq!(value["name"], "webdav_policy");
        assert_eq!(value["type"], "select");
        assert!(value.get("value_type").is_none(), "不应出现重命名后的键");
    }

    /// 宽松模型钉扎：`docs/api/alistv3.md` `GET 查询所有驱动配置模板列表` 的
    /// `115 Cloud` 条目（节选，字段值取自原文）可解为 [`DriverListResponse`]，
    /// 且单个条目可按需转换为强类型 [`DriverInfo`]。
    #[test]
    fn driver_list_response_decodes_openapi_example_and_bridges_to_driver_info() {
        let list: DriverListResponse = serde_json::from_value(serde_json::json!({
            "115 Cloud": {
                "common": [
                    {
                        "name": "mount_path",
                        "type": "string",
                        "default": "",
                        "options": "",
                        "required": true,
                        "help": "The path you want to mount to, it is unique and cannot be repeated"
                    }
                ],
                "additional": [
                    {
                        "name": "page_size",
                        "type": "number",
                        "default": "56",
                        "options": "",
                        "required": false,
                        "help": "list api per page size of 115 driver"
                    }
                ],
                "config": {
                    "name": "115 Cloud",
                    "local_sort": false,
                    "only_local": false,
                    "only_proxy": false,
                    "no_cache": false,
                    "no_upload": false,
                    "need_ms": false,
                    "default_root": "0",
                    "alert": ""
                }
            }
        }))
        .unwrap();
        assert!(list.contains_key("115 Cloud"));
        assert_eq!(list["115 Cloud"]["config"]["name"], "115 Cloud");

        let info: DriverInfo = serde_json::from_value(list["115 Cloud"].clone()).unwrap();
        assert_eq!(info.config.name, "115 Cloud");
        assert_eq!(info.additional[0].name, "page_size");
        assert_eq!(info.additional[0].default, "56");
    }

    /// 正向钉扎：`docs/api/alistv3.md` `GET /api/admin/driver/names` 返回示例（节选）。
    #[test]
    fn driver_names_response_decodes_openapi_example() {
        let names: DriverNamesResponse =
            serde_json::from_value(serde_json::json!(["Local", "115 Cloud", "AliyundriveOpen"]))
                .unwrap();
        assert_eq!(
            names,
            vec![
                "Local".to_owned(),
                "115 Cloud".to_owned(),
                "AliyundriveOpen".to_owned()
            ]
        );
    }
}
