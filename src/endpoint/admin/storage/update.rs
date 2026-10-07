//! admin-storage端点：更新存储。
//!
//! 对应 `POST /api/admin/storage/update`；服务端可能返回 `data: null`，建议以 `Option<T>` 或 `()` 作为模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
