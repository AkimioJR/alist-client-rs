//! fs 文件系统域数据模型。
//!
//! 涵盖 ObjResp/FsListResp、目录/重命名/移动/删除请求、归档与离线下载等模型。
//! schema 约定（`#[serde(default)]`、可选字段、示例 JSON 钉扎测试）见 `docs/design.md`；
//! 字段形状以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist` Go 源码为准。
