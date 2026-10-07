//! auth端点：验证并启用两步验证。
//!
//! 对应 `POST /api/auth/2fa/verify`；响应 `data: null`，建议以 `()` 作为模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
