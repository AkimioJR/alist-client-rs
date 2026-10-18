//! share 分享域数据模型。
//!
//! 覆盖分享管理端点（`/api/share/create|update|disable|list|delete`）与
//! 公开分享端点（`/api/public/share/info|auth|list|get`）的响应数据：
//! 分享管理实体 [`Share`]、公开分享元信息 [`PublicShareInfo`]、
//! 公开分享内对象 [`PublicShareObject`]、公开分享目录列表
//! [`PublicShareList`]、公开分享对象详情 [`PublicShareDetail`]
//! 与分享令牌应答 [`PublicShareAuth`]。
//!
//! ## 字段形状来源
//!
//! AList OpenAPI 规范未收录分享端点分组，字段形状以 AList 服务端实现为准：
//! 路由与响应结构体见 AList 服务端 handles.Share 相关处理模块（分享管理
//! `CreateShare`/`UpdateShare`/`ListShares` 等，公开分享 `GetPublicShareInfo`/
//! `AuthPublicShare`/`ListPublicShare`/`GetPublicShare`），持久化实体字段见
//! AList 服务端 model.Share 数据模型。
//!
//! ## 跨版本兼容
//!
//! - 分享端点自 AList `v3.59.0` 起提供，模型字段自引入即完整；
//! - `storage_class`/`download_url`/`preview_url` 在服务端标记 `omitempty`，
//!   缺失时建模为 [`None`]。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 服务端对集合类字段返回显式 `null` 时归约为空集合。
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// 分享管理实体（响应 `data`）。
///
/// 对应 AList 服务端 handles 分享管理模块的 `ShareResp`，
/// 由创建分享、更新分享直接返回，并作为当前用户分享列表
/// （`GET /api/share/list`）的内容元素。
/// 底层对应 AList 服务端 model.Share 数据模型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    /// 分享记录的数据库 ID（对应 Go `ShareResp.ID`）。
    pub id: u64,
    /// 分享 ID（对应 Go `ShareResp.ShareID`，链接中 `/s/{share_id}` 段）。
    pub share_id: String,
    /// 分享名称（对应 Go `ShareResp.Name`，未显式命名时为分享对象名）。
    pub name: String,
    /// 分享根路径（对应 Go `ShareResp.RootPath`，站点内绝对路径）。
    pub root_path: String,
    /// 分享目标是否为目录。
    pub is_dir: bool,
    /// 分享是否设置了密码。
    pub has_password: bool,
    /// 是否为阅后即焚分享（访问次数上限为 1）。
    pub burn_after_read: bool,
    /// 访问次数上限（0 表示不限）。
    pub access_limit: i64,
    /// 内容访问计数（浏览、下载与预览合计）。
    pub access_count: i64,
    /// 剩余可访问次数（未设上限时为 0）。
    pub remaining_accesses: i64,
    /// 是否允许在线预览。
    pub allow_preview: bool,
    /// 是否允许下载。
    pub allow_download: bool,
    /// 分享是否启用（禁用或已耗尽的分享为 `false`）。
    pub enabled: bool,
    /// 落地页浏览计数。
    pub view_count: i64,
    /// 下载计数。
    pub download_count: i64,
    /// 最近一次内容访问时间；从未被访问时为 [`None`]。
    #[serde(default)]
    pub last_access_at: Option<DateTime<Utc>>,
    /// 分享耗尽（达到访问上限）的时间；未耗尽时为 [`None`]。
    #[serde(default)]
    pub consumed_at: Option<DateTime<Utc>>,
    /// 过期时间；未设置过期时为 [`None`]。
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    /// 创建时间。
    pub created_at: DateTime<Utc>,
    /// 最近更新时间。
    pub updated_at: DateTime<Utc>,
    /// 分享链接完整地址（对应 Go `ShareResp.URL`，`{api_url}/s/{share_id}`）。
    pub url: String,
}

/// 公开分享元信息（响应 `data`）。
///
/// 对应 AList 服务端公开分享处理模块的 `PublicShareInfoResp`，
/// 由 `GET /api/public/share/info` 返回；无需认证即可获取，
/// 用于判断分享是否需要密码以及当前令牌是否已通过校验。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicShareInfo {
    /// 分享 ID。
    pub share_id: String,
    /// 分享名称。
    pub name: String,
    /// 分享目标是否为目录。
    pub is_dir: bool,
    /// 分享是否设置了密码。
    pub has_password: bool,
    /// 是否为阅后即焚分享（访问次数上限为 1）。
    pub burn_after_read: bool,
    /// 访问次数上限（0 表示不限）。
    pub access_limit: i64,
    /// 内容访问计数。
    pub access_count: i64,
    /// 剩余可访问次数（未设上限时为 0）。
    pub remaining_accesses: i64,
    /// 是否允许在线预览。
    pub allow_preview: bool,
    /// 是否允许下载。
    pub allow_download: bool,
    /// 当前请求是否已通过访问校验（无密码分享恒为 `true`；
    /// 有密码分享仅在携带有效分享令牌时为 `true`）。
    pub authed: bool,
    /// 分享耗尽（达到访问上限）的时间；未耗尽时为 [`None`]。
    #[serde(default)]
    pub consumed_at: Option<DateTime<Utc>>,
    /// 过期时间；未设置过期时为 [`None`]。
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    /// 创建时间。
    pub created_at: DateTime<Utc>,
}

/// 公开分享内的对象条目。
///
/// 对应 AList 服务端公开分享处理模块的 `PublicShareObjResp`，
/// 出现在公开分享目录列表的 `content` 与公开分享对象详情的 `item` 中。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicShareObject {
    /// 对象名。
    pub name: String,
    /// 大小（字节）。
    pub size: i64,
    /// 是否为目录。
    pub is_dir: bool,
    /// 修改时间（对应 Go `time.Time`）。
    pub modified: DateTime<Utc>,
    /// 创建时间；存储不支持时服务端返回零值时间。
    pub created: DateTime<Utc>,
    /// 缩略图地址。
    pub thumb: String,
    /// 文件类型枚举值（对应 Go `utils.GetObjType`，JSON 键为保留字 `type`）。
    pub r#type: i32,
    /// 分享内相对路径（以 `/` 起始；分享根目录为 `/`）。
    pub path: String,
    /// 存储类型标识（对应 Go `PublicShareObjResp.StorageClass`，`omitempty`）。
    ///
    /// 存储未提供或服务端省略该字段时为 [`None`]。
    #[serde(default)]
    pub storage_class: Option<String>,
    /// 文件下载直链（对应 Go `PublicShareObjResp.DownloadURL`，`omitempty`）。
    ///
    /// 仅在对象为文件且分享允许下载时返回，否则为 [`None`]。
    #[serde(default)]
    pub download_url: Option<String>,
    /// 文件预览直链（对应 Go `PublicShareObjResp.PreviewURL`，`omitempty`）。
    ///
    /// 仅在对象为文件且分享允许预览时返回，否则为 [`None`]。
    #[serde(default)]
    pub preview_url: Option<String>,
}

/// 公开分享目录列表（响应 `data`）。
///
/// 对应 AList 服务端公开分享处理模块的 `PublicShareListResp`，
/// 由 `POST /api/public/share/list` 返回；与通用分页响应不同，
/// 额外携带页码、每页条数、`has_more` 与总页数元信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicShareList {
    /// 当前页内容；服务端返回 `null` 时归约为空集合。
    #[serde(default, deserialize_with = "null_to_default")]
    pub content: Vec<PublicShareObject>,
    /// 分享内该目录的条目总数。
    pub total: i64,
    /// 当前页码。
    pub page: i32,
    /// 本次请求实际生效的每页条数（请求缺省时由服务端归一化）。
    pub per_page: i32,
    /// 是否还有下一页。
    pub has_more: bool,
    /// 总页数（目录为空时为 0）。
    pub pages_total: i32,
}

/// 公开分享对象详情（响应 `data`）。
///
/// 对应 AList 服务端公开分享处理模块的 `PublicShareGetResp`，
/// 由 `POST /api/public/share/get` 返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicShareDetail {
    /// 对象详情。
    pub item: PublicShareObject,
    /// 对象所在存储的驱动标识（如 `Local`、`S3`）；存储不可知时为 `"unknown"`。
    pub provider: String,
}

/// 公开分享鉴权应答（响应 `data`）。
///
/// 对应 AList 服务端公开分享处理模块 `AuthPublicShare` 以
/// `gin.H{"token": ...}` 返回的结构，由 `POST /api/public/share/auth` 返回。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicShareAuth {
    /// 分享访问令牌。
    ///
    /// 有效期由服务端决定（缺省 24 小时，且不超过分享剩余有效时间）；
    /// 分享未设置密码时为空字符串。
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 示例 JSON 取自 AList 服务端公开分享处理模块 `GetPublicShareInfo`
    /// 返回的 `PublicShareInfoResp` 形状（各字段均为服务端必返字段）。
    #[test]
    fn public_share_info_decodes_handler_shape() {
        let info: PublicShareInfo = serde_json::from_value(serde_json::json!({
            "share_id": "aBcD1234",
            "name": "docs",
            "is_dir": true,
            "has_password": true,
            "burn_after_read": false,
            "access_limit": 10,
            "access_count": 3,
            "remaining_accesses": 7,
            "allow_preview": true,
            "allow_download": true,
            "authed": false,
            "consumed_at": null,
            "expires_at": "2026-12-31T23:59:59+08:00",
            "created_at": "2026-03-20T10:00:00+08:00"
        }))
        .unwrap();
        assert_eq!(info.share_id, "aBcD1234");
        assert_eq!(info.name, "docs");
        assert!(info.is_dir);
        assert!(info.has_password);
        assert!(!info.burn_after_read);
        assert_eq!(info.access_limit, 10);
        assert_eq!(info.access_count, 3);
        assert_eq!(info.remaining_accesses, 7);
        assert!(info.allow_preview);
        assert!(info.allow_download);
        assert!(!info.authed);
        assert_eq!(info.consumed_at, None);
        assert!(info.expires_at.is_some());
        assert!(
            info.created_at
                .to_rfc3339()
                .starts_with("2026-03-20T02:00:00")
        );
    }

    /// 老版本形状钉扎：可空时间字段缺失（而非显式 `null`）时归约为 `None`。
    #[test]
    fn public_share_info_tolerates_missing_nullable_fields() {
        let info: PublicShareInfo = serde_json::from_value(serde_json::json!({
            "share_id": "aBcD1234",
            "name": "file.txt",
            "is_dir": false,
            "has_password": false,
            "burn_after_read": true,
            "access_limit": 1,
            "access_count": 1,
            "remaining_accesses": 0,
            "allow_preview": true,
            "allow_download": false,
            "authed": true,
            "created_at": "2026-03-20T10:00:00+08:00"
        }))
        .unwrap();
        assert_eq!(info.consumed_at, None);
        assert_eq!(info.expires_at, None);
        assert!(info.burn_after_read);
        assert!(info.authed);
    }

    /// 示例 JSON 取自 AList 服务端公开分享处理模块 `toPublicShareObjResp`
    /// 返回的 `PublicShareObjResp` 形状；`omitempty` 字段省略时为 `None`，
    /// 显式 `null` 时归约为 `None`。
    #[test]
    fn public_share_object_decodes_with_and_without_omitempty_fields() {
        // 文件对象：允许下载与预览时携带直链
        let file: PublicShareObject = serde_json::from_value(serde_json::json!({
            "name": "demo.txt",
            "size": 1024,
            "is_dir": false,
            "modified": "2026-03-20T10:00:00+08:00",
            "created": "2026-03-20T10:00:00+08:00",
            "thumb": "",
            "type": 0,
            "path": "/demo.txt",
            "storage_class": "Standard",
            "download_url": "https://alist.example/sd/aBcD1234/demo.txt?auth=t",
            "preview_url": "https://alist.example/sp/aBcD1234/demo.txt?auth=t"
        }))
        .unwrap();
        assert_eq!(file.name, "demo.txt");
        assert_eq!(file.size, 1024);
        assert!(!file.is_dir);
        assert_eq!(file.r#type, 0);
        assert_eq!(file.path, "/demo.txt");
        assert_eq!(file.storage_class.as_deref(), Some("Standard"));
        assert!(file.download_url.as_deref().unwrap().contains("/sd/"));
        assert!(file.preview_url.is_some());

        // 目录对象：omitempty 字段缺失，`type` 为目录枚举值
        let dir: PublicShareObject = serde_json::from_value(serde_json::json!({
            "name": "sub",
            "size": 0,
            "is_dir": true,
            "modified": "2026-03-20T10:00:00+08:00",
            "created": "0001-01-01T00:00:00Z",
            "thumb": "",
            "type": 1,
            "path": "/sub"
        }))
        .unwrap();
        assert!(dir.is_dir);
        assert_eq!(dir.r#type, 1);
        assert_eq!(dir.storage_class, None);
        assert_eq!(dir.download_url, None);
        assert_eq!(dir.preview_url, None);

        // 显式 `null` 的 omitempty 字段归约为 `None`
        let null_obj: PublicShareObject = serde_json::from_value(serde_json::json!({
            "name": "x.bin",
            "size": 1,
            "is_dir": false,
            "modified": "2026-03-20T10:00:00+08:00",
            "created": "0001-01-01T00:00:00Z",
            "thumb": "",
            "type": 6,
            "path": "/x.bin",
            "storage_class": null,
            "download_url": null,
            "preview_url": null
        }))
        .unwrap();
        assert_eq!(null_obj.storage_class, None);
        assert_eq!(null_obj.download_url, None);
        assert_eq!(null_obj.preview_url, None);
    }

    /// 示例 JSON 取自 AList 服务端公开分享处理模块 `ListPublicShare`
    /// 返回的 `PublicShareListResp` 形状；同时钉住空目录返回
    /// （`content` 为空数组、`pages_total` 为 0）与 `null` 容错。
    #[test]
    fn public_share_list_decodes_handler_shape() {
        let list: PublicShareList = serde_json::from_value(serde_json::json!({
            "content": [{
                "name": "demo.txt",
                "size": 1024,
                "is_dir": false,
                "modified": "2026-03-20T10:00:00+08:00",
                "created": "2026-03-20T10:00:00+08:00",
                "thumb": "",
                "type": 0,
                "path": "/demo.txt"
            }],
            "total": 1,
            "page": 1,
            "per_page": 30,
            "has_more": false,
            "pages_total": 1
        }))
        .unwrap();
        assert_eq!(list.total, 1);
        assert_eq!(list.page, 1);
        assert_eq!(list.per_page, 30);
        assert!(!list.has_more);
        assert_eq!(list.pages_total, 1);
        assert_eq!(list.content.len(), 1);
        assert_eq!(list.content[0].name, "demo.txt");

        let empty: PublicShareList = serde_json::from_value(serde_json::json!({
            "content": [],
            "total": 0,
            "page": 1,
            "per_page": 200,
            "has_more": false,
            "pages_total": 0
        }))
        .unwrap();
        assert!(empty.content.is_empty());
        assert_eq!(empty.pages_total, 0);

        let null_content: PublicShareList = serde_json::from_value(serde_json::json!({
            "content": null,
            "total": 0,
            "page": 1,
            "per_page": 200,
            "has_more": false,
            "pages_total": 0
        }))
        .unwrap();
        assert!(null_content.content.is_empty());
    }

    /// 示例 JSON 取自 AList 服务端公开分享处理模块 `GetPublicShare`
    /// 返回的 `PublicShareGetResp` 形状。
    #[test]
    fn public_share_detail_decodes_handler_shape() {
        let get: PublicShareDetail = serde_json::from_value(serde_json::json!({
            "item": {
                "name": "demo.txt",
                "size": 2048,
                "is_dir": false,
                "modified": "2026-03-20T10:00:00+08:00",
                "created": "2026-03-20T10:00:00+08:00",
                "thumb": "",
                "type": 0,
                "path": "/",
                "download_url": "https://alist.example/sd/aBcD1234?auth=t"
            },
            "provider": "Local"
        }))
        .unwrap();
        assert_eq!(get.item.name, "demo.txt");
        assert_eq!(get.item.path, "/");
        assert!(get.item.download_url.is_some());
        assert_eq!(get.provider, "Local");
    }

    /// 示例 JSON 取自 AList 服务端公开分享处理模块 `AuthPublicShare`
    /// 以 `gin.H{"token": ...}` 返回的结构；无密码分享 `token` 为空串。
    #[test]
    fn public_share_auth_decodes_handler_shape() {
        let authed: PublicShareAuth = serde_json::from_value(serde_json::json!({
            "token": "eyJhbGciOiJIUzI1NiJ9.payload.sig"
        }))
        .unwrap();
        assert_eq!(authed.token, "eyJhbGciOiJIUzI1NiJ9.payload.sig");

        let no_password: PublicShareAuth =
            serde_json::from_value(serde_json::json!({ "token": "" })).unwrap();
        assert_eq!(no_password.token, "");
    }

    /// 示例 JSON 取自 AList 服务端分享管理模块 `toShareResp`
    /// 返回的 `ShareResp` 形状（创建/更新响应与列表元素同形）。
    #[test]
    fn share_decodes_handler_shape() {
        let share: Share = serde_json::from_value(serde_json::json!({
            "id": 1,
            "share_id": "aBcD1234",
            "name": "docs",
            "root_path": "/data/docs",
            "is_dir": true,
            "has_password": true,
            "burn_after_read": false,
            "access_limit": 0,
            "access_count": 5,
            "remaining_accesses": 0,
            "allow_preview": true,
            "allow_download": true,
            "enabled": true,
            "view_count": 12,
            "download_count": 2,
            "last_access_at": "2026-04-01T08:00:00+08:00",
            "consumed_at": null,
            "expires_at": "2026-12-31T23:59:59+08:00",
            "created_at": "2026-03-20T10:00:00+08:00",
            "updated_at": "2026-03-21T10:00:00+08:00",
            "url": "https://alist.example/s/aBcD1234"
        }))
        .unwrap();
        assert_eq!(share.id, 1);
        assert_eq!(share.share_id, "aBcD1234");
        assert_eq!(share.root_path, "/data/docs");
        assert!(share.is_dir);
        assert!(share.has_password);
        assert!(!share.burn_after_read);
        assert_eq!(share.access_limit, 0);
        assert_eq!(share.access_count, 5);
        assert_eq!(share.remaining_accesses, 0);
        assert!(share.enabled);
        assert_eq!(share.view_count, 12);
        assert_eq!(share.download_count, 2);
        assert!(share.last_access_at.is_some());
        assert_eq!(share.consumed_at, None);
        assert!(share.expires_at.is_some());
        assert_eq!(share.url, "https://alist.example/s/aBcD1234");
    }

    /// 老版本形状钉扎：可空时间字段缺失时归约为 `None`（其余必返字段齐全）。
    #[test]
    fn share_tolerates_missing_nullable_fields() {
        let share: Share = serde_json::from_value(serde_json::json!({
            "id": 2,
            "share_id": "xYzW8765",
            "name": "file.txt",
            "root_path": "/data/file.txt",
            "is_dir": false,
            "has_password": false,
            "burn_after_read": true,
            "access_limit": 1,
            "access_count": 1,
            "remaining_accesses": 0,
            "allow_preview": true,
            "allow_download": true,
            "enabled": false,
            "view_count": 0,
            "download_count": 0,
            "created_at": "2026-03-20T10:00:00+08:00",
            "updated_at": "2026-03-20T10:00:00+08:00",
            "url": "https://alist.example/s/xYzW8765"
        }))
        .unwrap();
        assert_eq!(share.last_access_at, None);
        assert_eq!(share.consumed_at, None);
        assert_eq!(share.expires_at, None);
        assert!(share.burn_after_read);
        assert!(!share.enabled);
    }

    /// `GET /api/share/list` 响应外层为通用分页结构，元素为分享实体。
    #[test]
    fn share_list_page_response_wraps_share_elements() {
        let page: crate::schema::common::PageResponse<Share> =
            serde_json::from_value(serde_json::json!({
                "content": [{
                    "id": 1,
                    "share_id": "aBcD1234",
                    "name": "docs",
                    "root_path": "/data/docs",
                    "is_dir": true,
                    "has_password": false,
                    "burn_after_read": false,
                    "access_limit": 0,
                    "access_count": 0,
                    "remaining_accesses": 0,
                    "allow_preview": true,
                    "allow_download": true,
                    "enabled": true,
                    "view_count": 0,
                    "download_count": 0,
                    "created_at": "2026-03-20T10:00:00+08:00",
                    "updated_at": "2026-03-20T10:00:00+08:00",
                    "url": "https://alist.example/s/aBcD1234"
                }],
                "total": 1
            }))
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.content[0].share_id, "aBcD1234");
    }

    /// 序列化键名钉扎：`Share` 经 `serde_json::to_value` 的键集合
    /// 与 AList 服务端 `ShareResp` 的 JSON 字段名逐一对应。
    #[test]
    fn share_serializes_with_api_field_names() {
        let share = Share {
            id: 1,
            share_id: "aBcD1234".to_owned(),
            name: "docs".to_owned(),
            root_path: "/data/docs".to_owned(),
            is_dir: true,
            has_password: false,
            burn_after_read: false,
            access_limit: 0,
            access_count: 0,
            remaining_accesses: 0,
            allow_preview: true,
            allow_download: true,
            enabled: true,
            view_count: 0,
            download_count: 0,
            last_access_at: None,
            consumed_at: None,
            expires_at: None,
            created_at: "2026-03-20T10:00:00+08:00".parse().unwrap(),
            updated_at: "2026-03-20T10:00:00+08:00".parse().unwrap(),
            url: "https://alist.example/s/aBcD1234".to_owned(),
        };
        let value = serde_json::to_value(&share).unwrap();
        let mut keys: Vec<String> = value.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "access_count",
                "access_limit",
                "allow_download",
                "allow_preview",
                "burn_after_read",
                "consumed_at",
                "created_at",
                "download_count",
                "enabled",
                "expires_at",
                "has_password",
                "id",
                "is_dir",
                "last_access_at",
                "name",
                "remaining_accesses",
                "root_path",
                "share_id",
                "updated_at",
                "url",
                "view_count"
            ]
        );
        // 可空时间字段（响应模型）序列化为显式 `null` 而非被跳过
        assert!(value["last_access_at"].is_null());
        assert!(value["consumed_at"].is_null());
        assert!(value["expires_at"].is_null());
    }

    /// 序列化键名钉扎：`PublicShareInfo` 的键集合与
    /// AList 服务端 `PublicShareInfoResp` 的 JSON 字段名逐一对应。
    #[test]
    fn public_share_info_serializes_with_api_field_names() {
        let info = PublicShareInfo {
            share_id: "aBcD1234".to_owned(),
            name: "docs".to_owned(),
            is_dir: true,
            has_password: false,
            burn_after_read: false,
            access_limit: 0,
            access_count: 0,
            remaining_accesses: 0,
            allow_preview: true,
            allow_download: true,
            authed: true,
            consumed_at: None,
            expires_at: None,
            created_at: "2026-03-20T10:00:00+08:00".parse().unwrap(),
        };
        let value = serde_json::to_value(&info).unwrap();
        let mut keys: Vec<String> = value.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "access_count",
                "access_limit",
                "allow_download",
                "allow_preview",
                "authed",
                "burn_after_read",
                "consumed_at",
                "created_at",
                "expires_at",
                "has_password",
                "is_dir",
                "name",
                "remaining_accesses",
                "share_id"
            ]
        );
    }

    /// 序列化键名钉扎：`PublicShareObject` 的键集合与
    /// AList 服务端 `PublicShareObjResp` 的 JSON 字段名逐一对应；
    /// 特别钉住保留字键 `type`（Rust 字段为 `r#type`，无 rename）与
    /// `omitempty` 字段为 [`None`] 时序列化为显式 `null`。
    #[test]
    fn public_share_object_serializes_with_api_field_names() {
        let obj = PublicShareObject {
            name: "demo.txt".to_owned(),
            size: 1024,
            is_dir: false,
            modified: "2026-03-20T10:00:00+08:00".parse().unwrap(),
            created: "2026-03-20T10:00:00+08:00".parse().unwrap(),
            thumb: String::new(),
            r#type: 0,
            path: "/demo.txt".to_owned(),
            storage_class: None,
            download_url: None,
            preview_url: None,
        };
        let value = serde_json::to_value(&obj).unwrap();
        let mut keys: Vec<String> = value.as_object().unwrap().keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "created",
                "download_url",
                "is_dir",
                "modified",
                "name",
                "path",
                "preview_url",
                "size",
                "storage_class",
                "thumb",
                "type"
            ]
        );
        assert_eq!(value["type"], 0, "r#type 应序列化为保留字键 type");
        assert!(value["storage_class"].is_null());
    }

    /// 序列化键名钉扎：`PublicShareList`、`PublicShareDetail`
    /// 与 `PublicShareAuth` 的键集合与 AList 服务端
    /// `PublicShareListResp`、`PublicShareGetResp` 及 `gin.H{"token": ...}`
    /// 的 JSON 字段名逐一对应。
    #[test]
    fn public_share_list_detail_auth_serializes_with_api_field_names() {
        let list = PublicShareList {
            content: Vec::new(),
            total: 0,
            page: 1,
            per_page: 30,
            has_more: false,
            pages_total: 0,
        };
        let mut keys: Vec<String> = serde_json::to_value(&list)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "content",
                "has_more",
                "page",
                "pages_total",
                "per_page",
                "total"
            ]
        );

        let get = PublicShareDetail {
            item: PublicShareObject {
                name: "demo.txt".to_owned(),
                size: 0,
                is_dir: false,
                modified: "2026-03-20T10:00:00+08:00".parse().unwrap(),
                created: "2026-03-20T10:00:00+08:00".parse().unwrap(),
                thumb: String::new(),
                r#type: 0,
                path: "/".to_owned(),
                storage_class: None,
                download_url: None,
                preview_url: None,
            },
            provider: "Local".to_owned(),
        };
        let mut keys: Vec<String> = serde_json::to_value(&get)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        keys.sort();
        assert_eq!(keys, vec!["item", "provider"]);

        let auth = PublicShareAuth {
            token: "share-token".to_owned(),
        };
        assert_eq!(
            serde_json::to_value(&auth).unwrap(),
            serde_json::json!({ "token": "share-token" })
        );
    }
}
