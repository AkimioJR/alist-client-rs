//! fs 上传子句柄。
//!
//! 对应 `examples/alist/server/router.go` 中 `PUT /api/fs/put`（流式上传）与
//! `PUT /api/fs/form`（表单上传）两个路由；上传行为细节见
//! `examples/alist/server/handles/fsup.go`（`File-Path`/`As-Task`/`Overwrite`
//! 等请求头与哈希校验头）。
//!
//! # 上传方案选型指南
//!
//! 本句柄提供了 4 种文件上传方式，请根据文件体积、数据源形态与网络场景选择：
//!
//! | 方法 | 对应端点 | 数据来源 | 内存特点 | 401 自动重登重试 | 适用场景 |
//! |---|---|---|---|---|---|
//! | [`put`](Upload::put) | `PUT /api/fs/put` | 内存字节（`impl Into<Bytes>`） | 需整体载入内存 | ✅ 支持 | 中小型文件（配置、文本、图片等，数据已在内存中） |
//! | `put_file` | `PUT /api/fs/put` | 本地文件（`tokio::fs::File`） | 极低（分块流式直传） | ❌ 不支持 | 本地磁盘大文件（ISO、视频、大压缩包等，防止 OOM） |
//! | `put_stream` | `PUT /api/fs/put` | 任意异步流（`AsyncRead + AsyncSeek`） | 极低（分块流式直传） | ❌ 不支持 | 管道流、网络流、动态生成流等已知长度的非文件流 |
//! | [`form`](Upload::form) | `PUT /api/fs/form` | 内存字节（`impl Into<Bytes>`） | 需整体载入内存 | ❌ 不支持 | 需模拟浏览器网页表单上传，或下游驱动仅支持表单时 |
//!
//! - **流式直传（推荐）**：`PUT /api/fs/put` 直接向服务端发送裸二进制数据流，无 multipart 表单协议开销。
//!   中小型内存数据优先选用 [`put`](Upload::put)（支持 token 过期自动刷新重试）；本地大文件优先选用
//!   `put_file`（零内存暴涨）。
//! - **表单上传**：`PUT /api/fs/form` 封装为 `multipart/form-data` 格式，与前端网页表单行为完全对齐，
//!   适用于特定只兼容表单的存储驱动场景。
//! - **特性门控**：`put_file` 与 `put_stream` 需要启用 `stream` feature。

pub mod form;
pub mod put;

/// fs 上传句柄。
///
/// 通过 [`Fs::upload`](super::Fs::upload) 获取，例如
/// `client.fs().upload().put(...)`。
pub struct Upload<'a> {
    client: &'a crate::Client,
}

impl<'a> Upload<'a> {
    /// 创建上传句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}
