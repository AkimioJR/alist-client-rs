//! auth 认证域数据模型。
//!
//! 涵盖 LoginReq/LoginResp、RegisterReq、2FA 与 /api/me 用户模型等。
//! schema 约定（`#[serde(default)]`、可选字段、示例 JSON 钉扎测试）见 `docs/design.md`；
//! 字段形状以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist` Go 源码为准。
