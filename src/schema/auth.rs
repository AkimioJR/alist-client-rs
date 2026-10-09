//! auth 认证域数据模型。
//!
//! 覆盖登录（明文/哈希）`LoginRequest`/`LoginResponse`、注册 `RegisterRequest`、
//! 两步验证 `Generate2FaResponse`/`Verify2FaRequest`，以及 `/api/me` 的 `UserResponse`
//! （含 [`MeResponse`] 别名）。
//!
//! ## 字段形状来源
//!
//! - AList OpenAPI 规范的 auth 分组与 `/api/me` 路径（登录/2FA/me 示例）；
//! - AList 服务端认证处理模块：`LoginReq`、登录响应
//!   `gin.H{"token", "device_key"}`、`RegisterReq`、
//!   `UserResp`、`Verify2FAReq` 与
//!   `Generate2FA` 响应；
//! - AList 服务端 model.User 数据模型的 JSON 字段；
//! - AList 服务端 model.Role 数据模型的 `PermissionEntry`。
//!
//! ## 跨版本兼容
//!
//! - `/api/me` 的 `role`：服务端底层模型为角色 ID 列表，
//!   但老服务器与 OpenAPI 示例（`role: 2`）返回单值 int，历史部署还可能返回
//!   `null`（服务端 nil 切片序列化结果）；统一展开为 `Vec<i32>`；
//! - `role_names`/`permissions` 为服务端新增字段：老服务器不返回，
//!   新服务器在无角色时返回服务端 nil 切片序列化的 `null`，需同时容忍缺失与显式 `null`；
//! - `device_key` 为新版本登录响应新增字段，老服务器仅返回 `token`。

#[cfg(feature = "auth-schema")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// `/api/auth/login` 与 `/api/auth/login/hash` 的登录请求体。
///
/// 对应 AList 服务端认证处理模块的 `LoginReq`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct LoginRequest {
    /// 用户名。
    pub username: String,
    /// 登录密码。
    ///
    /// 对于 `/api/auth/login` 传入明文（服务端会做静态盐 SHA-256 哈希）；
    /// 对于 `/api/auth/login/hash` 传入预哈希值
    /// `sha256(密码-https://github.com/alist-org/alist)`。
    pub password: String,
    /// 可选的两步验证码。
    ///
    /// 启用 2FA 的账号登录时必填。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub otp_code: Option<String>,
}

/// 登录成功响应数据（`/api/auth/login` 与 `/api/auth/login/hash` 共用）。
///
/// 对应 AList 服务端登录响应 `gin.H{"token": token, "device_key": key}`；
/// 与 [`crate::Client`] 内部自动刷新 token 的登录实现使用同一 JSON 形状。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginResponse {
    /// 临时 JWT token，放入 `Authorization` 头使用。
    pub token: String,
    /// 当前登录设备键。
    ///
    /// 从 AList `v3.52.0` 起新增会话设备键；老服务器缺失时为 [`None`]。
    #[serde(default)]
    pub device_key: Option<String>,
}

/// `/api/auth/register` 的注册请求体。
///
/// 对应 AList 服务端认证处理模块的 `RegisterReq`；该端点不在 OpenAPI 规范中。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct RegisterRequest {
    /// 新用户名。
    pub username: String,
    /// 明文密码。
    ///
    /// 服务端注册时自行加盐哈希。
    pub password: String,
}

/// `/api/auth/2fa/generate` 的响应数据。
///
/// 对应 AList 服务端生成 2FA 响应结构。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct Generate2FaResponse {
    /// 二维码 PNG 图片的 data URL（`data:image/png;base64,...`）。
    pub qr: String,
    /// TOTP 密钥。
    ///
    /// 交由 `/api/auth/2fa/verify` 校验后才正式启用。
    pub secret: String,
}

/// `/api/auth/2fa/verify` 的请求体。
///
/// 对应 AList 服务端认证处理模块的 `Verify2FAReq`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct Verify2FaRequest {
    /// 用户从认证器 App 中读取的当前 TOTP 验证码。
    pub code: String,
    /// [`Generate2FaResponse::secret`] 返回的 2FA 密钥。
    pub secret: String,
}

/// 按路径前缀划分的权限位掩码条目。
///
/// 对应 AList 服务端 model.Role 数据模型的 `PermissionEntry`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct PermissionEntry {
    /// 路径前缀，例如 `/movies`。
    pub path: String,
    /// 该路径前缀上的权限位掩码。
    pub permission: i32,
}

/// `/api/me` 返回的当前用户信息。
///
/// 对应 AList 服务端认证处理模块的 `UserResponse`（内嵌 `model.User` 的 JSON 字段；
/// `password` 由处理函数置空）。新服务器追加的 `role_names`/`permissions` 字段与
/// `role` 的历史形状均做了兼容处理，详见模块文档。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "auth-schema")]
pub struct UserResponse {
    /// 数字用户 ID。
    ///
    /// 对应服务端用户主键 ID。
    pub id: u64,
    /// 用户名。
    pub username: String,
    /// 密码字段。
    ///
    /// `/api/me` 恒为空字符串，个别部署可能缺失或为 `null`。
    #[serde(default)]
    pub password: Option<String>,
    /// 用户根目录路径。
    pub base_path: String,
    /// 角色 ID 列表。
    ///
    /// 兼容数组、单值 int 与 `null` 三种历史形状。
    #[serde(deserialize_with = "deserialize_role_ids")]
    pub role: Vec<i32>,
    /// 账号是否被禁用。
    pub disabled: bool,
    /// 聚合权限位掩码。
    pub permission: i32,
    /// SSO 平台用户 ID。
    ///
    /// 未绑定时为空字符串或 [`None`]。
    #[serde(default)]
    pub sso_id: Option<String>,
    /// 是否已启用两步验证。
    pub otp: bool,
    /// 角色名称列表。
    ///
    /// 从 AList `v3.46.0` 起新增；老版本缺失或 `null` 时归约为空列表。
    #[serde(default, deserialize_with = "null_to_default")]
    pub role_names: Vec<String>,
    /// 按路径划分的权限条目。
    ///
    /// 从 AList `v3.46.0` 起新增；老版本缺失或 `null` 时归约为空列表。
    #[serde(default, deserialize_with = "null_to_default")]
    pub permissions: Vec<PermissionEntry>,
}

/// [`UserResponse`] 的别名，与 `/api/me` 端点名称对应。
#[cfg(feature = "auth-schema")]
pub type MeResponse = UserResponse;

/// 将 `role` 字段的三种历史形状（数组 / 单值 int / `null`）统一展开为 `Vec<i32>`。
#[cfg(feature = "auth-schema")]
fn deserialize_role_ids<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum RoleIds {
        Many(Vec<i32>),
        One(i32),
    }

    match Option::<RoleIds>::deserialize(deserializer)? {
        Some(RoleIds::Many(values)) => Ok(values),
        Some(RoleIds::One(value)) => Ok(vec![value]),
        None => Ok(Vec::new()),
    }
}

/// 将显式 `null` 归约为 `T::default()`（集合字段兼容 Go nil 切片序列化的 `null`）。
#[cfg(feature = "auth-schema")]
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + DeserializeOwned,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::Response;

    /// 正向钉扎：openapi `POST /api/auth/login` 返回示例。
    #[test]
    fn login_response_decodes_openapi_example() {
        // 示例来源：AList OpenAPI 规范的 /api/auth/login 200 响应
        let resp: Response<LoginResponse> = serde_json::from_value(serde_json::json!({
            "code": 200,
            "message": "success",
            "data": { "token": "abcd" }
        }))
        .unwrap();
        assert_eq!(resp.data.token, "abcd");
        // 老服务器不返回 device_key
        assert_eq!(resp.data.device_key, None);
    }

    /// 兼容钉扎：新版本登录响应携带 `device_key`。
    #[test]
    fn login_response_tolerates_device_key_from_go_source() {
        let resp: LoginResponse = serde_json::from_value(serde_json::json!({
            "token": "abcd",
            "device_key": "MD5(1-client)"
        }))
        .unwrap();
        assert_eq!(resp.device_key.as_deref(), Some("MD5(1-client)"));
    }

    /// 序列化键名钉扎：`LoginRequest` 与 openapi 请求示例键一致，`otp_code` 缺省时跳过。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn login_request_serializes_with_api_field_names() {
        // 示例来源：AList OpenAPI 规范的 /api/auth/login 请求示例
        let req = LoginRequest {
            username: "akimio".to_owned(),
            password: "JuXQMCe4m6LstB".to_owned(),
            otp_code: None,
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "username": "akimio",
                "password": "JuXQMCe4m6LstB"
            })
        );

        let with_otp = LoginRequest {
            otp_code: Some("123456".to_owned()),
            ..req
        };
        assert_eq!(
            serde_json::to_value(&with_otp).unwrap(),
            serde_json::json!({
                "username": "akimio",
                "password": "JuXQMCe4m6LstB",
                "otp_code": "123456"
            })
        );
    }

    /// 序列化键名钉扎：`RegisterRequest` 键与服务端 `RegisterReq` 一致。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn register_request_serializes_with_api_field_names() {
        let req = RegisterRequest {
            username: "string".to_owned(),
            password: "string".to_owned(),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "username": "string",
                "password": "string"
            })
        );
    }

    /// 正向钉扎：openapi `POST /api/auth/2fa/generate` 返回示例。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn generate_2fa_response_decodes_openapi_example() {
        // 示例来源：AList OpenAPI 规范的 /api/auth/2fa/generate 200 响应
        let resp: Generate2FaResponse = serde_json::from_value(serde_json::json!({
            "qr": "data:image/png;base64,iVBORw0KGgoAAAANSUhE",
            "secret": "RPQZG4MDS3"
        }))
        .unwrap();
        assert_eq!(resp.qr, "data:image/png;base64,iVBORw0KGgoAAAANSUhE");
        assert_eq!(resp.secret, "RPQZG4MDS3");
    }

    /// 序列化键名钉扎：`Verify2FaRequest` 键与 OpenAPI 请求示例（`code`/`secret`）一致。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn verify_2fa_request_serializes_with_api_field_names() {
        let req = Verify2FaRequest {
            code: "123456".to_owned(),
            secret: "RPQZG4MDS3".to_owned(),
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "code": "123456",
                "secret": "RPQZG4MDS3"
            })
        );
    }

    /// 正向钉扎：OpenAPI `/api/me` 返回示例；`role` 钉住老服务器单值 int 形状。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn me_response_decodes_openapi_example_with_single_int_role() {
        // 示例来源：AList OpenAPI 规范的 /api/me 200 响应
        let me: MeResponse = serde_json::from_value(serde_json::json!({
            "id": 1,
            "username": "admin",
            "password": "",
            "base_path": "/",
            "role": 2,
            "disabled": false,
            "permission": 0,
            "sso_id": "",
            "otp": true
        }))
        .unwrap();
        assert_eq!(me.id, 1);
        assert_eq!(me.username, "admin");
        assert_eq!(me.password.as_deref(), Some(""));
        assert_eq!(me.base_path, "/");
        assert_eq!(me.role, vec![2]);
        assert!(!me.disabled);
        assert_eq!(me.permission, 0);
        assert_eq!(me.sso_id.as_deref(), Some(""));
        assert!(me.otp);
        assert!(me.role_names.is_empty());
        assert!(me.permissions.is_empty());
    }

    /// 正向钉扎：服务端源码形状——`role` 为数组、`permissions` 携带路径权限条目。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn me_response_decodes_go_shape_with_array_role_and_permissions() {
        // 形状来源：AList 服务端认证处理模块 UserResp 与 model.Role 数据模型 PermissionEntry
        let me: MeResponse = serde_json::from_value(serde_json::json!({
            "id": 2,
            "username": "user",
            "password": "",
            "base_path": "/",
            "role": [2, 3],
            "disabled": false,
            "permission": 65535,
            "sso_id": "",
            "otp": false,
            "role_names": ["admin", "general"],
            "permissions": [{ "path": "/", "permission": 65535 }]
        }))
        .unwrap();
        assert_eq!(me.role, vec![2, 3]);
        assert_eq!(
            me.role_names,
            vec!["admin".to_owned(), "general".to_owned()]
        );
        assert_eq!(
            me.permissions,
            vec![PermissionEntry {
                path: "/".to_owned(),
                permission: 65535
            }]
        );
    }

    /// 兼容钉扎：老服务器响应缺少 `role_names`/`permissions`/`sso_id` 等新增字段。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn me_response_tolerates_missing_optional_fields() {
        let me: MeResponse = serde_json::from_value(serde_json::json!({
            "id": 2,
            "username": "guest",
            "base_path": "/",
            "role": [],
            "disabled": false,
            "permission": 0,
            "otp": false
        }))
        .unwrap();
        assert!(me.role.is_empty());
        assert_eq!(me.role_names, Vec::<String>::new());
        assert_eq!(me.permissions, Vec::<PermissionEntry>::new());
        assert_eq!(me.sso_id, None);
        assert_eq!(me.password, None);
    }

    /// 兼容钉扎：真实部署服务器会把 Go nil 切片序列化为显式 `null`
    /// （`role_names`/`permissions`），`role` 为 `null` 时归约为空列表。
    #[cfg(feature = "auth-schema")]
    #[test]
    fn me_response_tolerates_null_collection_fields() {
        let me: MeResponse = serde_json::from_value(serde_json::json!({
            "id": 1,
            "username": "akimio",
            "password": "",
            "base_path": "/",
            "role": null,
            "disabled": false,
            "permission": 65535,
            "sso_id": "",
            "otp": false,
            "role_names": null,
            "permissions": null
        }))
        .unwrap();
        assert!(me.role.is_empty());
        assert_eq!(me.role_names, Vec::<String>::new());
        assert!(me.permissions.is_empty());
    }
}
