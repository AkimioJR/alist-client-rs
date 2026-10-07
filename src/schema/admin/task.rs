//! admin-task 任务域数据模型。
//!
//! 涵盖上传任务查询/操作请求与响应模型（复用 common::TaskInfo）。
//! schema 约定（`#[serde(default)]`、可选字段、示例 JSON 钉扎测试）见 `docs/design.md`；
//! 字段形状以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist` Go 源码为准。
