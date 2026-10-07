//! fs端点：搜索文件或文件夹。
//!
//! 对应 `POST /api/fs/search`；依赖服务端索引（search 索引不可用时返回 SearchNotAvailable）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
