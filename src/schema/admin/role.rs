//! admin-role 角色域数据模型。
//!
//! 涵盖角色实体 [`Role`] 与路径级权限条目 [`PermissionEntry`]，
//! 服务于 `/api/admin/role` 下的 list/get/create/update 端点。
//! 该 API 分组未被 `docs/api/alistv3.openapi.yaml` 收录，
//! 字段形状以 `examples/alist/internal/model/role.go` 的 JSON 标签
//! 与 `examples/alist/server/handles/role.go` 的实际返回为准。

use serde::{Deserialize, Serialize};

/// 路径级权限条目：某个路径前缀上的权限位掩码。
///
/// 对应 Go `model.PermissionEntry`（`examples/alist/internal/model/role.go:10-13`）。
/// 既出现在 [`Role::permission_scopes`] 中，也是角色创建/更新请求体
/// `permission_scopes` 数组的元素类型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionEntry {
    /// 路径前缀，例如 `/admin`（服务端会做路径清洗，见 `internal/op/role.go` 的 `CreateRole`）。
    pub path: String,
    /// 权限位掩码（Go `int32`）；内置 `admin` 角色为 `0xFFFF`（65535）。
    pub permission: i32,
}

/// 角色实体：可绑定到用户的权限模板。
///
/// 对应 Go `model.Role`（`examples/alist/internal/model/role.go:16-25`）：
/// 由 `GET /api/admin/role/get` 直接返回，并作为 `GET /api/admin/role/list`
/// 的分页元素（见 `examples/alist/server/handles/role.go:27,42`）。
/// Go 侧 `RawPermission` 带 `json:"-"`，不出现在 JSON 中，故不建模。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Role {
    /// 角色 ID（Go `uint`，64 位平台为 64 位无符号，数据库自增主键）；
    /// 内置角色 `admin`/`guest` 不可改删。
    pub id: u64,
    /// 角色名（服务端要求唯一）。
    pub name: String,
    /// 角色描述；无描述时为空字符串。
    pub description: String,
    /// 是否为默认角色（新用户注册时自动绑定，对应服务端设置项 `default_role`）。
    pub default: bool,
    /// 各路径前缀上的权限条目。
    ///
    /// 服务端在数据库中无权限记录时会返回 `null`（Go 侧 `AfterFind` 将空的
    /// `RawPermission` 反序列化为 nil 切片，见 `internal/model/role.go:42-53`），
    /// 此处归约为空 [`Vec`]。
    #[serde(default, deserialize_with = "null_to_default")]
    pub permission_scopes: Vec<PermissionEntry>,
}

/// 将 JSON `null` 归约为 `T::default()`（schema 约定见 `docs/design.md` §5）。
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

    /// 正向钉扎：按 Go `model.Role` 的 JSON 标签形状构造角色对象。
    ///
    /// 示例取自 `examples/alist/internal/model/role.go:16-25` 与
    /// `internal/op/role.go:29-32`（内置 admin 角色的权限条目为
    /// `{Path: "/", Permission: 0xFFFF}`）。
    #[test]
    fn role_decodes_go_model_shape() {
        let role: Role = serde_json::from_value(serde_json::json!({
            "id": 1,
            "name": "admin",
            "description": "built-in admin role",
            "default": true,
            "permission_scopes": [
                { "path": "/", "permission": 65535 }
            ]
        }))
        .unwrap();
        assert_eq!(role.id, 1);
        assert_eq!(role.name, "admin");
        assert_eq!(role.description, "built-in admin role");
        assert!(role.default);
        assert_eq!(
            role.permission_scopes,
            vec![PermissionEntry {
                path: "/".to_owned(),
                permission: 65535,
            }]
        );
    }

    /// 显式 `null` 兼容：`AfterFind` 将空 `RawPermission` 置为 nil 切片，
    /// 服务端序列化为 `"permission_scopes": null`（`internal/model/role.go:42-45`）。
    #[test]
    fn role_tolerates_null_permission_scopes() {
        let role: Role = serde_json::from_value(serde_json::json!({
            "id": 2,
            "name": "guest",
            "description": "",
            "default": false,
            "permission_scopes": null
        }))
        .unwrap();
        assert_eq!(role.permission_scopes, Vec::<PermissionEntry>::new());
    }

    /// 兼容钉扎：新增字段缺失时使用默认值（跨版本兼容，`docs/design.md` §5.3）。
    #[test]
    fn role_tolerates_missing_permission_scopes() {
        let role: Role = serde_json::from_value(serde_json::json!({
            "id": 3,
            "name": "viewer",
            "description": "只读角色",
            "default": false
        }))
        .unwrap();
        assert_eq!(role.permission_scopes, Vec::<PermissionEntry>::new());
    }

    /// 列表包裹形态：`ListRoles` 返回 `common.PageResp{Content: roles, Total: total}`
    /// （`examples/alist/server/handles/role.go:27`），复用 [`crate::schema::common::PageResp`]。
    #[test]
    fn role_list_decodes_page_resp_wrapper() {
        let page: crate::schema::common::PageResp<Role> =
            serde_json::from_value(serde_json::json!({
                "content": [
                    {
                        "id": 1,
                        "name": "admin",
                        "description": "",
                        "default": true,
                        "permission_scopes": [{ "path": "/", "permission": 65535 }]
                    },
                    {
                        "id": 2,
                        "name": "guest",
                        "description": "",
                        "default": false,
                        "permission_scopes": null
                    }
                ],
                "total": 2
            }))
            .unwrap();
        assert_eq!(page.total, 2);
        assert_eq!(page.content.len(), 2);
        assert_eq!(page.content[0].name, "admin");
        assert!(page.content[0].permission_scopes[0].permission == 65535);
        assert!(page.content[1].permission_scopes.is_empty());
    }

    /// 序列化键名钉扎：`PermissionEntry` 作为创建/更新请求体数组元素时，
    /// JSON 键必须为 `path`/`permission`（Go JSON 标签，`internal/model/role.go:11-12`）。
    #[test]
    fn permission_entry_serializes_with_api_field_names() {
        let entry = PermissionEntry {
            path: "/data".to_owned(),
            permission: 15,
        };
        assert_eq!(
            serde_json::to_value(&entry).unwrap(),
            serde_json::json!({ "path": "/data", "permission": 15 })
        );
    }

    /// 序列化键名钉扎：`Role` 的 JSON 键与 Go 结构体标签一致（含 `default`）。
    #[test]
    fn role_serializes_with_api_field_names() {
        let role = Role {
            id: 4,
            name: "editor".to_owned(),
            description: String::new(),
            default: false,
            permission_scopes: vec![PermissionEntry {
                path: "/data".to_owned(),
                permission: 15,
            }],
        };
        assert_eq!(
            serde_json::to_value(&role).unwrap(),
            serde_json::json!({
                "id": 4,
                "name": "editor",
                "description": "",
                "default": false,
                "permission_scopes": [{ "path": "/data", "permission": 15 }]
            })
        );
    }
}
