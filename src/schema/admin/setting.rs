//! admin-setting 设置域数据模型。
//!
//! 建模 `/api/admin/setting` 端点族的数据形状：
//!
//! - [`Setting`]：设置条目，同时用作 `list`/`get`/`get_by_keys` 的响应模型与
//!   `save` 的请求体元素，
//!   对应 `examples/alist/internal/model/setting.go` 的 `SettingItem`；
//! - [`SetAria2Request`] / [`SetQbitRequest`]：`set_aria2`/`set_qbit` 的扁平请求体形状，
//!   对应 `examples/alist/server/handles/offline_download.go` 的 Go `SetAria2Req`/`SetQbittorrentReq`；
//!   端点构建器（`set_aria2::Request`、`set_qbit::Request`）以同名字段直接生成同形 JSON。
//!
//! 字段形状以 `docs/api/alistv3.openapi.yaml` 的 `admin/setting` 分组与上述 Go 源码为准；
//! 注意 Go `SettingItem.Index uint`（JSON 键 `index`）未被 openapi 文档收录，但服务端会实际返回，
//! 这里以 `#[serde(default)]` 接住，保证对老版本服务端的兼容。

use serde::{Deserialize, Serialize};

/// AList 设置条目。
///
/// 对应 `examples/alist/internal/model/setting.go` 的 `SettingItem`：
/// `GET /api/admin/setting/list`、`GET /api/admin/setting/get` 的响应元素，
/// 同时是 `POST /api/admin/setting/save` 的请求体元素。
/// 设置值统一为字符串承载，实际类型由 [`Setting::value_type`] 描述。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
    /// 设置键（唯一标识），例如 `site_title`、`token`；对应 Go `SettingItem.Key`。
    pub key: String,
    /// 设置值；数值/布尔等一律以字符串形式存储，对应 Go `SettingItem.Value`。
    pub value: String,
    /// 帮助信息；对应 Go `SettingItem.Help`。
    pub help: String,
    /// 值类型：`string` / `number` / `bool` / `select`；
    /// 对应 Go `SettingItem.Type`（JSON 键为 `type`，Rust 字段名因此改名）。
    #[serde(rename = "type")]
    pub value_type: String,
    /// `select` 类型的候选项（逗号分隔）；对应 Go `SettingItem.Options`。
    pub options: String,
    /// 前端分组编号；对应 Go `SettingItem.Group`。取值参考 Go 常量
    /// （`internal/model/setting.go`）：`0`-单独项（SINGLE，含令牌等）、`1`-站点、
    /// `2`-样式、`3`-预览、`4`-全局、`5`-离线下载（含 aria2/qBittorrent 等）、
    /// `6`-索引、`7`-单点登录、`8`-LDAP、`9`-S3、`10`-FTP、`11`-流量、`12`-FRP。
    pub group: i32,
    /// 可见性标志；对应 Go `SettingItem.Flag`：`0`-公开、`1`-私有、`2`-只读、`3`-弃用。
    pub flag: i32,
    /// 排序序号；对应 Go `SettingItem.Index uint`（JSON 键 `index`）。
    /// openapi 文档未收录该字段，老版本服务端可能不返回，缺失时归零。
    #[serde(default)]
    pub index: u64,
}

/// `POST /api/admin/setting/set_aria2` 的请求体形状。
///
/// 对应 `examples/alist/server/handles/offline_download.go` 的 Go `SetAria2Req`
/// 与 `docs/api/alistv3.openapi.yaml` `admin/setting/set_aria2` 的请求体；
/// 仅含 `uri`/`secret` 两个字段（保存后服务端立即初始化 aria2 连接）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetAria2Request {
    /// aria2 JSON-RPC 地址，例如 `http://localhost:6800/jsonrpc`；对应 Go `SetAria2Req.Uri`。
    pub uri: String,
    /// aria2 RPC 密钥；未设置时为空字符串；对应 Go `SetAria2Req.Secret`。
    pub secret: String,
}

/// `POST /api/admin/setting/set_qbit` 的请求体形状。
///
/// 对应 `examples/alist/server/handles/offline_download.go` 的 `SetQbittorrentReq`
/// 与 `docs/api/alistv3.openapi.yaml` `admin/setting/set_qbit` 的请求体；
/// 仅含 `url`/`seedtime` 两个字段。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetQbitRequest {
    /// qBittorrent WebUI 地址（可内嵌凭据），例如 `http://user:pass@localhost:8080/`；
    /// 对应 Go `SetQbittorrentReq.Url`。
    pub url: String,
    /// 做种时间（字符串形式的数值）；对应 Go `SetQbittorrentReq.Seedtime`。
    pub seedtime: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` `admin/setting/list` 返回示例
    /// （`data` 为设置项数组，非分页结构）。
    #[test]
    fn setting_list_decodes_openapi_example() {
        let resp: Response<Vec<Setting>> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": [
                {
                    "key": "aria2_uri",
                    "value": "http://localhost:6800/jsonrpc",
                    "help": "",
                    "type": "string",
                    "options": "",
                    "group": 5,
                    "flag": 1
                },
                {
                    "key": "index_progress",
                    "value": "{\"obj_count\":0,\"is_done\":true,\"last_done_time\":null,\"error\":\"\"}",
                    "help": "",
                    "type": "text",
                    "options": "",
                    "group": 0,
                    "flag": 1
                }
            ]
        }))
        .unwrap();
        assert_eq!(resp.data.len(), 2);

        let aria2 = &resp.data[0];
        assert_eq!(aria2.key, "aria2_uri");
        assert_eq!(aria2.value, "http://localhost:6800/jsonrpc");
        assert_eq!(aria2.value_type, "string");
        assert_eq!(aria2.group, 5);
        assert_eq!(aria2.flag, 1);
        // openapi 示例未含 index 字段 → serde(default) 归零（老版本服务端兼容）
        assert_eq!(aria2.index, 0);

        let progress = &resp.data[1];
        assert_eq!(progress.value_type, "text");
        assert!(progress.value.contains("\"is_done\":true"));
    }

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` `admin/setting/get` 返回示例
    /// （`data` 为单个设置项；`value` 为正则文本）。
    #[test]
    fn setting_get_decodes_openapi_example() {
        let resp: Response<Setting> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "key": "hide_files",
                "value": "/\\/README.md/i",
                "help": "",
                "type": "text",
                "options": "",
                "group": 4,
                "flag": 0
            }
        }))
        .unwrap();
        assert_eq!(resp.data.key, "hide_files");
        assert_eq!(resp.data.value, "/\\/README.md/i");
        assert_eq!(resp.data.value_type, "text");
        assert_eq!(resp.data.group, 4);
        assert_eq!(resp.data.flag, 0);
    }

    /// 兼容钉扎：Go `SettingItem.Index uint`（`internal/model/setting.go`）会随响应返回，
    /// openapi 文档示例缺失该字段；两种形状（含/不含 `index`）都必须能解码。
    #[test]
    fn setting_tolerates_index_field_presence() {
        let with_index: Setting = serde_json::from_value(serde_json::json!({
            "key": "site_title",
            "value": "AList",
            "help": "",
            "type": "string",
            "options": "",
            "group": 1,
            "flag": 0,
            "index": 3
        }))
        .unwrap();
        assert_eq!(with_index.index, 3);

        let without_index: Setting = serde_json::from_value(serde_json::json!({
            "key": "site_title",
            "value": "AList",
            "help": "",
            "type": "string",
            "options": "",
            "group": 1,
            "flag": 0
        }))
        .unwrap();
        assert_eq!(without_index.index, 0);
    }

    /// 序列化键名钉扎：`type` 键经 `#[serde(rename)]` 保留，`index` 一并序列化
    /// （服务端 `ShouldBind` 对缺失字段的绑定结果与显式 `0` 等价）。
    #[test]
    fn setting_serializes_with_api_field_names() {
        let setting = Setting {
            key: "site_title".to_owned(),
            value: "AList".to_owned(),
            help: String::new(),
            value_type: "string".to_owned(),
            options: String::new(),
            group: 1,
            flag: 0,
            index: 0,
        };
        assert_eq!(
            serde_json::to_value(&setting).unwrap(),
            serde_json::json!({
                "key": "site_title",
                "value": "AList",
                "help": "",
                "type": "string",
                "options": "",
                "group": 1,
                "flag": 0,
                "index": 0
            })
        );
    }

    /// 序列化键名钉扎：`docs/api/alistv3.openapi.yaml` `admin/setting/set_aria2`
    /// 请求体（`uri`/`secret`，无其它字段）。
    #[test]
    fn set_aria2_request_serializes_with_api_field_names() {
        let req = SetAria2Request {
            uri: "http://localhost:6800/jsonrpc".to_owned(),
            secret: String::new(),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "uri": "http://localhost:6800/jsonrpc",
                "secret": ""
            })
        );
    }

    /// 序列化键名钉扎：`docs/api/alistv3.openapi.yaml` `admin/setting/set_qbit`
    /// 请求体（`url`/`seedtime`，无其它字段）。
    #[test]
    fn set_qbit_request_serializes_with_api_field_names() {
        let req = SetQbitRequest {
            url: "http://user:pass@localhost:8080/".to_owned(),
            seedtime: "30".to_owned(),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "url": "http://user:pass@localhost:8080/",
                "seedtime": "30"
            })
        );
    }
}
