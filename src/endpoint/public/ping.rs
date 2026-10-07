//! public端点：连通性检测。
//!
//! 对应 `GET /ping`；返回纯文本 `pong`，应使用 `Client::execute_text`（不走 JSON 信封）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
