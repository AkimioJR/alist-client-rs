//! fs端点：流式上传。
//!
//! 对应 `PUT /api/fs/put`；请求头携带 File-Path/As-Task/Overwrite 等，直传成功 `data: null`，As-Task 时返回任务信息；上传实现不走派生宏 JSON body，需手工构建请求体后调用 execute。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。
