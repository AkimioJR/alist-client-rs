//! admin-message 消息队列域数据模型。
//!
//! 建模 `/api/admin/message` 端点族的数据形状：
//!
//! - [`Message`]：`get` 端点的响应模型（服务端待推送给前端的消息），
//!   对应 AList 服务端 message 模块的 `Message`；
//!   `send` 端点的请求体为单字段 `message` 字符串对象，直接由端点构建器生成。
//!
//! ## 字段形状来源
//!
//! 字段形状以 AList 服务端 message 模块（消息取用与投递处理器，实现为
//! `GetHandle`/`SendHandle`）及其 `Message` 数据模型为准；
//! AList OpenAPI 规范未收录该分组。消息队列用于服务端与前端之间的双向提醒
//! （例如部分驱动需要用户输入验证码时，通过该队列投递交互消息）。

use serde::{Deserialize, Serialize};

/// 服务端待推送消息。
///
/// 对应 AList 服务端 message 模块的 `Message`（`POST /api/admin/message/get`
/// 响应 `data`）。`type` 标识消息种类（如 `string` 文本消息），`content`
/// 为消息载荷；部分驱动通过该队列请求用户输入验证码等交互信息。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// 消息类型标识。
    ///
    /// 对应服务端 `Message.type`（JSON 键为保留字 `type`，以 raw identifier 命名字段）；
    /// 服务端内置用法为 `string`（文本消息）。
    pub r#type: String,
    /// 消息内容载荷（任意 JSON 值）。
    ///
    /// 对应服务端 `Message.content`（Go 空接口）；服务端未填充内容时序列化为
    /// `null`，缺失时归约为 [`serde_json::Value::Null`]。
    #[serde(default)]
    pub content: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正向钉扎：服务端投递文本消息的形状（内容载荷为字符串）。
    #[test]
    fn message_decodes_string_content_shape() {
        // 形状来源：AList 服务端 message 模块 `Message{Type, Content}`
        // （开发模式内置用例投递的 `string` 类型文本消息）
        let message: Message = serde_json::from_value(serde_json::json!({
            "type": "string",
            "content": "dev mode"
        }))
        .unwrap();
        assert_eq!(message.r#type, "string");
        assert_eq!(message.content, serde_json::json!("dev mode"));
    }

    /// 正向钉扎：内容载荷为任意 JSON 对象的形状。
    #[test]
    fn message_decodes_object_content_shape() {
        // 形状来源：AList 服务端 message 模块 `Message` 的 `content` 为
        // Go 空接口，可承载任意 JSON 值
        let message: Message = serde_json::from_value(serde_json::json!({
            "type": "captcha",
            "content": { "prompt": "input code", "timeout": 30 }
        }))
        .unwrap();
        assert_eq!(message.r#type, "captcha");
        assert_eq!(message.content["prompt"], "input code");
        assert_eq!(message.content["timeout"], 30);
    }

    /// 兼容钉扎：`content` 显式 `null` 与字段缺失时均归约为
    /// [`serde_json::Value::Null`]。
    #[test]
    fn message_tolerates_missing_or_null_content() {
        let null_content: Message =
            serde_json::from_value(serde_json::json!({ "type": "string", "content": null }))
                .unwrap();
        assert_eq!(null_content.content, serde_json::Value::Null);

        let missing_content: Message =
            serde_json::from_value(serde_json::json!({ "type": "string" })).unwrap();
        assert_eq!(missing_content.content, serde_json::Value::Null);
    }

    /// 序列化键名钉扎：序列化键与服务端 `Message` 的 JSON 键一致
    /// （`type` 为 JSON 保留字，以 raw identifier 直接映射）。
    #[test]
    fn message_serializes_with_api_field_names() {
        let message = Message {
            r#type: "string".to_owned(),
            content: serde_json::json!("dev mode"),
        };
        assert_eq!(
            serde_json::to_value(&message).unwrap(),
            serde_json::json!({ "type": "string", "content": "dev mode" })
        );
    }
}
