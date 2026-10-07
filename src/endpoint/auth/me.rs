//! auth端点：获取当前用户信息。
//!
//! 对应 `GET /api/me`；注意 Go 侧 `role` 为数组（`model.Roles []int`），以 examples/alist 源码为准。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
