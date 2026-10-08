//! fs 文件系统域数据模型。
//!
//! 覆盖 `fs` 端点组的三个子域：
//!
//! - **文件系统**：目录列表、文件详情、目录树、搜索与目录/重命名/移动/复制/删除/
//!   离线下载等管理操作（`/api/fs/*`）；
//! - **归档（archive）**：压缩包元信息、内部列表与解压（`/api/fs/archive/*`），
//!   模型供归档端点代理直接复用；
//! - **上传（upload）**：`PUT /api/fs/put` / `PUT /api/fs/form` 的响应模型
//!   [`UploadResponse`]（上传请求载荷全部位于
//!   HTTP 头与原始 body，无 JSON 请求模型）。
//!
//! 字段形状的数据来源（与 Go 源码冲突时以 Go 为准）：
//! `docs/api/alistv3.openapi.yaml` 的 `fs` 分组与 `examples/alist` 的
//! `server/handles/fsread.go`（列表/详情/目录树）、`server/handles/fsmanage.go`
//! （新建/重命名/移动/复制/删除）、`server/handles/fsbatch.go`（批量/正则重命名、
//! 聚合移动）、`server/handles/search.go` 与 `internal/model/search.go`（搜索）、
//! `server/handles/offline_download.go`（离线下载）、`server/handles/archive.go`
//! 与 `internal/model/archive.go`（归档）、`server/handles/fsup.go`（上传）。
//!
//! 跨版本兼容说明：新版服务端在 fs 列表/详情对象上新增 `id`/`path`/`virtual_path`/
//! `hashinfo`/`hash_info`/`storage_class`/`label_list` 与 `filtered_total`/`page`/
//! `per_page`/`has_more`/`pages_total` 等字段，老版本缺失或返回显式 `null`，
//! 相关字段均以 `#[serde(default)]`（集合字段配合 `null_to_default`）接住。

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
/// `server/handles/archive.go:224-238` 的自定义 `UnmarshalJSON`）。
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
/// 对应 `examples/alist/server/handles/fsread.go` 的 `ObjResponse`（fsread.go:35-50），
/// 同时作为归档内部列表条目（`archive.go:41-44` 的 `ArchiveContentResponse` 嵌入字段）。
/// `id`/`path`/`virtual_path`/`hashinfo`/`hash_info`/`storage_class` 为新版服务端
/// 新增字段，老版本缺失时归约为空值。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjResponse {
    /// 对象 ID（对应 Go `ObjResponse.Id`；老版本不返回，归约为空串）。
    #[serde(default)]
    pub id: String,
    /// 对象在存储内的实际路径（对应 Go `ObjResponse.Path`；老版本不返回）。
    #[serde(default)]
    pub path: String,
    /// 对象在站点内的展示路径（对应 Go `ObjResponse.VirtualPath`；老版本不返回）。
    #[serde(default)]
    pub virtual_path: String,
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
    /// 下载签名；用于构造带签名的直链。
    pub sign: String,
    /// 缩略图地址。
    pub thumb: String,
    /// 文件类型枚举值（对应 Go `utils.GetObjType`，JSON 键为保留字 `type`）。
    pub r#type: i32,
    /// 哈希信息字符串（对应 Go `ObjResponse.HashInfoStr`；老版本可能缺失，归约为空串）。
    #[serde(default)]
    pub hashinfo: String,
    /// 结构化哈希信息；键为哈希算法名（如 `md5`/`sha1`/`sha256`），
    /// 对应 Go `ObjResponse.HashInfo`（`map[*utils.HashType]string`）。
    /// 老版本不返回或返回 `null`，归约为空映射。
    #[serde(default, deserialize_with = "null_to_default")]
    pub hash_info: HashMap<String, String>,
    /// 存储类型标识（对应 Go `ObjResponse.StorageClass`，`omitempty`；缺失归约为 `None`）。
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// 文件对象携带的标签条目。
///
/// 对应 `examples/alist/internal/model/label.go` 的 `model.Label`，
/// 出现在目录列表条目的 `label_list` 字段中（`fsread.go:66-82` 的 `ObjLabelResponse`）。
/// 与 admin 域的标签模型形状一致，但独立定义以避免 feature 交叉依赖。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjLabel {
    /// 标签 ID（对应 Go `model.Label.ID`，gorm 主键 `uint`）。
    pub id: u64,
    /// 标签类型（对应 Go `model.Label.Type`，JSON 键为保留字 `type`）。
    #[serde(rename = "type")]
    pub label_type: i32,
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
/// 对应 `examples/alist/server/handles/fsread.go` 的 `ObjLabelResponse`（fsread.go:66-82），
/// 即 `/api/fs/list` 响应 `content` 的元素类型；`label_list` 与新增的
/// `id`/`path`/`virtual_path`/`storage_class` 在老版本服务端缺失时归约为空值。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjLabelResponse {
    /// 对象 ID（对应 Go `ObjLabelResponse.Id`；老版本不返回，归约为空串）。
    #[serde(default)]
    pub id: String,
    /// 对象在存储内的实际路径（老版本不返回）。
    #[serde(default)]
    pub path: String,
    /// 对象在站点内的展示路径（老版本不返回）。
    #[serde(default)]
    pub virtual_path: String,
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
    /// 哈希信息字符串；老版本可能缺失，归约为空串。
    #[serde(default)]
    pub hashinfo: String,
    /// 结构化哈希信息；键为哈希算法名（如 `md5`/`sha1`）；缺失或 `null` 归约为空映射。
    #[serde(default, deserialize_with = "null_to_default")]
    pub hash_info: HashMap<String, String>,
    /// 文件绑定的标签列表（对应 Go `ObjLabelResponse.LabelList`）；目录恒为空，
    /// 老版本不返回或返回 `null`，归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub label_list: Vec<ObjLabel>,
    /// 存储类型标识（`omitempty`；缺失归约为 `None`）。
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// 目录列表响应。
///
/// 对应 `examples/alist/server/handles/fsread.go` 的 `FsListResponse`（fsread.go:52-64）。
/// 新版服务端额外返回 `filtered_total`/`page`/`per_page`/`has_more`/`pages_total`
/// 分页元信息（`has_more`/`pages_total` 可用于翻页终止判断），老版本缺失时归约为零值；
/// `content` 在目录为空时可能为 `null`，归约为空数组。
///
/// 注意 `per_page = -1`（请求全部条目）时服务端对任意页码都返回完整列表，
/// 不适合配合自动翻页流使用。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsListResponse {
    /// 当前页内容；目录为空时服务端可能返回 `null`，归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<ObjLabelResponse>,
    /// 过滤后的条目总数（角色过滤前）。
    pub total: i64,
    /// 角色过滤后的条目总数（新版服务端字段；老版本归约为 `0`）。
    #[serde(default)]
    pub filtered_total: i64,
    /// 服务端实际生效的页码（新版服务端字段；老版本归约为 `0`）。
    #[serde(default)]
    pub page: i32,
    /// 服务端实际生效的每页条数（新版服务端字段；老版本归约为 `0`）。
    #[serde(default)]
    pub per_page: i32,
    /// 是否还有下一页（新版服务端字段；老版本归约为 `false`）。
    #[serde(default)]
    pub has_more: bool,
    /// 总页数（新版服务端字段；老版本归约为 `0`）。
    #[serde(default)]
    pub pages_total: i32,
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
/// 对应 `examples/alist/server/handles/fsread.go` 的 `DirResponse`（fsread.go:208-211），
/// 即 `/api/fs/dirs` 响应数组的元素类型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirResponse {
    /// 文件夹名。
    pub name: String,
    /// 修改时间。
    pub modified: DateTime<Utc>,
}

/// 文件/目录详情响应。
///
/// 对应 `examples/alist/server/handles/fsread.go` 的 `FsGetResponse`（fsread.go:346-354），
/// 即 `/api/fs/get` 响应 `data`；Go 侧嵌入 `ObjResponse`（JSON 平铺），
/// 此处以 `serde(flatten)` 复用 [`ObjResponse`]。
/// `web_proxy` 为新版服务端字段（老版本缺失归约为 `false`）；
/// `related` 为同目录下同前缀的相关文件，无相关文件时服务端返回 `null`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsGetResponse {
    /// 对象基础字段（Go 侧嵌入的 `ObjResponse`，JSON 平铺）。
    #[serde(flatten)]
    pub obj: ObjResponse,
    /// 直链原始 URL；目录恒为空串。
    pub raw_url: String,
    /// 目录说明（元信息 `readme`）。
    pub readme: String,
    /// 目录页头（元信息 `header`）。
    pub header: String,
    /// 存储驱动名。
    pub provider: String,
    /// 存储是否开启 Web 代理（新版服务端字段；老版本归约为 `false`）。
    #[serde(default)]
    pub web_proxy: bool,
    /// 同目录下同前缀的相关文件列表；无相关文件时服务端返回 `null`，归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub related: Vec<ObjLabelResponse>,
}

/// 搜索结果条目。
///
/// 对应 `examples/alist/server/handles/search.go` 的 `SearchResponse`（search.go:22-25，
/// 由 `internal/model/search.go` 的 `SearchNode` 附加 `type` 字段构成），
/// 即 `/api/fs/search` 响应 `content` 的元素类型。
/// `type` 为新版服务端附加字段，老版本缺失时归约为 `0`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResponse {
    /// 结果所在父目录。
    pub parent: String,
    /// 文件/目录名。
    pub name: String,
    /// 是否为目录。
    pub is_dir: bool,
    /// 大小（字节）。
    pub size: i64,
    /// 文件类型枚举值（对应 Go `utils.GetObjType`；老版本缺失归约为 `0`）。
    #[serde(default)]
    pub r#type: i32,
}

/// 搜索范围类型。
///
/// 对应 AList `POST /api/fs/search` 请求体中的 `scope` 字段（Go `model.SearchReq.Scope`，
/// `internal/model/search.go:19`；OpenAPI `0-全部 1-文件夹 2-文件`）：
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
/// 对应 `examples/alist/server/handles/fsbatch.go` 的 `BatchRenameReq.RenameObjects`
/// 元素（fsbatch.go:148-152），作为 `/api/fs/batch_rename` 请求体 `rename_objects` 的元素。
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
/// `examples/alist/server/handles/const.go:4-6` 与 `handles/fsbatch.go:117-126`）：
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

/// 复制端点的响应数据。
///
/// 对应 `examples/alist/server/handles/fsmanage.go` 的 `FsCopy` 返回的
/// `gin.H{"tasks": ...}`（fsmanage.go:215-217）；跨存储复制会创建后台任务。
/// 老版本服务端复制成功时 `data` 为 `null`，因此端点模型为 `Option<CopyResponse>`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyResponse {
    /// 复制创建的后台任务列表（无后台任务时为空数组）。
    #[serde(default, deserialize_with = "null_to_default")]
    pub tasks: Vec<TaskInfo>,
}

/// 离线下载端点的响应数据。
///
/// 对应 `examples/alist/server/handles/offline_download.go` 的 `AddOfflineDownload`
/// 返回的 `gin.H{"tasks": ...}`（offline_download.go:383-385）：每个 URL 至多产生一个
/// 后台任务，响应为任务数组（而非单个任务对象），与 openapi 示例一致。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfflineDownloadResponse {
    /// 离线下载创建的后台任务列表。
    #[serde(default, deserialize_with = "null_to_default")]
    pub tasks: Vec<TaskInfo>,
}

/// 归档元信息中的排序设置。
///
/// 对应 `examples/alist/internal/model/storage.go` 的 `model.Sort`（storage.go:24-28），
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
/// 对应 `examples/alist/server/handles/archive.go` 的 `ArchiveMetaResponse`
/// （archive.go:32-39），即 `POST /api/fs/archive/meta` 响应 `data`。
/// 压缩包为空或驱动未提供树形结构时 `content` 可能为 `null`，归约为空数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveMetaResponse {
    /// 压缩包注释。
    pub comment: String,
    /// 压缩包内容是否加密（对应 Go `ArchiveMetaResponse.IsEncrypted`，JSON 键 `encrypted`）。
    pub encrypted: bool,
    /// 压缩包内文件的树形结构；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<ArchiveContentResponse>,
    /// 驱动提供的排序设置；未提供时缺省（`omitempty`）。
    #[serde(default)]
    pub sort: Option<ArchiveSort>,
    /// 归档预览直链（`/ae` 或 `/ad` 前缀）。
    pub raw_url: String,
    /// 归档直链签名；未开启签名时为空串。
    pub sign: String,
}

/// 压缩包内的文件条目（树形）。
///
/// 对应 `examples/alist/server/handles/archive.go` 的 `ArchiveContentResponse`
/// （archive.go:41-44）：嵌入 `ObjResponse`（JSON 平铺）并递归携带 `children`；
/// 目录无子项时服务端返回 `null`，归约为空数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveContentResponse {
    /// 对象基础字段（Go 侧嵌入的 `ObjResponse`，JSON 平铺）。
    #[serde(flatten)]
    pub obj: ObjResponse,
    /// 子文件/子目录；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub children: Vec<ArchiveContentResponse>,
}

/// 压缩包内部列表响应。
///
/// 对应 `examples/alist/server/handles/archive.go` 的 `ArchiveListResponse`
/// （archive.go:153-156），即 `POST /api/fs/archive/list` 响应 `data`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveListResponse {
    /// 当前页的压缩包内部条目；为空时归约为空数组。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<ObjResponse>,
    /// 条目总数。
    pub total: i64,
}

/// 解压端点的响应数据。
///
/// 对应 `examples/alist/server/handles/archive.go` 的 `FsArchiveDecompress` 返回的
/// `gin.H{"task": ...}`（archive.go:318-320）：JSON 键为单数 `task`，但值是任务数组
/// （每个解压目标一项）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchiveDecompressResponse {
    /// 解压创建的后台任务列表（无后台任务时为空数组）。
    #[serde(default, deserialize_with = "null_to_default")]
    pub task: Vec<TaskInfo>,
}

/// 解压目标名集合：兼容单个字符串与字符串数组两种 JSON 形状。
///
/// 对应 `examples/alist/server/handles/archive.go` 的 `StringOrArray`
/// （archive.go:224-238），作为 `POST /api/fs/archive/decompress` 请求体 `name` 字段的
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
/// 即 `examples/alist/server/handles/fsup.go` 中 `FsStream`/`FsForm` 以
/// `gin.H{"task": getTaskInfo(t)}` 返回的结构，复用共享模型
/// [`UploadResponse`](crate::schema::common::UploadResponse)。
/// 直传成功（未启用 `As-Task` 头）时端点 `data` 为 `null`（fsup.go:104-109），
/// 因此端点模型为 `Option<UploadResponse>`；上传请求载荷全部位于 HTTP 头与原始 body，
/// 无 JSON 请求模型。
pub type UploadResponse = crate::schema::common::UploadResponse;

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/list` 返回示例
    /// （老版本形状：条目无 `id`/`path`/`virtual_path`/`label_list`，
    /// 响应无 `filtered_total` 等新增分页字段）。
    #[test]
    fn fs_list_response_decodes_openapi_example() {
        let resp: FsListResponse = serde_json::from_value(serde_json::json!({
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
        assert_eq!(obj.hashinfo, "null");
        assert!(
            obj.hash_info.is_empty(),
            "显式 null 的 hash_info 归约为空映射"
        );
        assert_eq!(obj.id, "", "老版本缺失的 id 归约为空串");
        assert!(
            obj.label_list.is_empty(),
            "老版本缺失的 label_list 归约为空数组"
        );
        assert_eq!(obj.storage_class, None);
        assert_eq!(resp.total, 1);
        assert!(resp.write);
        assert_eq!(resp.provider, "Local");
        // 新版分页字段缺失 → 零值
        assert_eq!(resp.filtered_total, 0);
        assert_eq!(resp.page, 0);
        assert!(!resp.has_more);
        assert_eq!(resp.pages_total, 0);
    }

    /// 兼容钉扎：按 `examples/alist/server/handles/fsread.go` 的 `FsListResponse`/
    /// `ObjLabelResponse` JSON tag 构造新版服务端形状（含新增字段与标签列表）。
    #[test]
    fn fs_list_response_decodes_new_server_shape() {
        let resp: FsListResponse = serde_json::from_value(serde_json::json!({
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
        assert_eq!(obj.id, "local-1");
        assert_eq!(obj.virtual_path, "/local/Alist V3.md");
        assert_eq!(
            obj.hash_info.get("sha1").map(String::as_str),
            Some("abc123")
        );
        assert_eq!(obj.label_list.len(), 1);
        assert_eq!(obj.label_list[0].name, "文档");
        assert_eq!(obj.storage_class.as_deref(), Some("STANDARD"));
        assert_eq!(resp.filtered_total, 1);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.per_page, 200);
        assert!(!resp.has_more);
        assert_eq!(resp.pages_total, 1);
    }

    /// 兼容钉扎：目录为空时服务端 `content` 为 `null`（Go `toObjsResp` 对空切片
    /// 返回 `nil`），应归约为空数组。
    #[test]
    fn fs_list_response_tolerates_null_content() {
        let resp: FsListResponse = serde_json::from_value(serde_json::json!({
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

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/get` 返回示例
    /// （`related: null`、无 `web_proxy` 字段的老版本形状）。
    #[test]
    fn fs_get_response_decodes_openapi_example() {
        let resp: FsGetResponse = serde_json::from_value(serde_json::json!({
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
        assert!(!resp.web_proxy, "老版本缺失的 web_proxy 归约为 false");
    }

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/dirs` 返回示例。
    #[test]
    fn dir_response_decodes_openapi_example() {
        let dirs: Vec<DirResponse> = serde_json::from_value(serde_json::json!([
            { "name": "a", "modified": "2023-07-19T09:48:13.695585868+08:00" }
        ]))
        .unwrap();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, "a");
    }

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/search` 返回示例；
    /// 兼容老版本无 `type` 字段的形状。
    #[test]
    fn search_response_decodes_openapi_example() {
        let page: crate::schema::common::PageResponse<SearchResponse> = serde_json::from_value(
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
        assert_eq!(page.content[0].r#type, 0);

        // 老版本服务端不返回 type 字段
        let old: SearchResponse = serde_json::from_value(serde_json::json!({
            "parent": "/m", "name": "a.txt", "is_dir": false, "size": 1
        }))
        .unwrap();
        assert_eq!(old.r#type, 0);
    }

    /// 序列化键名钉扎：批量重命名单项按 fsbatch.go 的 JSON tag 序列化。
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

    /// 正向钉扎：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/add_offline_download`
    /// 返回示例；`/api/fs/copy` 响应同构（`gin.H{"tasks": ...}`）。
    #[test]
    fn offline_download_response_decodes_openapi_example() {
        let resp: OfflineDownloadResponse = serde_json::from_value(serde_json::json!({
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

        let copy: CopyResponse =
            serde_json::from_value(serde_json::json!({ "tasks": [] })).unwrap();
        assert!(copy.tasks.is_empty());
    }

    /// 钉扎：`examples/alist/server/handles/archive.go` 的 `ArchiveMetaResponse`/
    /// `ArchiveContentResponse` JSON tag 形状（树形 children、可选 sort）。
    #[test]
    fn archive_meta_response_decodes_go_server_shape() {
        let resp: ArchiveMetaResponse = serde_json::from_value(serde_json::json!({
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
        let no_sort: ArchiveMetaResponse = serde_json::from_value(serde_json::json!({
            "comment": "", "encrypted": true, "content": null,
            "raw_url": "", "sign": ""
        }))
        .unwrap();
        assert_eq!(no_sort.sort, None);
        assert!(no_sort.content.is_empty());
    }

    /// 钉扎：`examples/alist/server/handles/archive.go` 的 `ArchiveListResponse` 形状。
    #[test]
    fn archive_list_response_decodes_go_server_shape() {
        let resp: ArchiveListResponse = serde_json::from_value(serde_json::json!({
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

    /// 钉扎：解压响应 `gin.H{"task": ...}`（archive.go:318-320）——键为单数 `task`、值为数组。
    #[test]
    fn archive_decompress_response_decodes_task_array() {
        let resp: ArchiveDecompressResponse = serde_json::from_value(serde_json::json!({
            "task": [
                { "id": "abc", "name": "decompress demo.zip[/a.txt]", "state": 0,
                  "status": "", "progress": 0, "error": "" }
            ]
        }))
        .unwrap();
        assert_eq!(resp.task.len(), 1);
        assert_eq!(resp.task[0].id, "abc");
    }

    /// 钉扎：`StringOrArray`（archive.go:224-238）——单字符串归约为单元素数组，
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

    /// 钉扎：上传响应复用 [`crate::schema::common::UploadResponse`]（fsup.go `gin.H{"task": ...}`）。
    #[test]
    fn upload_response_alias_reuses_common_model() {
        use crate::schema::common::Response;

        let resp: Response<UploadResponse> = serde_json::from_value(serde_json::json!({
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
}
