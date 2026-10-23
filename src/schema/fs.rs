//! fs 文件系统域数据模型。
//!
//! 覆盖 `fs` 端点组的三个子域：
//!
//! - **文件系统**：目录列表、文件详情、目录树、搜索与目录/重命名/移动/复制/删除/
//!   离线下载等管理操作（`/api/fs/*`）；
//! - **归档（archive）**：压缩包元信息、内部列表与解压（`/api/fs/archive/*`），
//!   模型供归档端点代理直接复用；
//! - **上传（upload）**：`PUT /api/fs/put` / `PUT /api/fs/form` 的响应模型
//!   [`UploadResult`]（上传请求载荷全部位于
//!   HTTP 头与原始 body，无 JSON 请求模型）。
//!
//! ## 字段形状来源
//!
//! 字段形状以 AList OpenAPI 规范的 fs 分组与 AList 服务端各业务模块为准：
//! 包括 fsread 模块（列表/详情/目录树）、fsmanage 模块
//! （新建/重命名/移动/复制/删除）、fsbatch 模块（批量/正则重命名、
//! 聚合移动）、search 模块与 model.Search 数据模型（搜索）、
//! offline_download 模块（离线下载）、archive 模块
//! 与 model.Archive 数据模型（归档）、fsup 模块（上传）。
//!
//! ## 跨版本兼容性说明
//!
//! 新版服务端在 fs 列表/详情对象上陆续新增了 `id`/`path`/`virtual_path`/
//! `hashinfo`/`storage_class` 与 `filtered_total`/`page`/`per_page`/`has_more`/
//! `pages_total` 等标量字段：
//! - **标量演进字段**：统一采用 [`Option<T>`] 建模并在字段文档中注明引入的 Git Tag，
//!   老版本缺失时反序列化为 [`None`]，严格区分「字段不存在」与「有效零值」；
//! - **集合映射字段**：`hash_info` 与 `label_list` 在老版本缺失或返回显式 `null` 时，
//!   配合 `null_to_default` 归约为空集合，保持遍历人体工程学。

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::schema::common::TaskInfo;

/// 将显式 `null` 归约为 `T` 的默认值（老版本服务端可能对新字段返回 `null`）。
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// 兼容单字符串与字符串数组两种 JSON 形状（对应 Go `handles.StringOrArray`，
/// AList 服务端 archive 模块的自定义反序列化逻辑）。
fn deserialize_string_or_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrArrayInner {
        /// 单个字符串，服务端归约为单元素数组。
        One(String),
        /// 字符串数组。
        Many(Vec<String>),
    }
    Ok(match StringOrArrayInner::deserialize(deserializer)? {
        StringOrArrayInner::One(value) => vec![value],
        StringOrArrayInner::Many(values) => values,
    })
}

/// 文件/目录对象（无标签版本）。
///
/// 对应 AList 服务端 fsread 模块的 `ObjResponse`，
/// 同时作为归档内部列表条目（archive 模块嵌入字段）。
/// `id`/`path`/`virtual_path`/`hashinfo`/`hash_info`/`storage_class` 为新版服务端
/// 新增字段，老版本服务端不返回对应字段时为 [`None`] 或空集合。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FsObject {
    /// 对象 ID（对应 Go `ObjResponse.Id`）。
    ///
    /// 从 AList `v3.54.0` 起新增；老版本服务端不返回该字段时为 [`None`]。
    #[serde(default)]
    pub id: Option<String>,
    /// 对象在存储内的实际路径（对应 Go `ObjResponse.Path`）。
    ///
    /// 从 AList `v3.54.0` 起新增底层实际存储路径；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub path: Option<String>,
    /// 对象在站点内的展示路径（对应 Go `ObjResponse.VirtualPath`）。
    ///
    /// 从 AList `v3.61.0` 起新增规范虚拟展示路径；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub virtual_path: Option<String>,
    /// 文件/目录名。
    pub name: String,
    /// 大小（字节）。
    pub size: i64,
    /// 是否为目录。
    pub is_dir: bool,
    /// 修改时间（对应 Go `time.Time`）。
    pub modified: DateTime<Utc>,
    /// 创建时间；openapi 未将其列入必填（老版本可能缺失），归约为 `None`。
    #[serde(default)]
    pub created: Option<DateTime<Utc>>,
    /// 下载签名。
    ///
    /// 用于构造带签名的直链。
    pub sign: String,
    /// 缩略图地址。
    pub thumb: String,
    /// 文件类型枚举值（对应 Go `utils.GetObjType`，JSON 键为保留字 `type`）。
    pub r#type: i32,
    /// 哈希信息字符串（对应 Go `ObjResponse.HashInfoStr`）。
    ///
    /// 从 AList `v3.46.0` 起新增哈希信息字符串；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub hashinfo: Option<String>,
    /// 结构化哈希信息；键为哈希算法名（如 `md5`/`sha1`/`sha256`），
    /// 对应 Go `ObjResponse.HashInfo`（`map[*utils.HashType]string`）。
    ///
    /// 从 AList `v3.28.0` 起新增；老版本不返回或返回 `null`，归约为空映射。
    #[serde(default, deserialize_with = "null_to_default")]
    pub hash_info: HashMap<String, String>,
    /// 存储类型标识（对应 Go `ObjResponse.StorageClass`，`omitempty`）。
    ///
    /// 从 AList `v3.54.0` 起新增；缺失归约为 [`None`]。
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// 文件对象携带的标签条目。
///
/// 对应 AList 服务端 model.Label 数据模型，
/// 出现在目录列表条目的 `label_list` 字段中。
/// 与 admin 域的标签模型形状一致，但独立定义以避免 feature 交叉依赖。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjLabel {
    /// 标签 ID（对应 Go `model.Label.ID`，gorm 主键 `uint`）。
    pub id: u64,
    /// 标签类型（对应 Go `model.Label.Type`，JSON 键为保留字 `type`）。
    pub r#type: i32,
    /// 标签名称。
    pub name: String,
    /// 标签描述；缺失或 `null` 时归约为空串。
    #[serde(default, deserialize_with = "null_to_default")]
    pub description: String,
    /// 标签背景色；缺失或 `null` 时归约为空串。
    #[serde(default, deserialize_with = "null_to_default")]
    pub bg_color: String,
    /// 创建时间（对应 Go `model.Label.CreateTime`）。
    pub create_time: DateTime<Utc>,
}

/// 目录列表条目（带标签版本）。
///
/// 对应 AList 服务端 fsread 模块的 `ObjLabelResponse`，
/// 即 `/api/fs/list` 响应 `content` 的元素类型；`label_list` 与新增的
/// `id`/`path`/`virtual_path`/`storage_class` 在老版本服务端缺失时对应新字段为 [`None`] 或空集合。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabeledFsObject {
    /// 对象 ID（对应 Go `ObjLabelResponse.Id`）。
    ///
    /// 从 AList `v3.54.0` 起新增；老版本服务端不返回该字段时为 [`None`]。
    #[serde(default)]
    pub id: Option<String>,
    /// 对象在存储内的实际路径。
    ///
    /// 从 AList `v3.54.0` 起新增底层实际存储路径；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub path: Option<String>,
    /// 对象在站点内的展示路径。
    ///
    /// 从 AList `v3.61.0` 起新增规范虚拟展示路径；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub virtual_path: Option<String>,
    /// 文件/目录名。
    pub name: String,
    /// 大小（字节）。
    pub size: i64,
    /// 是否为目录。
    pub is_dir: bool,
    /// 修改时间。
    pub modified: DateTime<Utc>,
    /// 创建时间；老版本可能缺失，归约为 `None`。
    #[serde(default)]
    pub created: Option<DateTime<Utc>>,
    /// 下载签名。
    pub sign: String,
    /// 缩略图地址。
    pub thumb: String,
    /// 文件类型枚举值（JSON 键为保留字 `type`）。
    pub r#type: i32,
    /// 哈希信息字符串。
    ///
    /// 从 AList `v3.46.0` 起新增哈希信息字符串；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub hashinfo: Option<String>,
    /// 结构化哈希信息；键为哈希算法名（如 `md5`/`sha1`）；缺失或 `null` 归约为空映射。
    ///
    /// 从 AList `v3.28.0` 起新增；老版本不返回或返回 `null`，归约为空映射。
    #[serde(default, deserialize_with = "null_to_default")]
    pub hash_info: HashMap<String, String>,
    /// 文件绑定的标签列表（对应 Go `ObjLabelResponse.LabelList`）；目录恒为空，
    /// 老版本不返回或返回 `null`，归约为空数组。
    ///
    /// 从 AList `v3.46.0` 起新增。
    #[serde(default, deserialize_with = "null_to_default")]
    pub label_list: Vec<ObjLabel>,
    /// 存储类型标识（`omitempty`；缺失归约为 `None`）。
    ///
    /// 从 AList `v3.54.0` 起新增。
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// 目录列表响应。
///
/// 对应 AList 服务端 fsread 模块的 `FsListResponse`。
/// 新版服务端额外返回 `filtered_total`/`page`/`per_page`/`has_more`/`pages_total`
/// 分页元信息（`has_more`/`pages_total` 可用于翻页终止判断），老版本缺失时为 [`None`]；
/// `content` 在目录为空时可能为 `null`，归约为空数组。
///
/// 注意 `per_page = -1`（请求全部条目）时服务端对任意页码都返回完整列表，
/// 不适合配合自动翻页流使用。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirContent {
    /// 当前页内容；目录为空时服务端可能返回 `null`，归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<LabeledFsObject>,
    /// 过滤后的条目总数（角色过滤前）。
    pub total: i64,
    /// 角色过滤后的条目总数。
    ///
    /// 从 AList `v3.58.0` 起新增角色过滤后条目数；老版本不返回该字段时为 [`None`，严格区分未提供与实际匹配 0 条（`Some(0)`）]。
    #[serde(default)]
    pub filtered_total: Option<i64>,
    /// 服务端实际生效的页码。
    ///
    /// 从 AList `v3.58.0` 起新增服务端生效页码；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub page: Option<i32>,
    /// 服务端实际生效的每页条数。
    ///
    /// 从 AList `v3.58.0` 起新增服务端生效每页条数；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub per_page: Option<i32>,
    /// 是否还有下一页。
    ///
    /// 从 AList `v3.58.0` 起新增是否还有下一页；老版本不返回该字段时为 [`None`，避免误判为 `false` 导致翻页中断]。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 总页数。
    ///
    /// 从 AList `v3.58.0` 起新增总页数；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub pages_total: Option<i32>,
    /// 目录说明（元信息 `readme`）。
    pub readme: String,
    /// 目录页头（元信息 `header`）。
    pub header: String,
    /// 当前用户是否具有写权限。
    pub write: bool,
    /// 目录所在存储的驱动名（如 `Local`）。
    pub provider: String,
}

/// 子目录条目。
///
/// 对应 AList 服务端 fsread 模块的 `DirResponse`，
/// 即 `/api/fs/dirs` 响应数组的元素类型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirEntry {
    /// 文件夹名。
    pub name: String,
    /// 修改时间。
    pub modified: DateTime<Utc>,
}

/// 文件/目录详情响应。
///
/// 对应 AList 服务端 fsread 模块的 `FsGetResponse`，
/// 即 `/api/fs/get` 响应 `data`；Go 侧嵌入 `ObjResponse`（JSON 平铺），
/// 此处以 `serde(flatten)` 复用 [`FsObject`]。
/// `web_proxy` 缺失时为 [`None`]；
/// `related` 为同目录下同前缀的相关文件，无相关文件时服务端返回 `null`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsDetail {
    /// 对象基础字段（Go 侧嵌入的 `ObjResponse`，JSON 平铺）。
    #[serde(flatten)]
    pub obj: FsObject,
    /// 直链原始 URL。
    ///
    /// 目录恒为空串。
    pub raw_url: String,
    /// 目录说明（元信息 `readme`）。
    pub readme: String,
    /// 目录页头（元信息 `header`）。
    pub header: String,
    /// 存储驱动名。
    pub provider: String,
    /// 存储是否开启 Web 代理。
    ///
    /// 从 AList `v3.61.0` 起引入，部分老版本或驱动未返回该字段时为 [`None`]。
    #[serde(default)]
    pub web_proxy: Option<bool>,
    /// 同目录下同前缀的相关文件列表；无相关文件时服务端返回 `null`，归约为空数组。
    ///
    /// 从 AList `v3.46.0` 起新增。
    #[serde(default, deserialize_with = "null_to_default")]
    pub related: Vec<LabeledFsObject>,
}

/// 搜索结果条目。
///
/// 对应 AList 服务端 search 模块的 `SearchResponse`
/// （由 model.Search 数据模型的 `SearchNode` 附加 `type` 字段构成），
/// 即 `/api/fs/search` 响应 `content` 的元素类型。
/// `type` 为新版服务端附加字段，老版本缺失时为 [`None`]。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchItem {
    /// 结果所在父目录。
    pub parent: String,
    /// 文件/目录名。
    pub name: String,
    /// 是否为目录。
    pub is_dir: bool,
    /// 大小（字节）。
    pub size: i64,
    /// 文件类型枚举值（对应 Go `utils.GetObjType`）。
    ///
    /// 从 AList `v3.10.1` 起新增文件类型枚举值；老版本不返回该字段时为 [`None`]。
    #[serde(default)]
    pub r#type: Option<i32>,
}

/// 搜索范围类型。
///
/// 对应 AList `POST /api/fs/search` 请求体中的 `scope` 字段（Go `model.SearchReq.Scope`，
/// AList 服务端 search 模块与 OpenAPI 规范（`0-全部 1-文件夹 2-文件`）：
/// - `0` 为全部（文件与文件夹，默认值）；
/// - `1` 为仅文件夹 / 目录；
/// - `2` 为仅文件。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SearchScope {
    /// 搜索全部（文件与文件夹，服务端默认值）。
    #[default]
    All = 0,
    /// 仅搜索文件夹 / 目录。
    Folder = 1,
    /// 仅搜索文件。
    File = 2,
}

impl SearchScope {
    /// 转换为 AList 服务端对应的整数值。
    #[inline]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
}

impl From<SearchScope> for i32 {
    #[inline]
    fn from(scope: SearchScope) -> Self {
        scope.as_i32()
    }
}

impl Serialize for SearchScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.as_i32())
    }
}

impl<'de> Deserialize<'de> for SearchScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = i32::deserialize(deserializer)?;
        match val {
            0 => Ok(Self::All),
            1 => Ok(Self::Folder),
            2 => Ok(Self::File),
            other => Err(serde::de::Error::custom(format!(
                "invalid search scope {other}, expected 0 (all), 1 (folder), or 2 (file)"
            ))),
        }
    }
}

/// 批量重命名的单项。
///
/// 对应 AList 服务端 fsbatch 模块的 `BatchRenameReq.RenameObjects`
/// 元素，作为 `/api/fs/batch_rename` 请求体 `rename_objects` 的元素。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameObject {
    /// 原文件名。
    pub src_name: String,
    /// 新文件名（服务端会做名称合法性校验）。
    pub new_name: String,
}

/// 聚合移动冲突处理策略。
///
/// 对应 AList `POST /api/fs/recursive_move` 请求体中的 `conflict_policy` 字段（Go `handles` 常量，
/// AList 服务端 fsbatch 模块中常量定义）：
/// - `overwrite`: 直接覆盖已存在的目标文件；
/// - `cancel`: 目标已存在时取消操作并返回 403；
/// - `skip`: 跳过已存在的目标文件。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictPolicy {
    /// 直接覆盖已存在的目标文件。
    Overwrite,
    /// 目标已存在时取消操作并返回 403。
    Cancel,
    /// 跳过已存在的目标文件。
    Skip,
}

impl ConflictPolicy {
    /// 返回对应 AList 协议的字符串切片（`"overwrite"` / `"cancel"` / `"skip"`）。
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Overwrite => "overwrite",
            Self::Cancel => "cancel",
            Self::Skip => "skip",
        }
    }
}

impl AsRef<str> for ConflictPolicy {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// 离线下载临时文件删除策略。
///
/// 对应 AList `POST /api/fs/add_offline_download` 请求体中的 `delete_policy` 字段
/// （对应 AList 服务端 offline_download 工具模块的 `tool.DeletePolicy`）：
/// - `delete_on_upload_succeed`: 上传成功后删除本地临时文件（默认推荐）；
/// - `delete_on_upload_failed`: 上传失败后删除本地临时文件；
/// - `delete_never`: 从不删除本地临时文件；
/// - `delete_always`: 无论转存成功或失败，总是删除本地临时文件。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletePolicy {
    /// 上传成功后删除本地临时文件（默认值）。
    #[default]
    DeleteOnUploadSucceed,
    /// 上传失败后删除本地临时文件。
    DeleteOnUploadFailed,
    /// 从不删除本地临时文件。
    DeleteNever,
    /// 无论转存成功或失败，总是删除本地临时文件。
    DeleteAlways,
}

impl DeletePolicy {
    /// 返回对应 AList 协议的字符串切片。
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeleteOnUploadSucceed => "delete_on_upload_succeed",
            Self::DeleteOnUploadFailed => "delete_on_upload_failed",
            Self::DeleteNever => "delete_never",
            Self::DeleteAlways => "delete_always",
        }
    }
}

impl AsRef<str> for DeletePolicy {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// 复制端点的响应数据。
///
/// 对应 AList 服务端 fsmanage 模块中 `FsCopy` 返回的结构；跨存储复制会创建后台任务。
/// 老版本服务端复制成功时 `data` 为 `null`，因此端点模型为 `Option<CopyResult>`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyResult {
    /// 复制创建的后台任务列表（无后台任务时为空数组）。
    #[serde(default, deserialize_with = "null_to_default")]
    pub tasks: Vec<TaskInfo>,
}

/// 离线下载端点的响应数据。
///
/// 对应 AList 服务端 offline_download 模块的 `AddOfflineDownload`
/// 返回的结构：每个 URL 至多产生一个后台任务；任务 ID 恒为字符串。
/// 后台任务，响应为任务数组（而非单个任务对象），与 openapi 示例一致。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfflineDownloadResult {
    /// 离线下载创建的后台任务列表。
    #[serde(default, deserialize_with = "null_to_default")]
    pub tasks: Vec<TaskInfo>,
}

/// 文件原始直链响应（管理员）。
///
/// 对应 AList 服务端 fsmanage 模块 `Link` 处理器返回的 `model.Link`：
/// `url` 与 `header`（Go `http.Header`，键为头名、值为头值列表）、无
/// JSON tag 的 `Expiration`（序列化键沿用 Go 字段名 `Expiration`，
/// Go `time.Duration` 以纳秒整数序列化，`nil` 时为 `null`），以及
/// 多线程分段下载参数 `concurrency`/`part_size`（代理程序下载加速依据，
/// 非指针整数恒输出）。
/// 自 AList `v3.26.0` 起，服务端对范围读取器、文件句柄与 IP 缓存标记等
/// 内部字段显式忽略（不参与 JSON 序列化）；更早版本可能额外输出这些字段，
/// 反序列化时直接忽略即可。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadLink {
    /// 文件原始直链 URL（存储驱动返回的下载地址，或本站代理地址）。
    pub url: String,
    /// 访问直链所需携带的请求头（键为头名，值为头值列表）；
    /// 服务端未设置（`null`）或字段缺失时归约为空映射。
    #[serde(default, deserialize_with = "null_to_default")]
    pub header: HashMap<String, Vec<String>>,
    /// 直链缓存有效期（纳秒，对应 Go `time.Duration` 的整数序列化）；
    /// 服务端未提供（`null` 或字段缺失）时为 [`None`]。
    #[serde(rename = "Expiration", default)]
    pub expiration: Option<i64>,
    /// 多线程分段下载的并发数（对应 Go `model.Link.Concurrency`，
    /// 供代理程序加速请求使用）；老版本服务端未提供时为 [`None`]。
    ///
    /// 从 AList `v3.25.1` 起随 `model.Link` 引入，当时序列化键为 Go 字段名
    /// `Concurrency`；从 AList `v3.26.0` 起改为小写键名 `concurrency`
    /// （大写键名经别名兼容）。
    #[serde(rename = "concurrency", alias = "Concurrency", default)]
    pub concurrency: Option<i64>,
    /// 多线程分段下载的分段大小（对应 Go `model.Link.PartSize`，
    /// 供代理程序加速请求使用）；老版本服务端未提供时为 [`None`]。
    ///
    /// 版本演进同 [`DownloadLink::concurrency`]：`v3.25.1` 引入大写键名
    /// `PartSize`，`v3.26.0` 起为小写键名 `part_size`。
    #[serde(rename = "part_size", alias = "PartSize", default)]
    pub part_size: Option<i64>,
}

/// 归档元信息中的排序设置。
///
/// 对应 AList 服务端 model.Storage 数据模型的 `model.Sort`，
/// 由归档驱动随元信息返回（`ArchiveMetaResponse.Sort`）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveSort {
    /// 排序字段名（如 `name`、`size`）。
    pub order_by: String,
    /// 排序方向（如 `asc`、`desc`）。
    pub order_direction: String,
    /// 文件夹提取策略。
    pub extract_folder: String,
}

/// 压缩包元信息响应。
///
/// 对应 AList 服务端 archive 模块的 `ArchiveMetaResponse`，
/// 即 `POST /api/fs/archive/meta` 响应 `data`。
/// 压缩包为空或驱动未提供树形结构时 `content` 可能为 `null`，归约为空数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveMeta {
    /// 压缩包注释。
    pub comment: String,
    /// 压缩包内容是否加密（对应 Go `ArchiveMetaResponse.IsEncrypted`，JSON 键 `encrypted`）。
    pub encrypted: bool,
    /// 压缩包内文件的树形结构；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<ArchiveNode>,
    /// 驱动提供的排序设置；未提供时缺省（`omitempty`）。
    #[serde(default)]
    pub sort: Option<ArchiveSort>,
    /// 归档预览直链（`/ae` 或 `/ad` 前缀）。
    pub raw_url: String,
    /// 归档直链签名。
    ///
    /// 未开启签名时为空串。
    pub sign: String,
}

/// 压缩包内的文件条目（树形）。
///
/// 对应 AList 服务端 archive 模块的 `ArchiveContentResponse`：
/// 嵌入 `ObjResponse`（JSON 平铺）并递归携带 `children`；
/// 目录无子项时服务端返回 `null`，归约为空数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveNode {
    /// 对象基础字段（Go 侧嵌入的 `ObjResponse`，JSON 平铺）。
    #[serde(flatten)]
    pub obj: FsObject,
    /// 子文件/子目录；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub children: Vec<ArchiveNode>,
}

/// 压缩包内部列表响应。
///
/// 对应 AList 服务端 archive 模块的 `ArchiveListResponse`，
/// 即 `POST /api/fs/archive/list` 响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveList {
    /// 当前页的压缩包内部条目；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<FsObject>,
    /// 条目总数。
    pub total: i64,
}

/// 解压端点的响应数据。
///
/// 对应 AList 服务端 archive 模块中 `FsArchiveDecompress` 返回的
/// 结构：JSON 键为单数 `task`，但值是任务数组（每个被解压的文件对应一个任务）。
/// （每个解压目标一项）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecompressResult {
    /// 解压创建的后台任务列表（无后台任务时为空数组）。
    #[serde(default, deserialize_with = "null_to_default")]
    pub task: Vec<TaskInfo>,
}

/// 解压目标名集合：兼容单个字符串与字符串数组两种 JSON 形状。
///
/// 对应 AList 服务端 archive 模块的 `StringOrArray`，
/// 作为 `POST /api/fs/archive/decompress` 请求体 `name` 字段的
/// 类型；序列化恒为数组形状（与 Go 行为一致），反序列化兼容单值。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringOrArray(
    #[serde(deserialize_with = "deserialize_string_or_array")] pub Vec<String>,
);

impl StringOrArray {
    /// 以单个目标名构造。
    #[must_use]
    pub fn single(name: impl Into<String>) -> Self {
        Self(vec![name.into()])
    }
}

/// 上传端点（`PUT /api/fs/put` 流式上传与 `PUT /api/fs/form` 表单上传）的响应数据。
///
/// 即 AList 服务端上传处理模块以 `gin.H{"task": getTaskInfo(t)}` 返回的结构，复用共享模型
/// [`UploadResult`](crate::schema::common::UploadResult)。
/// 直传成功（未启用 `As-Task` 头）时端点 `data` 为 `null`，
/// 因此端点模型为 `Option<UploadResult>`；上传请求载荷全部位于 HTTP 头与原始 body，
/// 无 JSON 请求模型。
pub type UploadResult = crate::schema::common::UploadResult;

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：AList OpenAPI 规范的 `/api/fs/list` 返回示例（含 content 数组与 total 字段）。
    /// （老版本形状：条目无 `id`/`path`/`virtual_path`/`label_list`，
    /// 响应无 `filtered_total` 等新增分页字段）。
    #[test]
    fn dir_content_decodes_openapi_example() {
        let resp: DirContent = serde_json::from_value(serde_json::json!({
            "content": [
                {
                    "name": "Alist V3.md",
                    "size": 1592,
                    "is_dir": false,
                    "modified": "2024-05-17T13:47:55.4174917+08:00",
                    "created": "2024-05-17T13:47:47.5725906+08:00",
                    "sign": "",
                    "thumb": "",
                    "type": 4,
                    "hashinfo": "null",
                    "hash_info": null
                }
            ],
            "total": 1,
            "readme": "",
            "header": "",
            "write": true,
            "provider": "Local"
        }))
        .unwrap();
        assert_eq!(resp.content.len(), 1);
        let obj = &resp.content[0];
        assert_eq!(obj.name, "Alist V3.md");
        assert_eq!(obj.size, 1592);
        assert!(!obj.is_dir);
        assert_eq!(obj.r#type, 4);
        assert_eq!(obj.hashinfo, Some("null".to_owned()));
        assert!(
            obj.hash_info.is_empty(),
            "显式 null 的 hash_info 归约为空映射"
        );
        assert_eq!(obj.id, None, "老版本缺失的 id 为 None");
        assert_eq!(obj.path, None, "老版本缺失的 path 为 None");
        assert_eq!(obj.virtual_path, None, "老版本缺失的 virtual_path 为 None");
        assert!(
            obj.label_list.is_empty(),
            "老版本缺失的 label_list 归约为空数组"
        );
        assert_eq!(obj.storage_class, None);
        assert_eq!(resp.total, 1);
        assert!(resp.write);
        assert_eq!(resp.provider, "Local");
        // 新版分页字段缺失 → None
        assert_eq!(
            resp.filtered_total, None,
            "老版本缺失的 filtered_total 为 None"
        );
        assert_eq!(resp.page, None, "老版本缺失的 page 为 None");
        assert_eq!(resp.per_page, None, "老版本缺失的 per_page 为 None");
        assert_eq!(resp.has_more, None, "老版本缺失的 has_more 为 None");
        assert_eq!(resp.pages_total, None, "老版本缺失的 pages_total 为 None");
    }

    /// 兼容钉扎：按 AList 服务端 fsread 模块的 `FsListResponse` 形状反序列化真实响应。
    /// `ObjLabelResponse` JSON tag 构造新版服务端形状（含新增字段与标签列表）。
    #[test]
    fn dir_content_decodes_new_server_shape() {
        let resp: DirContent = serde_json::from_value(serde_json::json!({
            "content": [
                {
                    "id": "local-1",
                    "path": "/data/Alist V3.md",
                    "virtual_path": "/local/Alist V3.md",
                    "name": "Alist V3.md",
                    "size": 1592,
                    "is_dir": false,
                    "modified": "2024-05-17T13:47:55.4174917+08:00",
                    "created": "2024-05-17T13:47:47.5725906+08:00",
                    "sign": "",
                    "thumb": "",
                    "type": 4,
                    "hashinfo": "sha1:abc123",
                    "hash_info": { "sha1": "abc123" },
                    "label_list": [
                        {
                            "id": 1,
                            "type": 2,
                            "name": "文档",
                            "description": "",
                            "bg_color": "#1890ff",
                            "create_time": "2024-06-01T12:00:00+08:00"
                        }
                    ],
                    "storage_class": "STANDARD"
                }
            ],
            "total": 1,
            "filtered_total": 1,
            "page": 1,
            "per_page": 200,
            "has_more": false,
            "pages_total": 1,
            "readme": "readme",
            "header": "",
            "write": true,
            "provider": "Local"
        }))
        .unwrap();
        let obj = &resp.content[0];
        assert_eq!(obj.id, Some("local-1".to_owned()));
        assert_eq!(obj.path, Some("/data/Alist V3.md".to_owned()));
        assert_eq!(obj.virtual_path, Some("/local/Alist V3.md".to_owned()));
        assert_eq!(obj.hashinfo, Some("sha1:abc123".to_owned()));
        assert_eq!(
            obj.hash_info.get("sha1").map(String::as_str),
            Some("abc123")
        );
        assert_eq!(obj.label_list.len(), 1);
        assert_eq!(obj.label_list[0].name, "文档");
        assert_eq!(obj.label_list[0].r#type, 2);
        assert_eq!(obj.storage_class.as_deref(), Some("STANDARD"));
        assert_eq!(resp.filtered_total, Some(1));
        assert_eq!(resp.page, Some(1));
        assert_eq!(resp.per_page, Some(200));
        assert_eq!(resp.has_more, Some(false));
        assert_eq!(resp.pages_total, Some(1));
    }

    /// 兼容钉扎：目录为空时服务端 `content` 为 `null`（Go `toObjsResp` 对空切片
    /// 返回 `nil`），应归约为空数组。
    #[test]
    fn dir_content_tolerates_null_content() {
        let resp: DirContent = serde_json::from_value(serde_json::json!({
            "content": null,
            "total": 0,
            "readme": "",
            "header": "",
            "write": false,
            "provider": "Local"
        }))
        .unwrap();
        assert!(resp.content.is_empty());
        assert_eq!(resp.total, 0);
    }

    /// 正向钉扎：AList OpenAPI 规范的 `/api/fs/get` 返回示例（含 raw_url 与 provider 字段）。
    /// （`related: null`、无 `web_proxy` 字段的老版本形状）。
    #[test]
    fn fs_detail_decodes_openapi_example() {
        let resp: FsDetail = serde_json::from_value(serde_json::json!({
            "name": "Alist V3.md",
            "size": 2618,
            "is_dir": false,
            "modified": "2024-05-17T16:05:36.4651534+08:00",
            "created": "2024-05-17T16:05:29.2001008+08:00",
            "sign": "",
            "thumb": "",
            "type": 4,
            "hashinfo": "null",
            "hash_info": null,
            "raw_url": "http://127.0.0.1:5244/p/local/Alist%20V3.md",
            "readme": "",
            "header": "",
            "provider": "Local",
            "related": null
        }))
        .unwrap();
        assert_eq!(resp.obj.name, "Alist V3.md");
        assert_eq!(resp.obj.size, 2618);
        assert_eq!(resp.obj.r#type, 4);
        assert_eq!(resp.raw_url, "http://127.0.0.1:5244/p/local/Alist%20V3.md");
        assert_eq!(resp.provider, "Local");
        assert!(resp.related.is_empty(), "显式 null 的 related 归约为空数组");
        assert_eq!(resp.web_proxy, None, "老版本缺失的 web_proxy 为 None");
        assert_eq!(resp.obj.id, None, "老版本缺失的 id 为 None");
        assert_eq!(resp.obj.path, None, "老版本缺失的 path 为 None");
        assert_eq!(
            resp.obj.virtual_path, None,
            "老版本缺失的 virtual_path 为 None"
        );
        assert_eq!(resp.obj.hashinfo, Some("null".to_owned()));
    }

    /// 正向钉扎：AList OpenAPI 规范的 `/api/fs/dirs` 返回示例。
    #[test]
    fn dir_entry_decodes_openapi_example() {
        let dirs: Vec<DirEntry> = serde_json::from_value(serde_json::json!([
            { "name": "a", "modified": "2023-07-19T09:48:13.695585868+08:00" }
        ]))
        .unwrap();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, "a");
    }

    /// 正向钉扎：AList OpenAPI 规范的 `/api/fs/search` 返回示例；
    /// 兼容老版本无 `type` 字段的形状。
    #[test]
    fn search_item_decodes_openapi_example() {
        let page: crate::schema::common::PageResponse<SearchItem> = serde_json::from_value(
            serde_json::json!({
                "content": [
                    { "parent": "/m", "name": "4305da1e", "is_dir": false, "size": 393090, "type": 0 }
                ],
                "total": 1
            }),
        )
        .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].parent, "/m");
        assert_eq!(page.content[0].r#type, Some(0));

        // 老版本服务端不返回 type 字段
        let old: SearchItem = serde_json::from_value(serde_json::json!({
            "parent": "/m", "name": "a.txt", "is_dir": false, "size": 1
        }))
        .unwrap();
        assert_eq!(old.r#type, None);
    }

    /// 序列化键名钉扎：批量重命名单项按服务端 fsbatch 模块的 JSON 标签序列化。
    #[test]
    fn rename_object_serializes_with_api_field_names() {
        let item = RenameObject {
            src_name: "test.txt".to_owned(),
            new_name: "aaas2.txt".to_owned(),
        };
        assert_eq!(
            serde_json::to_value(&item).unwrap(),
            serde_json::json!({ "src_name": "test.txt", "new_name": "aaas2.txt" })
        );
    }

    /// 正向钉扎：AList OpenAPI 规范的 `/api/fs/add_offline_download` 返回示例；
    /// 返回示例；`/api/fs/copy` 响应同构（`gin.H{"tasks": ...}`）。
    #[test]
    fn offline_download_result_decodes_openapi_example() {
        let resp: OfflineDownloadResult = serde_json::from_value(serde_json::json!({
            "tasks": [
                {
                    "id": "jwy7BrfZRzbI2xWg7-y",
                    "name": "download https://www.baidu.com/img/20d6cf.png to (/local)",
                    "state": 0,
                    "status": "",
                    "progress": 0,
                    "error": ""
                }
            ]
        }))
        .unwrap();
        assert_eq!(resp.tasks.len(), 1);
        assert_eq!(resp.tasks[0].id, "jwy7BrfZRzbI2xWg7-y");

        let copy: CopyResult = serde_json::from_value(serde_json::json!({ "tasks": [] })).unwrap();
        assert!(copy.tasks.is_empty());
    }

    /// 正向钉扎：AList 服务端 fsmanage 模块 `Link` 处理器的响应形状
    /// （`model.Link` 的 JSON 序列化：`url`、`header`、无 tag 的 `Expiration`
    /// 与 `concurrency`/`part_size`，Go `time.Duration` 以纳秒整数序列化；
    /// 该端点不在 OpenAPI 规范中，形状取自服务端结构体，URL 为示例占位）。
    #[test]
    fn download_link_decodes_go_server_shape() {
        let link: DownloadLink = serde_json::from_value(serde_json::json!({
            "url": "https://example.com/d/file.mkv",
            "header": { "User-Agent": ["alist"], "Cookie": ["a=1", "b=2"] },
            "Expiration": 3600000000000i64,
            "concurrency": 4,
            "part_size": 10485760
        }))
        .unwrap();
        assert_eq!(link.url, "https://example.com/d/file.mkv");
        assert_eq!(
            link.header.get("Cookie").map(Vec::as_slice),
            Some(&["a=1".to_owned(), "b=2".to_owned()][..])
        );
        assert_eq!(
            link.header.get("User-Agent").map(Vec::as_slice).unwrap(),
            ["alist"]
        );
        assert_eq!(
            link.expiration,
            Some(3_600_000_000_000),
            "Go time.Duration 纳秒整数"
        );
        assert_eq!(link.concurrency, Some(4), "v3.26.0+ 小写键名 concurrency");
        assert_eq!(
            link.part_size,
            Some(10_485_760),
            "v3.26.0+ 小写键名 part_size"
        );
    }

    /// 兼容钉扎：AList `v3.25.1`（引入 `model.Link.Concurrency`/`PartSize`
    /// 时无 json tag）以 Go 字段名大写键名序列化，反序列化经别名兼容。
    #[test]
    fn download_link_accepts_v3_25_1_uppercase_keys() {
        let link: DownloadLink = serde_json::from_value(serde_json::json!({
            "url": "https://example.com/d/file.mkv",
            "header": {},
            "Expiration": null,
            "Concurrency": 8,
            "PartSize": 65536
        }))
        .unwrap();
        assert_eq!(link.concurrency, Some(8), "大写键名 Concurrency 经别名兼容");
        assert_eq!(link.part_size, Some(65_536), "大写键名 PartSize 经别名兼容");
    }

    /// 序列化键名钉扎：`Expiration` 键沿用 Go 无 tag 字段名（大写 `E`），
    /// 其余键为 `url`/`header`/`concurrency`/`part_size`（小写 JSON tag）；
    /// 内部字段（范围读取器等）不出现在序列化输出中。
    #[test]
    fn download_link_serializes_with_go_field_names() {
        let link = DownloadLink {
            url: "https://example.com/d/file.mkv".to_owned(),
            header: HashMap::new(),
            expiration: None,
            concurrency: Some(4),
            part_size: Some(10_485_760),
        };
        let value = serde_json::to_value(&link).unwrap();
        assert_eq!(value["url"], "https://example.com/d/file.mkv");
        assert!(
            value.get("Expiration").is_some(),
            "序列化必须携带 Expiration 键: {value}"
        );
        assert_eq!(value["Expiration"], serde_json::Value::Null);
        assert!(
            value.get("header").is_some(),
            "序列化必须携带 header 键: {value}"
        );
        assert_eq!(value["concurrency"], 4, "v3.26.0+ 序列化键名为小写");
        assert_eq!(value["part_size"], 10_485_760, "v3.26.0+ 序列化键名为小写");
    }

    /// 兼容钉扎：本站仅本地代理的存储返回 `header: null` 与 `Expiration: null`；
    /// 老版本服务端缺失字段（以及 v3.26.0 之前额外输出的内部字段）同样兼容。
    #[test]
    fn download_link_tolerates_null_and_missing_fields() {
        let proxied: DownloadLink = serde_json::from_value(serde_json::json!({
            "url": "http://127.0.0.1:5244/p/local/file.mkv?d&sign=x",
            "header": null,
            "Expiration": null
        }))
        .unwrap();
        assert!(proxied.header.is_empty(), "null 的 header 归约为空映射");
        assert_eq!(proxied.expiration, None);

        let legacy: DownloadLink =
            serde_json::from_value(serde_json::json!({ "url": "http://d/file" })).unwrap();
        assert!(legacy.header.is_empty(), "缺失的 header 归约为空映射");
        assert_eq!(legacy.expiration, None, "缺失的 Expiration 为 None");
        assert_eq!(
            legacy.concurrency, None,
            "v3.25.1 之前缺失的 concurrency 为 None"
        );
        assert_eq!(
            legacy.part_size, None,
            "v3.25.1 之前缺失的 part_size 为 None"
        );
    }

    /// 钉扎：AList 服务端 archive 模块的 `ArchiveMetaResponse`/
    /// `ArchiveContentResponse` JSON 标签形状（树形 children、可选 sort）。
    #[test]
    fn archive_meta_decodes_go_server_shape() {
        let resp: ArchiveMeta = serde_json::from_value(serde_json::json!({
            "comment": "demo",
            "encrypted": false,
            "content": [
                {
                    "name": "dir",
                    "size": 0,
                    "is_dir": true,
                    "modified": "2024-05-17T13:47:55.4174917+08:00",
                    "sign": "",
                    "thumb": "",
                    "type": 1,
                    "children": [
                        {
                            "name": "inner.txt",
                            "size": 12,
                            "is_dir": false,
                            "modified": "2024-05-17T13:47:55.4174917+08:00",
                            "sign": "",
                            "thumb": "",
                            "type": 4,
                            "children": null
                        }
                    ]
                }
            ],
            "sort": { "order_by": "name", "order_direction": "asc", "extract_folder": "front" },
            "raw_url": "http://127.0.0.1:5244/ae/local/demo.zip",
            "sign": ""
        }))
        .unwrap();
        assert_eq!(resp.comment, "demo");
        assert!(!resp.encrypted);
        let root = &resp.content[0];
        assert!(root.obj.is_dir);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].obj.name, "inner.txt");
        assert!(
            root.children[0].children.is_empty(),
            "显式 null 的 children 归约为空数组"
        );
        let sort = resp.sort.as_ref().unwrap();
        assert_eq!(sort.order_by, "name");
        assert_eq!(sort.order_direction, "asc");
        // 无 sort 字段时缺省
        let no_sort: ArchiveMeta = serde_json::from_value(serde_json::json!({
            "comment": "", "encrypted": true, "content": null,
            "raw_url": "", "sign": ""
        }))
        .unwrap();
        assert_eq!(no_sort.sort, None);
        assert!(no_sort.content.is_empty());
    }

    /// 钉扎：AList 服务端 archive 模块的 `ArchiveListResponse` 形状。
    #[test]
    fn archive_list_decodes_go_server_shape() {
        let resp: ArchiveList = serde_json::from_value(serde_json::json!({
            "content": [
                {
                    "name": "inner.txt",
                    "size": 12,
                    "is_dir": false,
                    "modified": "2024-05-17T13:47:55.4174917+08:00",
                    "sign": "",
                    "thumb": "",
                    "type": 4
                }
            ],
            "total": 1
        }))
        .unwrap();
        assert_eq!(resp.total, 1);
        assert_eq!(resp.content[0].name, "inner.txt");
    }

    /// 钉扎：解压响应——键为单数 `task`、值为数组。
    #[test]
    fn decompress_result_decodes_task_array() {
        let resp: DecompressResult = serde_json::from_value(serde_json::json!({
            "task": [
                { "id": "abc", "name": "decompress demo.zip[/a.txt]", "state": 0,
                  "status": "", "progress": 0, "error": "" }
            ]
        }))
        .unwrap();
        assert_eq!(resp.task.len(), 1);
        assert_eq!(resp.task[0].id, "abc");
    }

    /// 钉扎：`StringOrArray`——单字符串归约为单元素数组，
    /// 序列化恒为数组形状。
    #[test]
    fn string_or_array_accepts_single_and_array_shapes() {
        let single: StringOrArray = serde_json::from_value(serde_json::json!("a.txt")).unwrap();
        assert_eq!(single.0, vec!["a.txt".to_owned()]);
        let many: StringOrArray =
            serde_json::from_value(serde_json::json!(["a.txt", "b.txt"])).unwrap();
        assert_eq!(many.0, vec!["a.txt".to_owned(), "b.txt".to_owned()]);
        assert_eq!(
            serde_json::to_value(StringOrArray::single("a.txt")).unwrap(),
            serde_json::json!(["a.txt"])
        );
    }

    /// 钉扎：上传响应复用 [`crate::schema::common::UploadResult`]。
    #[test]
    fn upload_result_alias_reuses_common_model() {
        use crate::schema::common::Response;

        let resp: Response<UploadResult> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": {
                "task": {
                    "id": "sdH2LbjyWRk",
                    "name": "upload demo.txt to [/data](/alist)",
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

    /// 钉扎：`SearchScope` 序列化与反序列化形状符合 AList 服务端整数约定（0=全部, 1=文件夹, 2=文件）。
    #[test]
    fn search_scope_serde_matches_integer_contract() {
        assert_eq!(SearchScope::default(), SearchScope::All);
        assert_eq!(SearchScope::All.as_i32(), 0);
        assert_eq!(SearchScope::Folder.as_i32(), 1);
        assert_eq!(SearchScope::File.as_i32(), 2);
        assert_eq!(i32::from(SearchScope::File), 2);

        // 序列化为整数
        assert_eq!(
            serde_json::to_value(SearchScope::All).unwrap(),
            serde_json::json!(0)
        );
        assert_eq!(
            serde_json::to_value(SearchScope::Folder).unwrap(),
            serde_json::json!(1)
        );
        assert_eq!(
            serde_json::to_value(SearchScope::File).unwrap(),
            serde_json::json!(2)
        );

        // 反序列化自整数
        assert_eq!(
            serde_json::from_value::<SearchScope>(serde_json::json!(0)).unwrap(),
            SearchScope::All
        );
        assert_eq!(
            serde_json::from_value::<SearchScope>(serde_json::json!(1)).unwrap(),
            SearchScope::Folder
        );
        assert_eq!(
            serde_json::from_value::<SearchScope>(serde_json::json!(2)).unwrap(),
            SearchScope::File
        );

        // 非法数值反序列化报错
        assert!(serde_json::from_value::<SearchScope>(serde_json::json!(3)).is_err());
        assert!(serde_json::from_value::<SearchScope>(serde_json::json!(-1)).is_err());
    }

    /// 钉扎：`ConflictPolicy` 序列化与反序列化形状符合 AList 服务端小写字符串约定（overwrite, cancel, skip）。
    #[test]
    fn conflict_policy_serde_matches_lowercase_string_contract() {
        assert_eq!(ConflictPolicy::Overwrite.as_str(), "overwrite");
        assert_eq!(ConflictPolicy::Cancel.as_str(), "cancel");
        assert_eq!(ConflictPolicy::Skip.as_str(), "skip");
        assert_eq!(ConflictPolicy::Overwrite.as_ref(), "overwrite");

        // 序列化
        assert_eq!(
            serde_json::to_value(ConflictPolicy::Overwrite).unwrap(),
            serde_json::json!("overwrite")
        );
        assert_eq!(
            serde_json::to_value(ConflictPolicy::Cancel).unwrap(),
            serde_json::json!("cancel")
        );
        assert_eq!(
            serde_json::to_value(ConflictPolicy::Skip).unwrap(),
            serde_json::json!("skip")
        );

        // 反序列化
        assert_eq!(
            serde_json::from_value::<ConflictPolicy>(serde_json::json!("overwrite")).unwrap(),
            ConflictPolicy::Overwrite
        );
        assert_eq!(
            serde_json::from_value::<ConflictPolicy>(serde_json::json!("cancel")).unwrap(),
            ConflictPolicy::Cancel
        );
        assert_eq!(
            serde_json::from_value::<ConflictPolicy>(serde_json::json!("skip")).unwrap(),
            ConflictPolicy::Skip
        );

        // 非法字符串反序列化报错
        assert!(serde_json::from_value::<ConflictPolicy>(serde_json::json!("other")).is_err());
        assert!(serde_json::from_value::<ConflictPolicy>(serde_json::json!("OVERWRITE")).is_err());
    }

    /// 钉扎：`DeletePolicy` 序列化与反序列化形状符合 AList 服务端蛇形命名约定（delete_on_upload_succeed 等）。
    #[test]
    fn delete_policy_serde_matches_snake_case_string_contract() {
        assert_eq!(DeletePolicy::default(), DeletePolicy::DeleteOnUploadSucceed);
        assert_eq!(
            DeletePolicy::DeleteOnUploadSucceed.as_str(),
            "delete_on_upload_succeed"
        );
        assert_eq!(
            DeletePolicy::DeleteOnUploadFailed.as_str(),
            "delete_on_upload_failed"
        );
        assert_eq!(DeletePolicy::DeleteNever.as_str(), "delete_never");
        assert_eq!(DeletePolicy::DeleteAlways.as_str(), "delete_always");
        assert_eq!(
            DeletePolicy::DeleteOnUploadSucceed.as_ref(),
            "delete_on_upload_succeed"
        );

        // 序列化
        assert_eq!(
            serde_json::to_value(DeletePolicy::DeleteOnUploadSucceed).unwrap(),
            serde_json::json!("delete_on_upload_succeed")
        );
        assert_eq!(
            serde_json::to_value(DeletePolicy::DeleteOnUploadFailed).unwrap(),
            serde_json::json!("delete_on_upload_failed")
        );
        assert_eq!(
            serde_json::to_value(DeletePolicy::DeleteNever).unwrap(),
            serde_json::json!("delete_never")
        );
        assert_eq!(
            serde_json::to_value(DeletePolicy::DeleteAlways).unwrap(),
            serde_json::json!("delete_always")
        );

        // 反序列化
        assert_eq!(
            serde_json::from_value::<DeletePolicy>(serde_json::json!("delete_on_upload_succeed"))
                .unwrap(),
            DeletePolicy::DeleteOnUploadSucceed
        );
        assert_eq!(
            serde_json::from_value::<DeletePolicy>(serde_json::json!("delete_on_upload_failed"))
                .unwrap(),
            DeletePolicy::DeleteOnUploadFailed
        );
        assert_eq!(
            serde_json::from_value::<DeletePolicy>(serde_json::json!("delete_never")).unwrap(),
            DeletePolicy::DeleteNever
        );
        assert_eq!(
            serde_json::from_value::<DeletePolicy>(serde_json::json!("delete_always")).unwrap(),
            DeletePolicy::DeleteAlways
        );

        // 非法字符串反序列化报错
        assert!(
            serde_json::from_value::<DeletePolicy>(serde_json::json!("delete_on_success")).is_err()
        );
        assert!(serde_json::from_value::<DeletePolicy>(serde_json::json!("unknown")).is_err());
    }
}
