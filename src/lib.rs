//! AList v3 API 的异步 Rust 客户端。
//!
//! 本 crate 将 [AList](https://github.com/AlistGo/alist) 的 JSON 响应（[`Response`](crate::schema::common::Response)）与文件系统/管理端点
//! 建模为强类型 Rust API：HTTP 构建、发送、响应解码、认证刷新与客户端限速统一由
//! [`Client`] 处理；各业务端点以「域句柄 + 请求构建器」的形式暴露；数据模型集中在
//! [`schema`] 下。实现约定详见仓库内 `docs/design.md`。
//!
//! # 快速上手
//!
//! 端点段落按对应 feature 门控（`# #[cfg(feature = ...)]`），裁剪 feature 后示例仍可编译：
//!
//! ```no_run
//! use alist_client::{Authentication, Client};
//!
//! # async fn example() -> alist_client::Result<()> {
//! // 1. 创建客户端：传入站点地址（支持带路径前缀的反代部署，末尾 `/` 自动归一化）
//! let client = Client::new("https://alist.example.com")?;
//!
//! // 2. 配置认证：用户名密码凭据在 token 缺失或 401/403 时自动重新登录并重试一次；
//! //    也可直接使用已有 token：Authentication::token("...")
//! let client =
//!     client.with_authentication(Authentication::username_password("admin", "password", None));
//!
//! // 3. 可选：客户端侧限速，保持相邻请求的间隔
//! let client = client.with_api_request_interval(std::time::Duration::from_millis(250));
//!
//! # #[cfg(feature = "fs")]
//! # {
//! // 4. 列出目录：必选参数在访问器处传入，可选参数走链式 setter，`.await` 直接解码
//! let page = client.fs().list("/").page(1).per_page(20).await?;
//! for obj in &page.content {
//!     println!("{}\t{} 字节", obj.name, obj.size);
//! }
//!
//! // 5. 上传文件：直传成功返回 `None`；`as_task(true)` 时返回后台任务信息
//! let resp = client
//!     .fs()
//!     .upload()
//!     .put("/data/demo.txt", b"hello alist".to_vec())
//!     .as_task(true)
//!     .await?;
//! if let Some(resp) = resp {
//!     println!("后台上传任务 ID: {}", resp.task.id);
//! }
//! # }
//!
//! # #[cfg(feature = "public")]
//! # {
//! // 6. 公开端点：`/ping` 返回纯文本，走 `send_text` 通道（不做响应解码）
//! let pong = client.public().ping().send_text().await?;
//! assert_eq!(pong, "pong");
//! # }
//! # Ok(())
//! # }
//! ```
//!
//! # 架构总览
//!
//! ```text
//! Client（src/client.rs）
//!   ├── Authentication    凭据：UsernamePassword（自动重登）/ Token
//!   ├── 请求执行          响应解码、HTTP 状态检查、限速、401/403 自动重试
//!   └── 域句柄（src/endpoint/）
//!         ├── client.auth()    → Auth     认证（登录/注册、2FA、当前用户）
//!         ├── client.fs()      → Fs       文件系统（列表、增删改、搜索、离线下载）
//!         │     ├── .archive() → Archive  压缩包浏览与解压
//!         │     └── .upload()  → Upload   流式/表单上传
//!         ├── client.public()  → Public   ping、公共设置
//!         └── client.admin()   → Admin    管理端（元信息/用户/存储/驱动/设置/任务/角色/标签）
//! ```
//!
//! - **请求构建器**：每个操作一个 `Request` 结构体（`src/endpoint/<组>/<端点>.rs`），
//!   由 `alist-client-derive` 的 `EndpointRequest` 派生宏生成 `build_request`、
//!   [`send`](crate::endpoint::fs::mkdir::Request::send)、`IntoFuture` 与可选参数
//!   setter；可直接 `.await` 执行强类型解码。列表端点在 `into-stream` feature 下
//!   还可调用 `.into_stream()` 得到自动翻页的条目流。
//! - **数据模型**：[`schema`] 按 API 域分组（`src/schema/<域>.rs`），serde 驱动，
//!   对老版本服务器的缺失/`null` 字段保持兼容；响应封装（[`Response`](crate::schema::common::Response)）、
//!   分页（[`PageResponse`](crate::schema::common::PageResponse)）、任务（`TaskInfo`）、上传（[`UploadResponse`](crate::schema::common::UploadResponse)）等共享形状在 [`schema::common`]。
//! - **错误语义**：AList 多数错误以 HTTP 200 + 响应 `code` 非 200 返回，[`Client`]
//!   统一转换为 [`Error::Api`]（状态码见 [`ApiStatusCode`]，内部错误分类见
//!   [`InternalErrorKind`]）；HTTP 非 2xx 转换为 [`Error::HttpStatus`]。
//! - **测试基建**：`test_support` 模块（仅测试构建）提供手搓的 mock 服务器。
//!
//! # 功能矩阵（feature flags）
//!
//! 默认 `default = ["all"]` 启用全部功能，可按需裁剪：
//!
//! | feature | 说明 |
//! |---|---|
//! | `auth` | 认证端点：登录/注册、2FA 生成与校验、当前用户信息 |
//! | `fs` | 文件系统端点：列表、新建/重命名/复制/移动/删除、搜索、离线下载、归档、上传 |
//! | `public` | 公开端点：`/ping`、公共设置 |
//! | `admin` | 聚合下列全部 `admin-*` 管理端子域 |
//! | `admin-meta` | 管理端·元信息（目录密码/说明等） |
//! | `admin-user` | 管理端·用户 |
//! | `admin-storage` | 管理端·存储驱动实例 |
//! | `admin-driver` | 管理端·驱动列表与配置模板 |
//! | `admin-setting` | 管理端·站点设置 |
//! | `admin-task` | 管理端·后台任务（上传/离线下载/复制等） |
//! | `admin-role` | 管理端·角色 |
//! | `admin-label` | 管理端·标签 |
//! | `admin-label-file-binding` | 管理端·标签-文件绑定 |
//! | `stream` | 响应体流式读取（`tokio-util`） |
//! | `into-stream` | 列表端点自动翻页流（`async-stream` + `futures`） |
//!
//! 启用任一端点 feature 会自动启用同名镜像 schema feature（如 `auth` → `auth-schema`），
//! 因此端点代码引用 [`schema`] 类型时无需额外条件编译；`admin-*` 子 feature 可脱离
//! `admin` 聚合单独启用。

pub(crate) mod client;
pub mod endpoint;
pub mod error;
pub mod schema;

#[cfg(test)]
pub(crate) mod test_support;

pub use client::{Authentication, Client};
pub use error::{ApiStatusCode, Error, InternalErrorKind, Result};
