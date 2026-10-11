//! admin-session 会话管理域数据模型。
//!
//! 管理员会话列表条目与 auth 域的会话模型在服务端为同一 JSON 形状：
//! 服务端 session 处理模块的 `SessionResp` 同时作为
//! `GET /api/admin/session/list`（管理员列表，总会携带 `user_id`）与
//! 当前用户会话列表端点的响应元素。故本域直接复用 auth 域模型，
//! 以类型别名引入，不重复定义；
//! `admin-session-schema` feature 已声明拉取 `auth-schema` 以保证该类型可用。
//!
//! `evict` 端点的请求体为单字段 `session_id` 字符串对象，直接由端点构建器生成。

/// 管理员会话列表条目。
///
/// 与 auth 域 [`SessionResp`](crate::schema::auth::SessionResp) 为同一 JSON 形状
/// （服务端 session 处理模块的同一 `SessionResp` 结构体，从 AList `v3.52.0` 起新增），
/// 此处以类型别名引入；字段语义、`user_id` 的省略语义与版本说明见 auth 域文档。
/// 管理员列表（`GET /api/admin/session/list`）由服务端填入 `user_id`，
/// 当前用户接口则省略该字段——两种形状均由 auth 域模型的 [`Option<u64>`] 建模覆盖。
pub type SessionResp = crate::schema::auth::SessionResp;

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：管理员会话列表条目形状（服务端总会填入 `user_id`）。
    #[test]
    fn admin_session_resp_decodes_admin_list_shape() {
        // 形状来源：AList 服务端 session 处理模块的管理员会话列表处理器
        // （实现为 `ListSessions`，元素为服务端 `SessionResp`）
        let session: SessionResp = serde_json::from_value(serde_json::json!({
            "session_id": "device-admin-1",
            "user_id": 1,
            "last_active": 1710000000,
            "status": 1,
            "ua": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
            "ip": "192.168.1.10"
        }))
        .unwrap();
        assert_eq!(session.session_id, "device-admin-1");
        assert_eq!(session.user_id, Some(1));
        assert_eq!(session.last_active, 1710000000);
        assert_eq!(session.status, 1);
        assert_eq!(
            session.ua,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"
        );
        assert_eq!(session.ip, "192.168.1.10");
    }

    /// 兼容钉扎：条目缺失或显式 `null` 的 `user_id` 时解码为 [`None`]，
    /// 复用 auth 域模型的跨版本兼容语义。
    #[test]
    fn admin_session_resp_tolerates_missing_or_null_user_id() {
        let missing: SessionResp = serde_json::from_value(serde_json::json!({
            "session_id": "device-no-user",
            "last_active": 1710000000,
            "status": 1,
            "ua": "curl/8.0",
            "ip": "127.0.0.1"
        }))
        .unwrap();
        assert_eq!(missing.user_id, None);

        let null_user_id: SessionResp = serde_json::from_value(serde_json::json!({
            "session_id": "device-null-user",
            "user_id": null,
            "last_active": 1710000000,
            "status": 1,
            "ua": "curl/8.0",
            "ip": "127.0.0.1"
        }))
        .unwrap();
        assert_eq!(null_user_id.user_id, None);
    }
}
