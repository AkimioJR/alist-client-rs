# alist-client 架构与约定（设计文档）

> 本文档是后续所有代理/贡献者实现端点与 schema 的**约定来源**。动手前请先通读。
> 架构参考 `/Users/akimio/GitRepository/MediaArk/themoviedb-rs`（句柄 + Request 构建器 + derive 宏模式）；
> API 事实来源为 `docs/api/alistv3.openapi.yaml`、`docs/api/alistv3.md`，**文档不确定时以 `examples/alist` 的 Go 源码为准**
> （路由 `server/router.go`、响应包装 `server/common/resp.go`、上传 `server/handles/fsup.go`、fs 读取 `server/handles/fsread.go`、
> 任务 `server/handles/task.go`、标签 `internal/model/label.go`）。

## 1. 仓库布局与所有权边界

```
Cargo.toml                  # [workspace] + 主包；feature 矩阵见文件内注释（已冻结，勿改结构）
alist-client-derive/        # proc-macro crate：EndpointRequest 派生宏（已实现，含单元测试）
src/lib.rs                  # crate 文档 + 模块接线 + re-exports（Client/Error/Result/Authentication…）
src/client.rs               # Client 核心（已完成；凭据流转/请求执行/响应解码/自动重登）
src/error.rs                # Error/Result/ApiStatusCode/InternalErrorKind（已完成）
src/endpoint.rs             # 端点模块接线 + 派生宏冒烟测试（已完成；勿删测试）
src/endpoint/<组>.rs        # 各域句柄文件（已实现，勿改结构）
src/endpoint/<组>/<端点>.rs # 端点 Request 文件（空壳，待实现 ← 主要工作面）
src/schema.rs               # schema 模块接线（已完成）
src/schema/common.rs        # 共享模型：Response/PageRequest/PageResponse/TaskInfo/UploadResponse（已完成）
src/schema/<组>.rs          # 各域 schema 文件（空壳，待实现 ← 主要工作面）
# 单元测试 mock：采用 dev-dependencies 下的 mockito 异步服务器模拟 HTTP 响应
docs/design.md              # 本文档
```

- **禁止修改**：`examples/`（vendored 参考源码，已加入 .gitignore）、`docs/api/`、根 `Cargo.toml` 的 `[features]` 矩阵。
- **禁止**任何 git 操作（add/commit/push 等）。
- 端点 Request 文件与 schema 文件按「组」分配：改 `endpoint/auth/*` 的代理只允许碰
  `endpoint/auth/*.rs` 与 `schema/auth.rs`（+ 本文要求联动删除的 expect/allow 标记）。

## 2. Feature 门控规则

`[features]` 矩阵（根 Cargo.toml）：

- 端点 feature：`auth`、`fs`、`public`、`task`、`admin-meta`、`admin-user`、`admin-storage`、`admin-driver`、
  `admin-setting`、`admin-role`、`admin-label`、`admin-label-file-binding`；
  聚合：`admin`（拉全部 admin-*）；特殊：`stream`（tokio-util，响应流式读取）、`into-stream`（async-stream + futures，自动翻页流）。
- 每个 `X` 自动拉取镜像 `X-schema`。schema 模块只按 `X-schema` 门控；端点模块只按 `X` 门控。
- 因此：**端点代码引用 `crate::schema::<域>` 类型时无需再加 cfg**——feature 已保证 schema 模块存在。
- **登录端点常驻编译例外**：为支持 `Client::refresh_token` 在未启用 `auth` feature 时仍能自动重新登录，登录端点（`login` / `login_hash`）及对应模型在内部常驻编译（`#[cfg(not(feature = "auth"))] pub(crate) mod auth`，对应 schema 亦保持内部常驻）。当启用 `auth` feature 时，通过 `pub mod auth` 将这些结构体与句柄直接重导出给调用方。
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
| `model = FsListResponse` | ✅ | `send()`/`IntoFuture` 的解码目标；`data: null` 的端点写 `model = ()`，可能为 null 写 `model = Option<T>` |
| `into_stream = true` | — | 生成自动翻页流方法 `into_stream()`（生成代码自带 `#[cfg(feature = "into-stream")]`） |
| `stream_item = Obj` | — | 流元素类型；**仅在 `into_stream = true` 时允许** |

多个 `#[endpoint(...)]` 属性按顺序合并（`cfg_attr` 场景必需），后出现的键覆盖先前。
**必须用 `cfg_attr` 引入 `into_stream`**（feature 关闭时属性整体消失）：

```rust
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/admin/meta/list", model = PageResponse<Meta>)]
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
  `page += 1`，产出模型 `.content`（`PageResponse<T>` 形态）内元素，`content` 为空时停止（最后一页可能多发一次确认请求）。
  **要求**：Request 结构体存在 `page: Option<i32>` 字段；model 存在 `content: Vec<stream_item>` 字段。
- 宏限制：仅支持命名字段结构体、生命周期泛型（不支持类型/常量泛型参数）。

## 4. 端点 Request 文件模板（`src/endpoint/<组>/<端点>.rs`）

每个文件一个端点、一个 `Request`。完整模板（以 `fs/mkdir.rs` 为例，`#[query]`/body/into_stream 按需取舍）：

```rust
//! fs 端点：新建文件夹。
//!
//! 对应 `POST /api/fs/mkdir`；响应 `data: null`，以 `()` 作为端点模型。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。

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
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
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
6. 全部 rustdoc 中文；访问器 doc 的 `# Arguments` / `# Returns` / `# Errors` / `# Version` / `# Examples` 小节齐全，
   模块文档（`//!`）末尾同样带 `# Version`（**两处逐字一致**，规范见规则 9）；
   `# Examples` 用 ```no_run 并保证 `cargo test --all-features` 的 doctest 通过。
7. **无 dead_code 豁免约定**：端点与客户端核心已全部落地，dead_code 豁免已彻底清除。所有句柄字段、内部方法与端点均处于全链路调用状态，代码库严禁保留或引入 `allow(dead_code)`、`expect(dead_code)` 等临时标记。
8. HTTP 方法/路径必须与 `docs/api/alistv3.openapi.yaml` 或 `examples/alist/server/router.go` 一致；
   router 中 `g.Any(...)` 的读端点按 GET 处理，写端点按 POST 处理。
9. **端点版本小节（`# Version`）规范**：
   - **双处落点**：模块文档（`//!`）末尾与句柄访问器 doc（`# Errors` 与 `# Examples` 之间）各写一个
     `# Version` 小节，**两处内容逐字一致**（纯文本 + 反引号，不用 intra-doc 链接），
     保证阅读模块文档与调用方法两种场景都能看到版本信息；
   - **固定首个条目**：`- 自 AList \`vX.Y.Z\` 起提供本接口。`。AList v3 初期即存在的端点写 `v3.0.0`
     （v2 及更早版本不在本 crate 支持范围）；beta/rc 等预发布 tag 一律归一化到包含该提交的下一个正式版本 tag；
   - **差异条目**：其后按版本升序逐条列出**经 `examples/alist` Go 源码 git 历史证实**的接口级变更
     （路由路径变更、请求/响应字段集变更、权限或错误行为变更），句式
     `从 AList \`vX.Y.Z\` 起，<变更内容>；老版本<缺失/旧行为语义>。`。
     无证实差异则不写变更条目，**宁缺毋滥，严禁凭空推断**；
   - **考古方法**：路由引入版本用
     `git -C examples/alist log -S '<Handler 函数名 或 "/<路由>">' --format='%h %ad %s' --date=short -- server/`
     （**必须用目录级 pathspec**，避免 controllers→handles 改名导致漏史），取最早命中提交后用
     `git -C examples/alist describe --tags --contains <sha>` 映射 tag；最早命中早于 v3 重构提交
     （`refactor: init v3`）时按 `v3.0.0` 处理（可用
     `git -C examples/alist merge-base --is-ancestor <sha> <v3 重构提交>` 判定谱系）；
     响应字段演进对 `internal/model/` 与 `server/` 做 `log -S` 考古；
   - **与 schema 分工**：响应字段的存在性/缺失语义（`Option<T>` 建模与字段级 tag 溯源）归 schema rustdoc（§5），
     端点 `# Version` 只写接口级事实；tag 引用须与 schema 标注及文件内既有版本句保持一致；
   - **整合既有散落版本句**：文件内已有的「新版/老版本」散文描述与 Git Tag 版本句
     一并整合进 `# Version` 小节并补齐 tag，除必要的移动/去重外不重写无关文字；
   - **注释纪律**：端点版本标注只允许修改 `///` 与 `//!` 注释行，严禁触碰任何代码、属性与测试；
     禁止在注释中出现本地相对路径与上游代码行号。

## 5. Schema 约定（`src/schema/<域>.rs`）

### 5.1 数据模型（Schema）三阶命名分类规范（Type Taxonomy）

为了遵循 Rust 惯用命名习惯并消除“网络响应 DTO 泄露为业务实体”的异味，数据模型严格遵守以下三阶分类法则：

1. **核心业务实体 (Core Entity)**：
   - 具有独立持久化身份或代表系统核心业务资源；
   - **命名规则**：纯单数名词（`PascalCase`），严禁使用 `*Response`、`*Model`、`*DTO` 后缀；
   - **典型范例**：`User`、`FsObject`、`Storage`、`Session`、`Role`、`Share`、`DriverItem`、`Meta`、`Label`。
2. **实体变体 / 投影视图 (Entity Variant / View)**：
   - 实体的特定投影或扩展视图；
   - **命名规则**：`修饰词 + 核心实体名`，严禁使用 `*Response`；
   - **典型范例**：`LabeledFsObject`（带标签属性的文件对象）、`PublicShareObject`、`CurrentUser`。
3. **复合操作结果 / 上下文 (Composite Result / Context)**：
   - 包含多字段、列表及控制元信息的查询上下文产物；
   - **命名规则**：`名词 + Content / Detail / List`；
   - **典型范例**：`DirContent`（目录上下文与条目内容）、`FsDetail`（文件完整详情）、`PublicShareList`、`ArchiveMeta`、`ArchiveList`。
4. **动作回执 / 凭证 (Action Receipt / Credential)**：
   - 瞬时操作的执行结果、权限确认或非实体凭据；
   - **命名规则**：`动作/名词 + Result` 或业务专有凭证名词；
   - **典型范例**：`LoginResult`（登录凭据）、`TwoFactorSecret`（2FA 密钥与二维码）、`DownloadLink`（直链凭据）、`CopyResult`（复制任务列表）、`OfflineDownloadResult`、`DecompressResult`、`UploadResult`、`CreateLabelResult`、`CreateStorageResult`。
5. **条目 / 命中项 (Item / Hit)**：
   - 列表或搜索中的单个匹配项；
   - **命名规则**：`名词 + Entry / Item / Hit`；
   - **典型范例**：`DirEntry`（子目录项）、`SearchItem`（搜索命中项）、`ArchiveNode`（压缩包树节点）。
6. **协议传输外壳 (Wire Envelope)**：
   - 仅限底层的通用通信协议信封；
   - **命名规则**：通用技术名词 + 泛型；
   - **保留类型**：`Response<T>`、`PageResponse<T>`。

### 5.2 建模与演进准则

1. 所有模型 derive 至少 `Debug, Clone, PartialEq, Serialize, Deserialize`（响应模型可省 Serialize）。
2. **跨版本标量字段必须使用 `Option<T>`**：
   - 随 AList 版本演进新增或废弃的标量元数据、标识符或计数统计（如 `has_more`、`page`、`per_page`、`filtered_total`、`pages_total`、`virtual_path`、`disable_index`、`header`、`index` 等），**必须严格区分「字段未提供 / 老版本不支持（`None`）」与「字段存在且值为有效零值（`Some(0)` / `Some(false)` / `Some("")`）」**；
   - **严禁**使用 `#[serde(default)]` 直接回退为基本类型零值（如 `false` 或 `0`），避免产生业务误判（例如老版本缺失 `has_more` 被误归为 `false` 导致提前终止翻页）；
   - 所有演进字段必须搭配 `#[serde(default)]` 以保证老版本未返回时安全反序列化为 `None`。
3. **rustdoc 精确标明 Git Tag 版本溯源**：
   - 每一个跨版本演进字段必须在 rustdoc 中注明其首次引入或变更的 AList Git Tag（例如 `从 AList \`v3.61.0\` 起新增规范虚拟展示路径；老版本不返回该字段时为 [\`None\`]。`）；
   - 注释严格遵守首行独立简述 + 空行 + 详情段落的规范，禁止泄露本地相对路径和上游具体代码行号。
4. **集合与映射类容器字段的人体工程学维持**：
   - 集合类字段（如 `Vec<T>`、`HashMap<K, V>`）统一采用 `#[serde(default, deserialize_with = "null_to_default")]` 映射为空集合，避免调用方在日常遍历时产生不必要的多层解包开销；
   - 容器字段的 rustdoc 中依然需要标明对应的版本演进 Tag 说明。
5. **字段形状以 AList Go 源码为准**：
   - 当文档与 Go 源码冲突时以 Go 为准。例如：`/api/me` 的 `role` 为数组（Go `model.Roles []int`，老服务器可能返回单值，需 untagged 兼容）；`TaskInfo.progress` 为 `f64`；`time.Time` 映射为 `chrono::DateTime<Utc>`（`*time.Time` 映射为 `Option<_>`）。
6. **示例 JSON 钉扎测试**：
   - 每个 schema 文件在 `#[cfg(test)] mod tests` 中必须对新版本完整字段以及老版本缺失字段分别建立钉扎测试断言（老版本缺失断言为 `None`，新版本断言为 `Some(...)`），键名序列化用 `serde_json::to_value` 钉住。
7. **复用共享模型**：
   - 分页响应用 `crate::schema::common::PageResponse<T>`，任务体用 `TaskInfo`，上传回执复用 `UploadResult`。
8. **文件头部**：中文模块文档 + 数据来源说明；不写 pub use 之外的实现逻辑。

### 5.3 保留字字段命名规范（`r#type` 原生标识符）

在 AList 服务端协议中，`type`（如文件类型、驱动项类型、设置项值类型、标签类型、消息类型等）是极其高频出现的 JSON 键。由于 `type` 是 Rust 语言核心保留关键字，本 crate 严格遵循以下设计与工程纪律：

1. **统一采用 Raw Identifier（`r#type`）**：
   - 所有对应服务端 JSON 键为 `"type"` 的数据模型字段，**全库统一命名为 `pub r#type: T`**；
   - **严禁**使用领域特化别名（如 `label_type`、`value_type`、`item_type`）进行重命名，以消除多模型间命名碎片化，确保调用方心智模型与 AList 官方 API 及 Go 源码 1:1 绝对一致。
2. **免除冗余 `#[serde(rename = "type")]` 标注**：
   - Serde 宏在 AST 解析阶段内置了 `unraw()` 机制，提取字段名时会自动剥离前缀 `r#`；
   - 当服务端的 JSON 键为全小写 `"type"` 时，定义 `pub r#type: T` 会天然正确反序列化与序列化为 `"type"`，无需额外增加 `#[serde(rename = "type")]` 属性，保持模型定义干净清爽；
   - 仅当服务端 JSON 键非标准全小写（如大驼峰 `"Type"`）时才显式标注 `#[serde(rename = "Type")]`。
3. **钉扎测试必须覆盖序列化键名**：
   - 在各 schema 文件的单元测试中，必须通过 `serde_json::to_value` 钉住该字段序列化出的 JSON 键确为 `"type"`，并断言不存在 `r#type` 等泄漏键名。
4. **rustdoc 内部文档链接书写规范**：
   - 在 intra-doc links 引用结构体的 `r#type` 字段时，使用 `[`r#type`](Struct::type)`（如 `[`Setting::r#type`](Setting::type)`），以保证 rustdoc 链接解析器与 HTML 渲染能够精准命中该字段而无死链告警。

## 6. 错误与响应包装（envelope）语义（已实现，端点代理需理解）

- AList 多数错误以 **HTTP 200 + 响应 `code` 非 200** 返回；`Client::execute` 统一处理：
  HTTP 非 2xx → `Error::HttpStatus`；响应 `code` 非 200 → `Error::Api { code, kind, message, data }`；
  `data` 二次反序列化失败 → `Error::Json`（附 method/url/响应体上下文）。
- `Error::Api.code` 为 [`ApiStatusCode`]（0/200=Ok、202=Accepted、400/401/402/403/404/405/429/500、其余 Unknown）；
  `kind` 为 `InternalErrorKind::from_message` 按 `alist/internal/errs` 常量文本的尽力分类。
- **`data: null` 直接用 `model = ()` 或 `model = Option<T>` 解码**，不要在端点里特殊处理。
- 401/403（响应包装或 HTTP）且配置了 `Authentication::UsernamePassword` 时自动重新登录并重试一次；
  `Authentication::Token` 不触发。
  - **登录端点与模型复用**：登录端点（`login` / `login_hash`）及对应模型在内部常驻编译（`#[cfg(not(feature = "auth"))] pub(crate) mod auth`），由 `Client::refresh_token` 直接复用 `login::Request::new(...).build_request()` 构建请求并反序列化为 `LoginResult`；
  - **底层非认证通道与防死循环**：刷新过程走底层的非认证通道（`authenticated = false`），既避免了手写重复的请求体/响应体，又避免了递归重登死循环；
  - **按需对外暴露**：当启用 `auth` feature 时，通过 `pub mod auth` 将这些结构体与句柄直接重导出给调用方。
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
- 测试基础设施：采用 `mockito::Server::new_async().await` 模拟预设响应、
  可记录请求原文）。端点测试写法参考 `src/endpoint.rs` 的 `mod tests`（含 URL 构建断言、body/查询串断言、
  认证头断言、`data: null`、`into_stream` 翻页等）。客户端行为（响应包装/刷新/限速/错误分类）测试在
  `src/client.rs` 内联 `mod tests`。
- 每个端点文件建议至少一个 URL/方法断言测试；有 body 或 query 的端点补序列化断言；
  列表类端点如启用 `into-stream`，补翻页测试。

## 8. Client 架构与设计速查（已实现）

- `Client::new(base_url) -> Result<Client>`：基址末尾 `/` 归一化；路径相对拼接（支持反代路径前缀）。
- `set_authentication / with_authentication / clear_authentication`（`Authentication::username_password(usr, pwd, otp)` / `Authentication::token(t)`）。
- `set_api_request_interval / with_api_request_interval / api_request_interval`：客户端侧限速（串行保持间隔）。
- `base_url()`；token 与凭据为内部状态（`token()`/`authentication()` 为 `pub(crate)`）。
- `pub(crate) request(method, path)`：构建请求（无认证头）；`pub(crate) execute<R>(builder)`：响应解码 + 限速 + 401/403 自动重登重试一次；
  `pub(crate) execute_text(builder)`：原始文本响应（供 `/ping`），无响应解码。
- **底层请求通道与认证刷新设计**：
  - `send_response(builder, context, authenticated)`：内部底层发送通道。`authenticated = true` 为常规业务请求；`authenticated = false` 为非认证底层通道（不注入 Authorization 头）。
  - `Client::refresh_token`：使用保存的用户名/密码（及可选两步验证码）重新登录。为消除重复代码与维护负担，直接复用 `login::Request::new(...).build_request()` 构建请求，并反序列化为 `LoginResponse`。
  - **防递归死循环**：重新登录请求严格走底层非认证通道（`authenticated = false`），不携带已失效的旧 token，同时仅在顶层业务请求收到 401/403 时触发一次，重登自身失败即刻报错退出，避免了递归重登死循环。
  - **常驻编译与重导出**：登录端点（`login` / `login_hash`）及对应模型在内部常驻编译（`#[cfg(not(feature = "auth"))] pub(crate) mod auth`），因此无需依赖外部开启 `auth` feature；当调用方显式启用 `auth` feature 时，通过 `pub mod auth` 将这些结构体与句柄直接重导出给调用方。
- 流式 body（multipart 上传）无法克隆，401 重试对这类请求不可用（返回原始错误）——上传端点实现时注意在文档说明。
