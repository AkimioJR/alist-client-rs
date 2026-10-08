//! admin-setting 设置域数据模型。
//!
//! 建模 `/api/admin/setting` 端点族的数据形状：
//!
//! - [`Setting`]：设置条目，同时用作 `list`/`get`/`get_by_keys` 的响应模型与
//!   `save` 的请求体元素，
//!   对应 AList 服务端 model.Setting 数据模型中的 `SettingItem`；
//! - [`SetAria2Request`] / [`SetQbitRequest`]：`set_aria2`/`set_qbit` 的扁平请求体形状，
//!   对应 AList 服务端离线下载处理模块的请求结构；
//!   端点构建器（`set_aria2::Request`、`set_qbit::Request`）以同名字段直接生成同形 JSON。
//!
//! ## 字段形状来源
//!
//! 字段形状以 AList OpenAPI 规范的 admin/setting 分组与 AList 服务端源码为准；
//! 注意设置项 `index` 字段未被 OpenAPI 规范收录，但服务端会实际返回，
//! 这里以 `#[serde(default)]` 接住，保证对老版本服务端的兼容。

use serde::{Deserialize, Serialize};

/// 设置项可见性与权限标志。
///
/// 对应 AList 服务端常量（`internal/model/setting.go:21-26`）：
/// - `0`: 公开（公开 API 可见）；
/// - `1`: 私有（仅管理员可见）；
/// - `2`: 只读（不可在前端修改）；
/// - `3`: 弃用。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SettingFlag {
    /// 公开（默认值，对应整数 `0`）。
    #[default]
    Public = 0,
    /// 私有（仅管理员可见，对应整数 `1`）。
    Private = 1,
    /// 只读（对应整数 `2`）。
    Readonly = 2,
    /// 弃用（对应整数 `3`）。
    Deprecated = 3,
}

impl SettingFlag {
    /// 是否为已弃用的设置项。
    #[inline]
    #[must_use]
    pub const fn is_deprecated(self) -> bool {
        matches!(self, Self::Deprecated)
    }

    /// 转换为对应整数值。
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
}

impl From<SettingFlag> for i32 {
    #[inline]
    fn from(flag: SettingFlag) -> Self {
        flag.as_i32()
    }
}

impl Serialize for SettingFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.as_i32())
    }
}

impl<'de> Deserialize<'de> for SettingFlag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i32::deserialize(deserializer)?;
        match val {
            0 => Ok(Self::Public),
            1 => Ok(Self::Private),
            2 => Ok(Self::Readonly),
            3 => Ok(Self::Deprecated),
            other => Err(serde::de::Error::custom(format!(
                "invalid setting flag {other}, expected 0 (public), 1 (private), 2 (readonly), or 3 (deprecated)"
            ))),
        }
    }
}

/// 设置项前端分组编号。
///
/// 对应 AList 服务端常量（`internal/model/setting.go:3-19`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingGroup {
    /// 独立单项（如访问令牌等，对应整数 `0`）。
    Single,
    /// 站点设置（对应整数 `1`）。
    Site,
    /// 样式设置（对应整数 `2`）。
    Style,
    /// 预览设置（对应整数 `3`）。
    Preview,
    /// 全局设置（对应整数 `4`）。
    Global,
    /// 离线下载（对应整数 `5`）。
    OfflineDownload,
    /// 搜索索引（对应整数 `6`）。
    Index,
    /// 单点登录（对应整数 `7`）。
    Sso,
    /// LDAP 认证（对应整数 `8`）。
    Ldap,
    /// S3 存储网关（对应整数 `9`）。
    S3,
    /// FTP 服务（对应整数 `10`）。
    Ftp,
    /// 流量控制（对应整数 `11`）。
    Traffic,
    /// FRP 内网穿透（对应整数 `12`）。
    Frp,
    /// 未知分组（向后兼容未来新增分组）。
    Unknown(i32),
}

impl Default for SettingGroup {
    #[inline]
    fn default() -> Self {
        Self::Single
    }
}

impl SettingGroup {
    /// 转换为对应整数值。
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Single => 0,
            Self::Site => 1,
            Self::Style => 2,
            Self::Preview => 3,
            Self::Global => 4,
            Self::OfflineDownload => 5,
            Self::Index => 6,
            Self::Sso => 7,
            Self::Ldap => 8,
            Self::S3 => 9,
            Self::Ftp => 10,
            Self::Traffic => 11,
            Self::Frp => 12,
            Self::Unknown(n) => n,
        }
    }
}

impl From<SettingGroup> for i32 {
    #[inline]
    fn from(group: SettingGroup) -> Self {
        group.as_i32()
    }
}

impl From<i32> for SettingGroup {
    #[inline]
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Single,
            1 => Self::Site,
            2 => Self::Style,
            3 => Self::Preview,
            4 => Self::Global,
            5 => Self::OfflineDownload,
            6 => Self::Index,
            7 => Self::Sso,
            8 => Self::Ldap,
            9 => Self::S3,
            10 => Self::Ftp,
            11 => Self::Traffic,
            12 => Self::Frp,
            other => Self::Unknown(other),
        }
    }
}

impl Serialize for SettingGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.as_i32())
    }
}

impl<'de> Deserialize<'de> for SettingGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i32::deserialize(deserializer)?;
        Ok(Self::from(val))
    }
}

/// 设置项分组编号集合（用于 `list` 端点的 `groups` 查询参数）。
///
/// 内部自动格式化为 AList 服务端期望的逗号分隔字符串（如 `"5,0"`）。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SettingGroups(String);

impl SettingGroups {
    /// 以逗号分隔的字符串直接构造。
    #[inline]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// 从 [`SettingGroup`] 迭代器构造逗号分隔的分组集合。
    pub fn from_groups(groups: impl IntoIterator<Item = SettingGroup>) -> Self {
        let s = groups
            .into_iter()
            .map(|g| g.as_i32().to_string())
            .collect::<Vec<_>>()
            .join(",");
        Self(s)
    }

    /// 返回底层逗号分隔的字符串切片。
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SettingGroups {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for SettingGroups {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::fmt::Display for SettingGroups {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for SettingGroups {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SettingGroups {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}

impl From<&str> for SettingGroups {
    #[inline]
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for SettingGroups {
    #[inline]
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&[SettingGroup]> for SettingGroups {
    #[inline]
    fn from(groups: &[SettingGroup]) -> Self {
        Self::from_groups(groups.iter().copied())
    }
}

impl<const N: usize> From<[SettingGroup; N]> for SettingGroups {
    #[inline]
    fn from(groups: [SettingGroup; N]) -> Self {
        Self::from(groups.as_slice())
    }
}

impl From<Vec<SettingGroup>> for SettingGroups {
    #[inline]
    fn from(groups: Vec<SettingGroup>) -> Self {
        Self::from(groups.as_slice())
    }
}

/// AList 设置条目。
///
/// 对应 AList 服务端 model.Setting 数据模型中的 `SettingItem`：
/// `GET /api/admin/setting/list`、`GET /api/admin/setting/get` 的响应元素，
/// 同时是 `POST /api/admin/setting/save` 的请求体元素。
/// 设置值统一为字符串承载，实际类型由 [`Setting::value_type`] 描述。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
    /// 设置键（唯一标识），例如 `site_title`、`token`。
    pub key: String,
    /// 设置值。
    ///
    /// 数值/布尔等一律以字符串形式存储。
    pub value: String,
    /// 帮助信息。
    pub help: String,
    /// 值类型：`string` / `number` / `bool` / `select`。
    ///
    /// 对应 JSON 键为 `type`，Rust 字段名因此改名。
    #[serde(rename = "type")]
    pub value_type: String,
    /// `select` 类型的候选项（逗号分隔）。
    pub options: String,
    /// 前端分组。
    pub group: SettingGroup,
    /// 可见性标志。
    pub flag: SettingFlag,
    /// 排序序号。
    ///
    /// OpenAPI 规范未收录该字段，老版本服务端可能不返回，缺失时归零。
    #[serde(default)]
    pub index: u64,
}

/// `POST /api/admin/setting/set_aria2` 的请求体形状。
///
/// 对应 AList 服务端离线下载处理模块与 AList OpenAPI 规范 `admin/setting/set_aria2` 的请求体；
/// 仅含 `uri`/`secret` 两个字段（保存后服务端立即初始化 aria2 连接）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetAria2Request {
    /// aria2 JSON-RPC 地址，例如 `http://localhost:6800/jsonrpc`。
    pub uri: String,
    /// aria2 RPC 密钥。
    ///
    /// 未设置时为空字符串。
    pub secret: String,
}

/// `POST /api/admin/setting/set_qbit` 的请求体形状。
///
/// 对应 AList 服务端离线下载处理模块与 AList OpenAPI 规范 `admin/setting/set_qbit` 的请求体；
/// 仅含 `url`/`seedtime` 两个字段。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetQbitRequest {
    /// qBittorrent WebUI 地址（可内嵌凭据），例如 `http://user:pass@localhost:8080/`。
    pub url: String,
    /// 做种时间（字符串形式的数值）。
    pub seedtime: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 正向钉扎：AList OpenAPI 规范 `admin/setting/list` 返回示例
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
        assert_eq!(aria2.group, SettingGroup::OfflineDownload);
        assert_eq!(aria2.flag, SettingFlag::Private);
        // openapi 示例未含 index 字段 → serde(default) 归零（老版本服务端兼容）
        assert_eq!(aria2.index, 0);

        let progress = &resp.data[1];
        assert_eq!(progress.value_type, "text");
        assert!(progress.value.contains("\"is_done\":true"));
    }

    /// 正向钉扎：AList OpenAPI 规范 `admin/setting/get` 返回示例
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
        assert_eq!(resp.data.group, SettingGroup::Global);
        assert_eq!(resp.data.flag, SettingFlag::Public);
    }

    /// 兼容钉扎：服务端设置项的 `index` 字段会随响应返回，
    /// OpenAPI 规范示例缺失该字段；两种形状（含/不含 `index`）都必须能解码。
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
            group: SettingGroup::Site,
            flag: SettingFlag::Public,
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

    /// 序列化键名钉扎：AList OpenAPI 规范 `admin/setting/set_aria2`
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

    /// 序列化键名钉扎：AList OpenAPI 规范 `admin/setting/set_qbit`
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

    /// 钉扎：`SettingFlag` 与 `SettingGroup` 的序列化、反序列化及未知值向前兼容。
    #[test]
    fn setting_enums_serde_matches_contract() {
        assert_eq!(SettingFlag::default(), SettingFlag::Public);
        assert!(!SettingFlag::Public.is_deprecated());
        assert!(SettingFlag::Deprecated.is_deprecated());
        assert_eq!(SettingFlag::Private.as_i32(), 1);
        assert_eq!(i32::from(SettingFlag::Readonly), 2);

        assert_eq!(
            serde_json::to_value(SettingFlag::Private).unwrap(),
            serde_json::json!(1)
        );
        assert_eq!(
            serde_json::from_value::<SettingFlag>(serde_json::json!(1)).unwrap(),
            SettingFlag::Private
        );
        assert!(serde_json::from_value::<SettingFlag>(serde_json::json!(99)).is_err());

        assert_eq!(SettingGroup::default(), SettingGroup::Single);
        assert_eq!(SettingGroup::Site.as_i32(), 1);
        assert_eq!(SettingGroup::Frp.as_i32(), 12);
        assert_eq!(SettingGroup::Unknown(99).as_i32(), 99);
        assert_eq!(SettingGroup::from(5), SettingGroup::OfflineDownload);
        assert_eq!(SettingGroup::from(99), SettingGroup::Unknown(99));

        assert_eq!(
            serde_json::to_value(SettingGroup::OfflineDownload).unwrap(),
            serde_json::json!(5)
        );
        assert_eq!(
            serde_json::from_value::<SettingGroup>(serde_json::json!(5)).unwrap(),
            SettingGroup::OfflineDownload
        );
        // 向前兼容：未知分组不报错，解码为 Unknown(99)
        assert_eq!(
            serde_json::from_value::<SettingGroup>(serde_json::json!(99)).unwrap(),
            SettingGroup::Unknown(99)
        );
    }

    /// 钉扎：`SettingGroups` 从切片、数组、字符串构造并序列化为逗号分隔字符串。
    #[test]
    fn setting_groups_conversions_and_serde_match_contract() {
        let from_slice =
            SettingGroups::from([SettingGroup::OfflineDownload, SettingGroup::Single].as_slice());
        assert_eq!(from_slice.as_str(), "5,0");
        assert_eq!(&*from_slice, "5,0");

        let from_array = SettingGroups::from([SettingGroup::OfflineDownload, SettingGroup::Single]);
        assert_eq!(from_array.as_str(), "5,0");

        let from_vec = SettingGroups::from(vec![SettingGroup::Site, SettingGroup::Style]);
        assert_eq!(from_vec.as_str(), "1,2");

        let from_str: SettingGroups = "5,0".into();
        assert_eq!(from_str.as_str(), "5,0");

        assert_eq!(
            serde_json::to_value(&from_array).unwrap(),
            serde_json::json!("5,0")
        );
        assert_eq!(
            serde_json::from_value::<SettingGroups>(serde_json::json!("5,0"))
                .unwrap()
                .as_str(),
            "5,0"
        );
    }
}
