//! admin-user 用户域数据模型。
//!
//! 本文件建模 `/api/admin/user` 组端点的用户条目 [`AdminUser`]，对应
//! `examples/alist/internal/model/user.go` 的 `model.User`（JSON 导出字段），
//! 由 `GET /api/admin/user/list`（分页形态 `{"content": [...], "total": N}` 的元素）与
//! `GET /api/admin/user/get`（单个对象）返回。字段示例取自
//! `docs/api/alistv3.openapi.yaml` 的 admin/user 组与 `docs/api/alistv3.md`
//! 的 `# admin/user` 分组。
//!
//! 创建/更新请求体（`POST /api/admin/user/create`、`/api/admin/user/update`）不在此定义：
//! 请求形状由端点构建器 `src/endpoint/admin/user/create.rs` 与 `update.rs` 的
//! `Request` 字段承载（请求体由 `EndpointRequest` 派生宏生成）。
//!
//! ## 兼容性说明
//!
//! - `role`：新版本服务端为数组（Go `model.Roles []int`，
//!   `examples/alist/internal/model/roles.go:9`）；老版本服务端与 openapi 文档示例为
//!   单值整数（如 `"role": 2`）。[`AdminUser::role`] 统一反序列化为 `Vec<i32>`：
//!   单值自动展开为单元素数组，缺失与显式 `null` 归约为空数组。
//! - `sso_id`：服务端较新字段；缺失或显式 `null` 时归约为空字符串。

use serde::{Deserialize, Serialize};

/// 将显式 JSON `null` 归约为类型默认值的反序列化辅助。
///
/// 服务端（尤其老版本）会对新增字段返回 `null`；字段缺失由 `#[serde(default)]` 接住。
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// 将单值整数、整数数组与显式 `null` 统一反序列化为角色 ID 数组。
fn deserialize_roles<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum RawRoles {
        /// 新版本服务端的数组形状（Go `model.Roles []int`）。
        Many(Vec<i32>),
        /// 老版本服务端与 openapi 文档示例的单值形状。
        One(i32),
    }
    match Option::<RawRoles>::deserialize(deserializer)? {
        Some(RawRoles::Many(values)) => Ok(values),
        Some(RawRoles::One(value)) => Ok(vec![value]),
        None => Ok(Vec::new()),
    }
}

/// AList 用户条目。
///
/// 对应 `examples/alist/internal/model/user.go` 的 `model.User` JSON 导出字段
/// （`PwdHash`/`PwdTS`/`Salt`/`OtpSecret`/`Authn` 等私有字段带 `json:"-"`，不出现在响应中）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminUser {
    /// 用户 ID；数据库自增主键（对应 Go `model.User.ID`）。
    pub id: i64,
    /// 用户名，全局唯一（对应 Go `model.User.Username`）。
    pub username: String,
    /// 明文密码字段（对应 Go `model.User.Password`）。
    ///
    /// 真实口令以加盐散列存储在服务端私有字段中；列表/详情响应中该字段恒为空字符串。
    pub password: String,
    /// 用户可见的根目录路径（对应 Go `model.User.BasePath`）。
    pub base_path: String,
    /// 角色 ID 列表（对应 Go `model.User.Role`，`model.Roles []int`）。
    ///
    /// 内置角色：`0` 普通用户、`1` 访客、`2` 管理员；
    /// 老版本单值形状自动展开为单元素数组。
    #[serde(default, deserialize_with = "deserialize_roles")]
    pub role: Vec<i32>,
    /// 是否禁用该账号（对应 Go `model.User.Disabled`）。
    #[serde(default)]
    pub disabled: bool,
    /// 权限位掩码（对应 Go `model.User.Permission`）。
    ///
    /// 按位控制可见隐藏文件、免密码访问、离线下载、上传、重命名/移动/复制/删除、
    /// WebDAV 与 FTP 读写、压缩包读取与解压、路径限制、MCP 读写等能力，
    /// 位定义见 `examples/alist/internal/model/user.go` 的 `Can*` 系列方法。
    #[serde(default)]
    pub permission: i32,
    /// SSO 平台唯一标识（对应 Go `model.User.SsoID`）；未绑定 SSO 时为空字符串。
    #[serde(default, deserialize_with = "null_to_default")]
    pub sso_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::PageResp;

    /// 正向钉扎：`docs/api/alistv3.md` `# admin/user` GET「列出所有用户」返回示例
    /// （与 `docs/api/alistv3.openapi.yaml` 的 `/api/admin/user/list` 示例一致），
    /// 验证 `PageResp` 分页包裹形态与 `role` 单值形状的展开。
    #[test]
    fn admin_user_list_example_decodes_page_resp() {
        let page: PageResp<AdminUser> = serde_json::from_value(serde_json::json!({
            "content": [
                {
                    "id": 1, "username": "admin", "password": "", "base_path": "/",
                    "role": 2, "disabled": false, "permission": 0, "sso_id": ""
                },
                {
                    "id": 2, "username": "guest", "password": "", "base_path": "/",
                    "role": 1, "disabled": true, "permission": 0, "sso_id": ""
                },
                {
                    "id": 3, "username": "N", "password": "", "base_path": "/",
                    "role": 0, "disabled": false, "permission": 256, "sso_id": ""
                }
            ],
            "total": 3
        }))
        .unwrap();
        assert_eq!(page.total, 3);
        assert_eq!(page.content.len(), 3);
        assert_eq!(page.content[0].username, "admin");
        assert_eq!(page.content[0].role, vec![2]); // 文档单值形状展开为数组
        assert!(page.content[1].disabled);
        assert_eq!(page.content[1].role, vec![1]);
        assert_eq!(page.content[2].permission, 256);
        assert_eq!(page.content[2].role, vec![0]);
    }

    /// 正向钉扎：`docs/api/alistv3.md` `# admin/user` GET「列出某个用户」返回示例
    /// （`/api/admin/user/get`，`role` 为单值 `2`）。
    #[test]
    fn admin_user_get_example_decodes_single_value_role() {
        let user: AdminUser = serde_json::from_value(serde_json::json!({
            "id": 1, "username": "admin", "password": "", "base_path": "/",
            "role": 2, "disabled": false, "permission": 0, "sso_id": ""
        }))
        .unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "admin");
        assert_eq!(user.base_path, "/");
        assert_eq!(user.role, vec![2]);
        assert!(!user.disabled);
        assert_eq!(user.sso_id, "");
    }

    /// 兼容钉扎：新版本服务端 `role` 为数组形状（Go `model.Roles []int`，
    /// `examples/alist/internal/model/roles.go`）。
    #[test]
    fn admin_user_accepts_array_role() {
        let user: AdminUser = serde_json::from_value(serde_json::json!({
            "id": 4, "username": "alice", "password": "", "base_path": "/data",
            "role": [3, 4], "disabled": false, "permission": 60, "sso_id": ""
        }))
        .unwrap();
        assert_eq!(user.role, vec![3, 4]);
    }

    /// 兼容钉扎：老版本服务端不含较新的 `sso_id` 字段时归约为空字符串。
    #[test]
    fn admin_user_tolerates_missing_optional_fields() {
        let user: AdminUser = serde_json::from_value(serde_json::json!({
            "id": 5, "username": "bob", "password": "", "base_path": "/",
            "role": 0, "disabled": false, "permission": 0
        }))
        .unwrap();
        assert_eq!(user.sso_id, "");
        assert_eq!(user.role, vec![0]);
        assert_eq!(user.permission, 0);
        assert!(!user.disabled);
    }

    /// 兼容钉扎：显式 `null` 归约为默认值（`role` → 空数组、`sso_id` → 空字符串）。
    #[test]
    fn admin_user_tolerates_null_fields() {
        let user: AdminUser = serde_json::from_value(serde_json::json!({
            "id": 6, "username": "carol", "password": "", "base_path": "/",
            "role": null, "disabled": false, "permission": 0, "sso_id": null
        }))
        .unwrap();
        assert!(user.role.is_empty());
        assert_eq!(user.sso_id, "");
    }

    /// 序列化键名钉扎：[`AdminUser`] 的 JSON 键与 API 字段名一致。
    #[test]
    fn admin_user_serializes_with_api_field_names() {
        let user = AdminUser {
            id: 1,
            username: "admin".to_owned(),
            password: String::new(),
            base_path: "/".to_owned(),
            role: vec![2],
            disabled: false,
            permission: 0,
            sso_id: String::new(),
        };
        assert_eq!(
            serde_json::to_value(&user).unwrap(),
            serde_json::json!({
                "id": 1, "username": "admin", "password": "", "base_path": "/",
                "role": [2], "disabled": false, "permission": 0, "sso_id": ""
            })
        );
    }
}
