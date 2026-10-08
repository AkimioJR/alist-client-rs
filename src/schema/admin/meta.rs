//! admin-meta 元信息域数据模型。
//!
//! 建模目录元信息规则（密码、写入、隐藏、说明与自定义响应头等），
//! 覆盖 `/api/admin/meta/list`、`/get`、`/create`、`/update`、`/delete` 五个端点的
//! 请求与响应数据。
//!
//! ## 字段形状来源
//!
//! - AList OpenAPI 规范的 admin/meta 分组（请求与响应示例 JSON）；
//! - AList 服务端 model.Meta 数据模型（字段保真仲裁来源，含 OpenAPI 未列出的 `header`/`header_sub` 两个字段）；
//! - AList 服务端元信息处理模块。
//!
//! `Meta` 同时用作响应条目与 `/create`、`/update` 的请求体。

use serde::{Deserialize, Serialize};

/// 目录元信息规则。
///
/// 对应 AList 服务端 model.Meta 数据模型：
/// 一条规则将密码/写入/隐藏/说明/自定义响应头等设置绑定到某个路径（可选择性
/// 应用到子目录）。同一路径仅能有一条规则（Go 侧 `gorm:"unique"` 标签）。
///
/// 服务端总是返回全部字段；为兼容不返回新字段（如 `header`/`header_sub`）的
/// 老版本服务器，所有字段均带 `#[serde(default)]`。
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Meta {
    /// 元信息 ID。
    ///
    /// 服务端自动分配；创建时应保持 `0`。对应服务端主键 ID。
    #[serde(default)]
    pub id: u64,
    /// 规则作用的目录路径。
    ///
    /// 同一路径唯一。对应服务端路径绑定字段。
    #[serde(default)]
    pub path: String,
    /// 目录密码。
    ///
    /// 空字符串表示不设密码。
    #[serde(default)]
    pub password: String,
    /// 密码是否应用于子目录。
    ///
    /// 对应 JSON 键 `p_sub`。
    #[serde(default)]
    pub p_sub: bool,
    /// 是否允许访客写入该目录。
    #[serde(default)]
    pub write: bool,
    /// 写权限是否应用于子目录。
    ///
    /// 对应 JSON 键 `w_sub`。
    #[serde(default)]
    pub w_sub: bool,
    /// 隐藏条目的匹配规则。
    ///
    /// 多条以 `\n` 分隔，每条为一个正则表达式；服务端创建/更新时会逐条校验正则合法性。
    #[serde(default)]
    pub hide: String,
    /// 隐藏规则是否应用于子目录。
    ///
    /// 对应 JSON 键 `h_sub`。
    #[serde(default)]
    pub h_sub: bool,
    /// 目录说明内容。
    ///
    /// 展示在目录页顶部。
    #[serde(default)]
    pub readme: String,
    /// 说明是否应用于子目录。
    ///
    /// 对应 JSON 键 `r_sub`。
    #[serde(default)]
    pub r_sub: bool,
    /// 自定义响应头。
    ///
    /// 每行一条 `Header: Value`；该字段未被 OpenAPI 规范收录，以服务端源码为准。
    #[serde(default)]
    pub header: String,
    /// 自定义响应头是否应用于子目录。
    ///
    /// 对应 JSON 键 `header_sub`；该字段未被 OpenAPI 规范收录，以服务端源码为准。
    #[serde(default)]
    pub header_sub: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::common::PageResponse;

    /// AList OpenAPI 规范 `/api/admin/meta/get` 的 200 响应示例：
    /// 不含 `header`/`header_sub`（openapi 未收录），应回退为默认值。
    #[test]
    fn meta_decodes_openapi_get_example() {
        let meta: Meta = serde_json::from_value(serde_json::json!({
            "id": 1,
            "path": "/a",
            "password": "c",
            "p_sub": false,
            "write": false,
            "w_sub": false,
            "hide": "",
            "h_sub": false,
            "readme": "",
            "r_sub": false
        }))
        .unwrap();
        assert_eq!(meta.id, 1);
        assert_eq!(meta.path, "/a");
        assert_eq!(meta.password, "c");
        assert!(!meta.p_sub);
        assert!(!meta.write);
        assert!(!meta.w_sub);
        assert_eq!(meta.hide, "");
        assert!(!meta.h_sub);
        assert_eq!(meta.readme, "");
        assert!(!meta.r_sub);
        // openapi 未收录的新字段缺失时回退默认值（跨版本兼容）
        assert_eq!(meta.header, "");
        assert!(!meta.header_sub);
    }

    /// AList 服务端 model.Meta 全字段形状：
    /// 含 OpenAPI 未列出的 `header`/`header_sub`。
    #[test]
    fn meta_decodes_full_go_model_shape() {
        let meta: Meta = serde_json::from_value(serde_json::json!({
            "id": 2,
            "path": "/docs",
            "password": "pw",
            "p_sub": true,
            "write": true,
            "w_sub": true,
            "hide": "\\.txt\nsecret",
            "h_sub": true,
            "readme": "# Hello",
            "r_sub": false,
            "header": "X-Custom: 1",
            "header_sub": true
        }))
        .unwrap();
        assert_eq!(meta.id, 2);
        assert_eq!(meta.path, "/docs");
        assert_eq!(meta.password, "pw");
        assert!(meta.p_sub);
        assert!(meta.write);
        assert!(meta.w_sub);
        assert_eq!(meta.hide, "\\.txt\nsecret");
        assert!(meta.h_sub);
        assert_eq!(meta.readme, "# Hello");
        assert!(!meta.r_sub);
        assert_eq!(meta.header, "X-Custom: 1");
        assert!(meta.header_sub);
    }

    /// 老版本服务器缺失部分字段时全部回退零值，不应报错。
    #[test]
    fn meta_tolerates_missing_fields() {
        let meta: Meta = serde_json::from_value(serde_json::json!({
            "id": 3,
            "path": "/"
        }))
        .unwrap();
        assert_eq!(meta.id, 3);
        assert_eq!(meta.path, "/");
        assert_eq!(meta.password, "");
        assert!(!meta.p_sub);
        assert!(!meta.write);
        assert_eq!(meta.header, "");
        assert!(!meta.header_sub);
    }

    /// 序列化键名与 Go JSON 标签一一对应（12 个字段全量输出）。
    #[test]
    fn meta_serializes_with_api_field_names() {
        let meta = Meta {
            id: 4,
            path: "/a".to_owned(),
            password: "c".to_owned(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&meta).unwrap(),
            serde_json::json!({
                "id": 4,
                "path": "/a",
                "password": "c",
                "p_sub": false,
                "write": false,
                "w_sub": false,
                "hide": "",
                "h_sub": false,
                "readme": "",
                "r_sub": false,
                "header": "",
                "header_sub": false
            })
        );
    }

    /// AList OpenAPI 规范 `/api/admin/meta/list` 的 200 响应示例：
    /// 分页包裹形态 `{"content": [...], "total": n}`（复用 [`PageResponse`]）。
    #[test]
    fn meta_list_decodes_page_response_example() {
        let resp: PageResponse<Meta> = serde_json::from_value(serde_json::json!({
            "content": [{
                "id": 1,
                "path": "/a",
                "password": "i",
                "p_sub": false,
                "write": false,
                "w_sub": false,
                "hide": "",
                "h_sub": false,
                "readme": "",
                "r_sub": false
            }],
            "total": 1
        }))
        .unwrap();
        assert_eq!(resp.total, 1);
        assert_eq!(resp.content.len(), 1);
        assert_eq!(resp.content[0].path, "/a");
        assert_eq!(resp.content[0].password, "i");
    }
}
