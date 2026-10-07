//! admin-storage 存储域数据模型。
//!
//! 涵盖 Storage 与创建/更新请求等模型；update 可能返回 null data。
//! schema 约定（`#[serde(default)]`、可选字段、示例 JSON 钉扎测试）见 `docs/design.md`；
//! 字段形状以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist` Go 源码为准。
