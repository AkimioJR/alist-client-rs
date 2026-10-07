//! auth端点：生成两步验证密钥。
//!
//! 对应 `POST /api/auth/2fa/generate`；返回二维码 data URL 与 TOTP 密钥。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
