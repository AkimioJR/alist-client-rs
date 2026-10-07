//! fs端点：列出目录内容。
//!
//! 对应 `POST /api/fs/list`；分页参数叠加 `PageReq`，响应含 content/total/readme/write 等。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
