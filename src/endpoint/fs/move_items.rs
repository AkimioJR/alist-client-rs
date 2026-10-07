//! fs端点：移动文件。
//!
//! 对应 `POST /api/fs/move`；跨存储移动会被服务端拒绝（MoveBetweenTwoStorages）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
