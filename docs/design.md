# alist-client 架构与约定（设计文档）

> 本文档是后续所有代理/贡献者实现端点与 schema 的**约定来源**。动手前请先通读。
> 架构参考 `/Users/akimio/GitRepository/MediaArk/themoviedb-rs`（句柄 + Request 构建器 + derive 宏模式）；
> API 事实来源为 `docs/api/alistv3.openapi.yaml`、`docs/api/alistv3.md`，**文档不确定时以 `examples/alist` 的 Go 源码为准**
> （路由 `server/router.go`、信封 `server/common/resp.go`、上传 `server/handles/fsup.go`、fs 读取 `server/handles/fsread.go`、
> 任务 `server/handles/task.go`、标签 `internal/model/label.go`）。

## 1. 仓库布局与所有权边界

```
Cargo.toml                  # [workspace] + 主包；feature 矩阵见文件内注释（已冻结，勿改结构）
alist-client-derive/        # proc-macro crate：EndpointRequest 派生宏（已实现，含单元测试）
src/lib.rs                  # crate 文档 + 模块接线 + re-exports（Client/Error/Result/Authentication…）
src/client.rs               # Client 核心（已完成；过渡期模块级 cfg_attr(not(test), allow(dead_code))，端点落地后移除）
src/error.rs                # Error/Result/ApiStatusCode/InternalErrorKind（已完成）
src/endpoint.rs             # 端点模块接线 + 派生宏冒烟测试（已完成；勿删测试）
src/endpoint/<组>.rs        # 各域句柄文件（已实现，勿改结构）
src/endpoint/<组>/<端点>.rs # 端点 Request 文件（空壳，待实现 ← 主要工作面）
src/schema.rs               # schema 模块接线（已完成）
src/schema/common.rs        # 共享模型：Envelope/PageReq/PageResp/TaskInfo/UploadResp（已完成）
src/schema/<组>.rs          # 各域 schema 文件（空壳，待实现 ← 主要工作面）
src/test_support.rs         # 仅测试构建：手搓 TcpListener mock 服务器（勿用于生产代码）
docs/design.md              # 本文档
```

- **禁止修改**：`examples/`（vendored 参考源码，已加入 .gitignore）、`docs/api/`、根 `Cargo.toml` 的 `[features]` 矩阵。
- **禁止**任何 git 操作（add/commit/push 等）。
- 端点 Request 文件与 schema 文件按「组」分配：改 `endpoint/auth/*` 的代理只允许碰
  `endpoint/auth/*.rs` 与 `schema/auth.rs`（+ 本文要求联动删除的 expect/allow 标记）。

## 2. Feature 门控规则

`[features]` 矩阵（根 Cargo.toml）：

- 端点 feature：`auth`、`fs`、`public`、`admin-meta`、`admin-user`、`admin-storage`、`admin-driver`、
  `admin-setting`、`admin-task`、`admin-role`、`admin-label`、`admin-label-file-binding`；
  聚合：`admin`（拉全部 admin-*）；特殊：`stream`（tokio-util，响应流式读取）、`into-stream`（async-stream + futures，自动翻页流）。
- 每个 `X` 自动拉取镜像 `X-schema`。schema 模块只按 `X-schema` 门控；端点模块只按 `X` 门控。
- 因此：**端点代码引用 `crate::schema::<域>` 类型时无需再加 cfg**——feature 已保证 schema 模块存在。
- `admin-*` 可单独启用（`endpoint/admin.rs` 用 `any(...)` 门控，子句柄访问器随子 feature 出现）。
- reqwest 启用 `json`/`query`/`stream`/`multipart` 四个 feature：`query` 是 reqwest 0.13 中
  `RequestBuilder::query` 的门控开关（派生宏生成的查询串依赖它），其余为流式下载/表单上传所需。

编译检查命令约定（见 §7）。

## 3. EndpointRequest 派生宏（属性面一行式摘要）

`#[derive(EndpointRequest)]` + `#[endpoint(method = <M>, path = "<完整路径>", model = <类型>[, into_stream = true][, stream_item = <类型>])]`；
字段属性：`#[query]`（进 URL 查询串，Option 跳过 None）、`#[endpoint(skip)]`（排除字段；`client` 字段必须标记）。

### 3.1 容器属性

| 属性 | 必填 | 说明 |
|---|---|---|
| `method = GET/POST/PUT/DELETE/PATCH/HEAD/OPTIONS` | ✅ | 生成 `::reqwest::Method::<M>` |
| `path = "/api/fs/list"` | ✅ | 自站点根起算的**完整路径**（含 `/api` 前缀；`/ping` 直接写 `"/ping"`） |
| `model = FsListResp` | ✅ | `send()`/`IntoFuture` 的解码目标；`data: null` 的端点写 `model = ()`，可能为 null 写 `model = Option<T>` |
| `into_stream = true` | — | 生成自动翻页流方法 `into_stream()`（生成代码自带 `#[cfg(feature = "into-stream")]`） |
| `stream_item = Obj` | — | 流元素类型；**仅在 `into_stream = true` 时允许** |

多个 `#[endpoint(...)]` 属性按顺序合并（`cfg_attr` 场景必需），后出现的键覆盖先前。
**必须用 `cfg_attr` 引入 `into_stream`**（feature 关闭时属性整体消失）：

```rust
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/meta/list", model = PageResp<Meta>)]
#[cfg_attr(feature = "into-stream", endpoint(into_stream = true, stream_item = Meta))]
pub struct Request<'a> { ... }
```

### 3.2 字段分类与生成的成员

| 字段 | 分类 | 进查询串 | 进 JSON body | 生成 setter |
|---|---|---|---|---|
| `client`（必须存在，必须 `#[endpoint(skip)]`） | Skip | ✗ | ✗ | ✗ |
| `#[query]` 字段 | Query | ✅ | ✗ | 仅 Option 字段 |
| 其余字段 | Body | ✗ | ✅ | 仅 Option 字段 |

- `client` 字段类型约定为 `client: &'a crate::Client`；宏校验其存在且带 `#[endpoint(skip)]`。
- **setter 只为 `Option<T>` 字段生成**，签名 `pub fn <字段名>(mut self, <字段名>: ...) -> Self`（消费式）：
  - `T = String` 时参数为 `impl Into<String>`（可直接传 `&str`）；
  - 其余类型参数直接为 `T`（避免整型字面量在 `impl Into<T>` 下的推断失败）。
  字段上的 `/// 文档` 会自动带到 setter 上。
- 必选字段（非 Option 的 Query/Body 字段）**不生成 setter**，只能由 `Request::new` 传入。
- 生成的文件私有结构体 `QueryParams<'a>` / `RequestBody<'a>` 持引用（无 Clone 要求）；
  同一模块不要有同名结构体（约定一个文件一个 Request，天然避免）。
- `build_request()` 为 `pub(crate)`，无认证头（认证由 `Client::execute` 发送时注入）。
  上传等需要原始请求体的端点：`let builder = req.build_request().body(bytes); client.execute(builder)`。
- 生成 `pub async fn send(&self)`（借用）与 `pub async fn send_raw<T: DeserializeOwned>(&self)`；
  以及 `IntoFuture`（`Output = crate::Result<model>`，手写 `Pin<Box<dyn Future + Send>>`，不依赖 futures）。
- `into_stream()`：`futures::stream::BoxStream<'a, crate::Result<stream_item>>`；从 `page`（缺省 1）起循环
  `page += 1`，产出模型 `.content`（`PageResp<T>` 形态）内元素，`content` 为空时停止（最后一页可能多发一次确认请求）。
  **要求**：Request 结构体存在 `page: Option<i32>` 字段；model 存在 `content: Vec<stream_item>` 字段。
- 宏限制：仅支持命名字段结构体、生命周期泛型（不支持类型/常量泛型参数）。

## 4. 端点 Request 文件模板（`src/endpoint/<组>/<端点>.rs`）

每个文件一个端点、一个 `Request`。完整模板（以 `fs/mkdir.rs` 为例，`#[query]`/body/into_stream 按需取舍）：

```rust
//! fs 端点：新建文件夹。
//!
//! 对应 `POST /api/fs/mkdir`；响应 `data: null`，以 `()` 作为端点模型。

use alist_client_derive::EndpointRequest;

use crate::Client;

/// 新建文件夹请求构建器
///
/// 通过 [`Fs::mkdir`](super::Fs::mkdir)（示例）创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/mkdir", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a Client,
    /// 目标目录路径（必选）。
    path: String,
    /// 目录密码（可选）。
    password: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走 setter。
    #[inline]
    #[must_use]
    pub(crate) fn new(client: &'a Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            password: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 新建文件夹
    ///
    /// （中文 rustdoc：一句话说明 + API 事实来源注释）
    ///
    /// # Arguments
    ///
    /// * `path` - 目标目录路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().mkdir("/new-dir").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn mkdir(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}
```

模板规则：

1. `Request::new(client, <必选参数…>)` 为 `pub(crate)`；**必选参数也可用 `impl Into<String>` 提升易用性**。
2. 可选参数 = `Option<T>` 字段 + 派生 setter；带 `///` 字段文档。
3. `Request` 与访问器方法均 `#[must_use]`（返回构建器的方法必须有 must_use）。
4. `build_request` 由宏生成（`pub(crate)`），**不要手写**；确需附加原始 body（上传）时在端点文件里对
   `build_request()` 的返回值继续加工。
5. 访问器定义在 `impl<'a> super::<父句柄><'a>` 块内，**直接读 `self.client`（父句柄私有字段，子模块可见）**。
6. 全部 rustdoc 中文；`# Arguments` / `# Returns` / `# Errors` / `# Examples` 小节齐全；
   `# Examples` 用 ```no_run 并保证 `cargo test --all-features` 的 doctest 通过。
7. **实现端点后清理过渡标记**：若所在句柄字段上存在 `#[expect(dead_code)]`（现在指向
   `client: &'a crate::Client` 字段，旁边有注释），端点读取该字段后该 expect 会变为「未满足」并报警告——
   此时删除该 `#[expect(dead_code)]` 及其注释行。`src/client.rs` 顶部的
   `#![cfg_attr(not(test), allow(dead_code))]` 待全部端点落地后由收尾代理移除。
8. HTTP 方法/路径必须与 `docs/api/alistv3.openapi.yaml` 或 `examples/alist/server/router.go` 一致；
   router 中 `g.Any(...)` 的读端点按 GET 处理，写端点按 POST 处理。

## 5. Schema 约定（`src/schema/<域>.rs`）

1. 所有模型 derive 至少 `Debug, Clone, PartialEq, Serialize, Deserialize`（响应模型可省 Serialize）。
2. **可选字段用 `Option<T>`** 并加 `#[serde(default)]`；集合字段额外容忍 `null` 时可用
   `#[serde(default, deserialize_with = "…null_to_default…")]`（模式见 git 历史 `src/models/auth.rs`）。
3. 服务端新增字段（如 `role_names`、`permissions`、`device_key`）必须以 `#[serde(default)]` 的 Option/Vec 接住，保证向后兼容。
4. 与 Go 源码冲突时以 Go 为准，例如：`/api/me` 的 `role` 是数组（`model.Roles []int`，老服务器可能返回单值，
   需 untagged 兼容）；`TaskInfo.progress` 是 `f64`；`time.Time` 映射 `chrono::DateTime<Utc>`（`*time.Time` → `Option<_>`）。
5. **示例 JSON 钉扎测试**：每个 schema 文件在 `#[cfg(test)] mod tests` 中用 `docs/api/alistv3.openapi.yaml`
   与 Go 源码里的真实示例 JSON 反序列化断言（参考 `src/schema/common.rs` 与 git 历史 `src/models/*` 的写法），
   键名序列化用 `serde_json::to_value` 断言钉住。
6. 复用共享模型：分页响应用 `crate::schema::common::PageResp<T>`，任务体用 `TaskInfo`，上传响应 `UploadResp`。
7. 文件头部：中文模块文档 + 数据来源说明；不写 pub use 之外的实现逻辑。

## 6. 错误与 envelope 语义（已实现，端点代理需理解）

- AList 多数错误以 **HTTP 200 + 信封 `code` 非 200** 返回；`Client::execute` 统一处理：
  HTTP 非 2xx → `Error::HttpStatus`；信封 `code` 非 200 → `Error::Api { code, kind, message, data }`；
  `data` 二次反序列化失败 → `Error::Json`（附 method/url/响应体上下文）。
- `Error::Api.code` 为 [`ApiStatusCode`]（0/200=Ok、202=Accepted、400/401/402/403/404/405/429/500、其余 Unknown）；
  `kind` 为 `InternalErrorKind::from_message` 按 `alist/internal/errs` 常量文本的尽力分类。
- **`data: null` 直接用 `model = ()` 或 `model = Option<T>` 解码**，不要在端点里特殊处理。
- 401/403（信封或 HTTP）且配置了 `Authentication::UsernamePassword` 时自动重新登录并重试一次；
  `Authentication::Token` 不触发。登录实现内置于 `client.rs`（`/api/auth/login`），不依赖任何端点 feature。
- 端点只管「构建 + 发送 + 解码」，**不要**在端点层重复检查状态码。

## 7. 测试与检查命令约定

```bash
# 自验（全部必须通过）
cargo build --all-features
cargo test --all-features                      # 含 doctest 与派生宏冒烟测试
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --check                     # 检测格式（勿裸跑 `cargo fmt`）
cargo check --no-default-features              # feature 门控完整性
# 按域验证（实现某组端点后至少跑对应组合）
cargo test --no-default-features --features fs
cargo test --no-default-features --features auth
cargo test --no-default-features --features public
cargo test --no-default-features --features admin        # admin 聚合
cargo test --no-default-features --features into-stream,admin
cargo test -p alist-client-derive                         # 宏自身单元测试
```

- **格式化纪律**：`cargo +nightly fmt --check` 只用于检测；需要格式化时只格式化自己负责的文件：
  `cargo +nightly fmt -- <file>...`。**禁止裸跑 `cargo fmt`**（会波及他人正在修改的文件）。
- 测试基础设施：`crate::test_support::spawn_mock_server`（手搓 tokio TcpListener，逐请求返回预设响应、
  可记录请求原文）。端点测试写法参考 `src/endpoint.rs` 的 `mod tests`（含 URL 构建断言、body/查询串断言、
  认证头断言、`data: null`、`into_stream` 翻页等）。客户端行为（信封/刷新/限速/错误分类）测试在
  `src/client.rs` 内联 `mod tests`。
- 每个端点文件建议至少一个 URL/方法断言测试；有 body 或 query 的端点补序列化断言；
  列表类端点如启用 `into-stream`，补翻页测试。

## 8. Client 公共面速查（已实现）

- `Client::new(base_url) -> Result<Client>`：基址末尾 `/` 归一化；路径相对拼接（支持反代路径前缀）。
- `set_authentication / with_authentication / clear_authentication`（`Authentication::username_password(usr, pwd, otp)` / `Authentication::token(t)`）。
- `set_api_request_interval / with_api_request_interval / api_request_interval`：客户端侧限速（串行保持间隔）。
- `base_url()`；token 与凭据为内部状态（`token()`/`authentication()` 为 `pub(crate)`）。
- `pub(crate) request(method, path)`：构建请求（无认证头）；`pub(crate) execute<R>(builder)`：信封解码 + 限速 + 401/403 自动重登重试一次；
  `pub(crate) execute_text(builder)`：原始文本响应（供 `/ping`），无信封解码。
- 流式 body（multipart 上传）无法克隆，401 重试对这类请求不可用（返回原始错误）——上传端点实现时注意在文档说明。
