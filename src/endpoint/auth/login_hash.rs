//! auth端点：哈希登录获取 token。
//!
//! 对应 `POST /api/auth/login/hash`；密码需先拼接 `-https://github.com/alist-org/alist` 后取 SHA-256。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
