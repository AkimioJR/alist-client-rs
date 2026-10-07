---
name: alist-endpoint-authoring
description: "在 alist-client crate 中实现单个 API 端点（src/endpoint/<组>/<端点>.rs）。涵盖 EndpointRequest 派生宏的完整属性面、一个操作一个文件的 Request 模板、句柄访问器写法、#[query] 与 into-stream 用法、/ping 与上传等手写特例、请求形状测试模板、过渡标记清理与自验命令清单。实现 auth/fs/public/admin-* 任一端点文件之前必读。"
---

# AList 端点实现指南

## Goal

在既有骨架（句柄 + 派生宏 + Client）之上，为 `src/endpoint/<组>/<端点>.rs` 的空壳文件补全「一个操作一个文件」的 Request 构建器实现，并配套请求形状测试。本文与 `docs/design.md` 冲突时以 `docs/design.md` 为准，以仓库实际代码为准。

## 硬性禁令（违反即返工）

1. **禁止执行任何 git 操作**（add/commit/push 等）。提交由工作流统一完成。
2. **禁止裸跑 `cargo fmt`**——它会把整个 crate（包括其他并行代理正在修改的文件）全部重排。检测用 `cargo +nightly fmt --check`；确需格式化时只格式化自己的文件：`cargo +nightly fmt -- <你的文件>...`。
3. **禁止修改**：`examples/`、`docs/api/`、根 `Cargo.toml`（含 `[features]` 矩阵）、`src/client.rs`、`src/error.rs`、`src/endpoint.rs`、`src/endpoint/<组>.rs` 句柄结构、`alist-client-derive/`。
4. **所有权边界**：负责 `<组>` 的代理只允许改 `src/endpoint/<组>/*.rs` 与 `src/schema/<域>.rs`，外加本文第 8 节要求的 `#[expect(dead_code)]` 联动清理。
5. 一个端点文件**只放一个** `pub struct Request`（派生宏会生成文件私有的 `QueryParams`/`RequestBody` 结构体，同名冲突）。

## 实现前必读与事实来源

- `docs/design.md`：模板与全部约定的唯一来源（§3 派生宏、§4 端点模板、§6 错误语义、§7 命令）。
- HTTP 方法与路径核对顺序：
  1. `docs/api/alistv3.openapi.yaml`（路径索引：`grep -n '^  /' docs/api/alistv3.openapi.yaml`）；
  2. `docs/api/alistv3.md`（按 `# auth`、`# fs`、`# public`、`# admin/<子域>` 分组，`## GET/POST/PUT <描述>` 一节对应一个操作；该文件中文为 GBK 乱码，但方法/路径/JSON 示例可读）；
  3. **不确定时以 `examples/alist/server/router.go` 为准**。`g.Any(...)` 的读端点按 GET 处理、写端点按 POST 处理（router.go:223-228）。
- 三组不在 openapi 中的端点（`admin-role`、`admin-label`、`admin-label-file-binding`）完全以 router.go 为准，逐条路由：
  - `/api/admin/role`：GET `/list`、GET `/get`、POST `/create`、POST `/update`、POST `/delete`（router.go:149-154）；
  - label：GET list/get 挂在 `/api/label`（`_label`，router.go:119、256-259），POST create/update/delete 挂在 `/api/admin/label`（router.go:204-207）；
  - label_file_binding：GET `/get`、GET `/get_file_by_label` 挂在 `/api/label_file_binding`（router.go:120、261-264），GET `/list` 与 POST `/create`、`/create_batch`、`/delete`、`/restore` 挂在 `/api/admin/label_file_binding`（router.go:209-214）；
  - admin/task 各操作对应 router.go:191 + `handles/task.go:123-226`（GET `/undone`、`/done`；POST `/info`、`/cancel`、`/delete`、`/retry`、`/clear_done`、`/clear_succeeded` 等），且任务按类别分子路径（`task.go:218-226`，如 `/api/admin/task/upload/undone`），实现时需对照决定路径。

## 派生宏属性面（速查，已核对于 alist-client-derive/src/endpoint_request.rs）

```rust
#[derive(EndpointRequest)]
#[endpoint(method = GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS,
           path = "/api/...",          // 自站点根起算的完整路径（含 /api 前缀；/ping 写 "/ping"）
           model = <类型>)]            // send()/IntoFuture 的解码目标
#[cfg_attr(feature = "into-stream", endpoint(into_stream = true, stream_item = <元素类型>))]
pub struct Request<'a> { ... }
```

- 多个 `#[endpoint(...)]` 属性按序合并，后出现的键覆盖先前（endpoint_request.rs:31-33）；这是 `cfg_attr` 引入 `into_stream` 的前提。**不要裸写 `into_stream = true`**，必须经 `cfg_attr(feature = "into-stream", ...)`。
- 字段属性：`#[query]`（进 URL 查询串，`Option::None` 时跳过）与 `#[endpoint(skip)]`（排除字段）。二者不能同时标注（endpoint_request.rs:331-336）。
- `client: &'a crate::Client` 字段必须存在且必须 `#[endpoint(skip)]`（endpoint_request.rs:213-228）。
- 派生生成物（无需手写）：
  - `pub(crate) fn build_request(&self) -> ::reqwest::RequestBuilder`：method + path + 查询串 + JSON body；**无 body 字段时不设 body、不带 JSON Content-Type**；无认证头（认证由 `Client::execute` 发送时注入）；
  - `pub async fn send(&self)` / `pub async fn send_raw<T: DeserializeOwned>(&self)`：经 `Client::execute` 解信封；
  - `IntoFuture`：`Output = crate::Result<model>`，可直接 `.await`；
  - **仅 `Option<T>` 字段**生成消费式 setter：`T = String` 时参数为 `impl Into<String>`（可直接传 `&str`），其余类型参数直接为内层类型 `T`；字段上的 `///` 文档自动带到 setter 上（endpoint_request.rs:434-468）；
  - 非 `Option` 的 Query/Body 字段**不生成 setter**，只能由 `Request::new` 传入（即必选参数）。
- `into_stream = true` 的前置条件：结构体必须存在 `page: Option<i32>` 字段（endpoint_request.rs:255-260，通常标 `#[query]`）、必须给出 `stream_item`；运行期要求 model 有 `content: Vec<stream_item>` 字段（`PageResp<T>` 形态）。流从 `page`（缺省 1）逐页 `page += 1` 产出 `content` 元素，空页停止（最后一页可能多一次确认请求）。
- 宏限制：仅命名字段结构体、仅生命周期泛型（不支持类型/常量泛型参数）。

## 完整模板（一个操作一个文件）

以下模板可直接复制，按注释替换。以 `src/endpoint/fs/mkdir.rs` 为例；`#[query]`/body 字段/into_stream 按端点实际取舍。

```rust
//! fs 端点：新建文件夹。
//!
//! 对应 `POST /api/fs/mkdir`；响应 `data: null`，以 `()` 作为端点模型。
//! （模块文档：一句话说明 + 对应 API 路径 + 数据来源；文件头不要写 pub use 之外的逻辑）

use alist_client_derive::EndpointRequest;

// model 引用 schema 类型时再导入（如 use crate::schema::fs::MkdirResp;）；
// 端点代码引用 crate::schema::<域> 无需再加 cfg（端点 feature 已镜像拉取 <域>-schema）。
// 本例响应 data: null，model = ()，无 schema 导入。

/// 新建文件夹请求构建器。
///
/// 通过 [`Fs::mkdir`](super::Fs::mkdir) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/mkdir", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标目录路径（必选）。
    path: String,
    /// 目录密码（可选）。
    password: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    ///
    /// 必选参数也可用 `impl Into<String>` 提升易用性。
    #[inline]
    #[must_use]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            password: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 新建文件夹。
    ///
    /// （中文 rustdoc：一句话说明；API 事实来源；模型说明。小节要求见下文「中文 rustdoc 模板」。）
    #[inline]
    #[must_use]
    pub fn mkdir(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path) // 父句柄私有字段 client，子模块可直接读
    }
}
```

要点核对清单（写完逐条自查）：

- `Request::new` 为 `pub(crate)`，只收 `client` 与必选参数；可选参数一律 `Option<T>` 字段 + 派生 setter，并在字段上写 `///` 文档。
- `Request` 结构体与返回构建器的句柄方法均 `#[must_use]`（结构体带理由文案，方法用裸 `#[must_use]`）。
- 访问器定义在 `impl<'a> super::<父句柄><'a>` 块内（子句柄则 `super::<子句柄>`），直接读 `self.client`，**不要**新造 `client()` 访问器。
- `build_request` 由宏生成，**不要手写**；HTTP 方法/路径必须与 openapi 或 router.go 一致。
- 模型选择：响应 `data: null` → `model = ()`；可能为 null → `model = Option<T>`；分页列表 → `crate::schema::common::PageResp<T>`；上传响应 → `crate::schema::common::UploadResp`。
- 端点只管「构建 + 发送 + 解码」，**不要**在端点层检查状态码（401/403 自动重登重试一次已在 `Client::execute` 内实现，`src/client.rs:266-295`）。

## 中文 rustdoc 模板

每个句柄访问器方法的文档必须中文且含以下小节（`# Examples` 用 ```no_run 并保证 `cargo test --all-features` 的 doctest 通过；隐藏包装写法照抄）：

```rust
/// 新建文件夹。
///
/// 对应 AList `POST /api/fs/mkdir`；成功时响应 `data` 为 `null`。
/// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `fs/mkdir` 与
/// `examples/alist/server/handles/fsread.go`（实现为 `fs.Mkdir`）。
///
/// # Arguments
///
/// * `path` - 目标目录路径（相对于某存储的完整路径）。
/// * `password` - 可选：目录密码。
///
/// # Returns
///
/// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
///
/// # Errors
///
/// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，返回 [`crate::Error`]。
///
/// # Examples
///
/// ```no_run
/// use alist_client::{Authentication, Client};
///
/// # async fn example() -> alist_client::Result<()> {
/// let client = Client::new("https://alist.example.com")?
///     .with_authentication(Authentication::token("TOKEN".to_owned()));
/// client.fs().mkdir("/new-dir")
///     .password("secret".to_owned()) // 可选参数链式 setter；String 字段可直接传 &str
///     .await?;
/// # Ok(())
/// # }
/// ```
```

## 特例一：`/ping`（纯文本端点）

`GET /ping` 返回纯文本 `pong`，**不走 JSON 信封**，不能用派生宏的 `send()`。写法：仍用派生宏声明 method/path（获得 `build_request`），发送走 `Client::execute_text`：

```rust
impl<'a> Request<'a> {
    /// 发送 ping 并返回原始响应文本。
    pub async fn send_text(&self) -> crate::Result<String> {
        self.client.execute_text(self.build_request()).await
    }
}
```

注意：派生宏仍会生成 `send`/`send_raw`/`IntoFuture`，对 `/ping` 而言语义不对；在 rustdoc 中明确「本端点请使用 `send_text`」。参考 `src/endpoint/public/ping.rs` 空壳中的说明。

## 特例二：上传端点（`PUT /api/fs/put` 流式、`PUT /api/fs/form` 表单）

两者均为 **PUT**（router.go:239-240）。请求信息全部在 HTTP 头与原始 body 中，**没有任何 JSON body 字段**——载荷字段必须全部标 `#[endpoint(skip)]`（派生 setter 不会为 skip 字段生成，需手写消费式链式 setter，风格与派生 setter 一致：`pub fn xxx(mut self, xxx: T) -> Self`）。请求头语义以 `examples/alist/server/handles/fsup.go` 为准：

| 头 | 语义（fsup.go 行号） |
|---|---|
| `File-Path` | 目标路径；服务端会 `url.PathUnescape`（fsup.go:31-35），客户端需 URL 编码（可用 `urlencoding::encode`，该 crate 已是依赖） |
| `As-Task` | 仅 `"true"` 时转后台上传任务（fsup.go:37） |
| `Overwrite` | 仅 `"false"` 时禁止覆盖；缺省允许覆盖（fsup.go:38） |
| `Last-Modified` | 文件修改时间，**epoch 毫秒整数**（fsup.go:21-27） |
| `X-File-Md5` / `X-File-Sha1` / `X-File-Sha256` | 可选哈希校验（fsup.go:65-73） |
| `Content-Type` | 流式上传中兼作 mimetype（fsup.go:74）；表单上传的 mimetype 取自文件 part（fsup.go:165） |
| `Content-Length` | 服务端读 body 大小（fsup.go:54）；`body(Vec<u8>)` 时 reqwest 自动携带 |

发送写法（设计文档 §3.2 明确：对 `build_request()` 的返回值继续加工）：

```rust
impl<'a> Request<'a> {
    /// 附加请求体与上传头并发送。
    ///
    /// # Errors
    ///
    /// 语义同 [`Client::execute`](crate::Client::execute)。注意：multipart/流式请求体
    /// 无法克隆，携带该类请求体时 401/403 自动重试不可用，将直接返回原始错误
    /// （见 `docs/design.md` §8），请在文档中说明。
    pub async fn send_upload(self) -> crate::Result<Option<crate::schema::common::UploadResp>> {
        // 头值取自结构体字段（此处示意两个）：
        // File-Path 必须 URL 编码后传字符串（urlencoding::encode(...).into_owned()，
        // 不要把 Cow 直接塞给 .header()——它不能 TryInto<HeaderValue>）
        let builder = self
            .build_request()                       // PUT /api/fs/put（或 /api/fs/form）
            .header("File-Path", urlencoding::encode(&self.path).into_owned())
            .header("Overwrite", if self.overwrite { "true" } else { "false" })
            .body(self.content);                   // 流式；form 用 .multipart(...)，字段名必须为 "file"（fsup.go:143 c.FormFile("file")）
        self.client.execute(builder).await
    }
}
```

模型：直传成功（未启用 `As-Task`）时 `data` 为 `null`（fsup.go:104-109），因此端点模型写 `model = Option<UploadResp>`；转后台任务时解出 `UploadResp { task: TaskInfo }`（共享类型在 `src/schema/common.rs:120-124`）。

**已知陷阱**：派生宏对每个 Request 一律生成 `send`/`send_raw`/`IntoFuture`（endpoint_request.rs:517-540、577-602），它们不带 body——上传端点直接 `.await` 会发出无 body 的 PUT。因此上传端点的发送方法必须用**其他名字**（如 `send_upload`），并在 rustdoc 中显式警告不要 `.await`。若你实现了更好的折衷（例如完全手写不用派生宏），在评审 notes 中说明偏差即可，但不得让 `.await` 语义错误暴露给用户。

## 请求形状测试模板

每个端点文件在 `#[cfg(test)] mod tests` 中**至少一个 URL/方法断言测试**（不必经过网络，`build_request().build()` 即可）；有 body/query 的补序列化断言；走完整收发路径的用 `crate::test_support::spawn_mock_server`（手搓 TcpListener，逐请求返回预设响应、可记录请求原文，见 `src/test_support.rs:34-74`）。以下三种模板照抄改参数：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：build_request().build() 检查 method 与 URL（含查询串）。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/dir")
            .password("secret".to_owned())
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/mkdir"), "URL 应包含路径: {url}");
        // 查询参数：有 #[query] 字段时断言出现；None 时断言被跳过
        // assert!(url.contains("page=2"), ...); assert!(!url.contains("page="), ...);
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文。
    #[tokio::test]
    async fn send_posts_expected_request() {
        use std::sync::{Arc, Mutex};
        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "/dir").send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("POST /api/fs/mkdir "), "{}", recorded[0]);
        assert!(recorded[0].contains("\"path\":\"/dir\""), "{}", recorded[0]);
    }

    /// 3) into-stream 翻页断言（仅列表端点，需 #[cfg(feature = "into-stream")]）。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use futures::StreamExt;
        use std::sync::{Arc, Mutex};
        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![
                ok_json(r#"{"code":200,"message":"success","data":{"content":[{"id":1}],"total":1}}"#),
                ok_json(r#"{"code":200,"message":"success","data":{"content":[],"total":1}}"#),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let items: Vec<u64> = Request::new(&client)
            .into_stream()
            .map(|item| item.unwrap().id)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(items, vec![1]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 2); // 空页会多一次确认请求
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
    }
}
```

完整的对照样例在 `src/endpoint.rs` 的 `mod tests`（URL/body/认证头/`data: null`/翻页断言均有）与 `src/client.rs` 的 `mod tests`。

## 过渡标记清理（实现后必须做）

- 句柄的 `client` 字段上现在压着 `#[expect(dead_code)]`（旁边有一行中文注释），例如 `src/endpoint/auth.rs:33-36`、`src/endpoint/fs/archive.rs:15-18`、`src/endpoint/fs/upload.rs:15-18`、`src/endpoint/public.rs:16-19`、`src/endpoint/admin/*.rs` 各叶子句柄。
- 当你实现该句柄的**第一个**端点文件、其访问器开始读取 `self.client` 后，这个 expect 变为「未满足」并报警告——此时**必须删除**该 `#[expect(dead_code)]` 及其上方注释行（整组一起清理，不要只删一半）。`Fs` 本体（`src/endpoint/fs.rs:31-33`）因子句柄访问器已读字段，没有 expect，不要添加。
- `src/client.rs:20` 顶部的 `#![cfg_attr(not(test), allow(dead_code))]` 由收尾代理在全部端点落地后移除，**不要动**。

## 自验命令清单（全部必须通过）

以下每条命令已在本仓库当前基线（骨架 + 已完成模块）逐条实测通过；实现端点后必须保持全绿。

```bash
cargo build --all-features
cargo test --all-features                      # 含 doctest；实测 30 单测 + 4 doctest 全绿
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --check                     # 实测 clean；禁止裸跑 cargo fmt
cargo check --no-default-features              # 实测 Finished；feature 门控完整性
# 按域验证（实现某组端点后至少跑对应组合）
cargo test --no-default-features --features <你的组>          # 如 fs / auth / public
cargo test --no-default-features --features admin            # admin 聚合
cargo test --no-default-features --features into-stream,<你的组>  # 有翻页端点时
cargo test -p alist-client-derive                             # 宏自身单元测试（11 个，改了宏才需要）
```

格式化只针对自己的文件：`cargo +nightly fmt -- src/endpoint/<组>/<端点>.rs src/schema/<域>.rs`。改完提交前自查：`git status` 中不得出现你所有权范围之外的文件名改动（只看不提交）。
