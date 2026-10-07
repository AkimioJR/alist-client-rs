---
name: alist-review-checklist
description: "评审 alist-client crate 的端点与 schema 实现（src/endpoint/<组>/*.rs、src/schema/<域>.rs）。提供 API 保真度核对方法（对照 docs/api 与 examples/alist Go 源码的精确文件路径与行号）、构建器约定检查、rustdoc 质量标准、测试覆盖要求、feature 门控验证、必须运行的确切命令，以及 approved/issues/notes 三段式评审输出格式。作为端点/schema 工作的评审代理或自查清单使用。"
---

# AList 端点/Schema 评审清单

## Goal

对并行代理产出的 `src/endpoint/<组>/<端点>.rs` 与 `src/schema/<域>.rs` 做逐条核对评审。评审者**只读代码、只跑命令**：不修代码、不补测试、不提交。

## 硬性禁令（对评审者同样生效）

1. **禁止执行任何 git 提交/暂存/推送**（提交由工作流统一完成）。`git diff` / `git status` / `git log` / `git show` 等只读命令允许。
2. **禁止裸跑 `cargo fmt`**——会格式化整个 crate，破坏并行工作区。检测格式只允许 `cargo +nightly fmt --check`；发现格式问题记入 issues，由实现者用 `cargo +nightly fmt -- <文件>` 自行修复。
3. **禁止修改任何源文件**（包括顺手修小问题）。评审结论全部写入输出。

## 评审输入

- 被审文件：该代理所有权内的 `src/endpoint/<组>/*.rs` 与 `src/schema/<域>.rs`，以及按规则允许的过渡标记清理（见第 4 条）。
- 全局改动面：`git status --porcelain` 与 `git diff --stat` 中不得出现所有权之外的文件（禁改清单：`examples/`、`docs/api/`、根 `Cargo.toml`、`src/client.rs`、`src/error.rs`、`src/endpoint.rs`、`src/endpoint/<组>.rs` 句柄结构、`src/schema.rs`、`src/schema/common.rs`、`alist-client-derive/`）。例外：该组句柄文件上的 `#[expect(dead_code)]` 联动删除是**允许且必须**的（见第 4 条）。

## 1. API 保真度核对

逐端点核对「方法 + 路径 + 模型」，核对顺序与精确位置：

1. **主来源** `docs/api/alistv3.openapi.yaml`：`grep -n '^  /' docs/api/alistv3.openapi.yaml` 得到全部路径行号，逐条比对该组端点的 `path = "..."` 与 `method = ...`。
2. **可读版** `docs/api/alistv3.md`：按 `# auth` / `# fs` / `# public` / `# admin/<子域>` 分组，`## GET/POST/PUT <描述>` 一节对应一个操作（文件中文为 GBK 乱码，只看方法/路径/JSON 示例）。
3. **仲裁来源** `examples/alist/server/router.go`（文档不确定时以此为准）：
   - `g.Any(...)` 读端点按 GET、写端点按 POST（router.go:223-228）；
   - fs 全组：router.go:222-250（注意 `/api/fs/put` 与 `/api/fs/form` 均为 **PUT**，router.go:239-240）；
   - auth 组：router.go:72-85；public 组：router.go:102-109；
   - admin 组：router.go:130-220；
   - **不在 openapi 中的三组**，逐条路由核对：
     - `/api/admin/role`：GET `list`/`get`，POST `create`/`update`/`delete`（router.go:149-154）；
     - label 的 GET `list`/`get` 在 `/api/label`（router.go:119、256-259），POST `create`/`update`/`delete` 在 `/api/admin/label`（router.go:204-207）；
     - label_file_binding 的 GET `get`/`get_file_by_label` 在 `/api/label_file_binding`（router.go:120、261-264），GET `list` 与 POST `create`/`create_batch`/`delete`/`restore` 在 `/api/admin/label_file_binding`（router.go:209-214）；
   - admin/task：路由在 router.go:191 + `examples/alist/server/handles/task.go:123-226`（GET `undone`/`done`，POST `info`/`cancel`/`delete`/`retry`/`clear_done`/`clear_succeeded` 等），且按任务类别分子路径（task.go:218-226）——核对端点路径是否正确处理了类别维度。
4. **模型保真度**（schema）：
   - 响应/请求字段对照 openapi 示例 JSON 与 Go 结构体（`examples/alist/internal/model/`、`server/handles/fsread.go`、`fsup.go`、`task.go`、`internal/model/label.go`）；冲突以 Go 为准；
   - 已知坑逐条检查：`/api/me` 的 `role` 是数组且需兼容老服务器单值（untagged）；`TaskInfo.progress` 是 `f64`；`TaskInfo` 必须含 `creator`/`creator_role`/`start_time`/`end_time`/`total_bytes`；`time.Time` → `chrono::DateTime<Utc>`、`*time.Time` → `Option<_>`；
   - `data: null` 端点的 model 必须是 `()` 或 `Option<T>`；可空响应必须 `Option<T>`；分页必须复用 `crate::schema::common::PageResp<T>`（不得重复定义分页壳）；
   - 上传端点（若有）：头语义对照 `examples/alist/server/handles/fsup.go`（`File-Path` 需 URL 编码（fsup.go:31-35 服务端 unescape）、`As-Task` 仅 `"true"` 生效（fsup.go:37）、`Overwrite` 仅 `"false"` 禁止（fsup.go:38）、`Last-Modified` 为 epoch 毫秒（fsup.go:21-27）、哈希头 `X-File-Md5/X-File-Sha1/X-File-Sha256`（fsup.go:65-73）、multipart 字段名必须为 `file`（fsup.go:143））。

## 2. 构建器约定检查

逐文件核对（来源 `docs/design.md` §3、§4 与 `src/endpoint.rs` 冒烟测试）：

- [ ] 一个文件恰好一个 `pub struct Request`（派生宏生成同名 `QueryParams`/`RequestBody`，多 Request 会冲突）。
- [ ] `client: &'a crate::Client` 字段存在且标 `#[endpoint(skip)]`（宏会强制，但人要确认没有用别的字段名绕过）。
- [ ] `Request::new` 为 `pub(crate)`、`#[must_use]`，**只接收 client 与必选参数**；必选参数允许 `impl Into<String>`；可选参数一律 `Option<T>` 字段并在字段上写 `///` 文档（文档会自动带到派生 setter 上）。
- [ ] 每个 `Option` 字段都应能通过派生 setter 设置；确认没有为 skip 字段遗漏手写 setter（上传特例）。
- [ ] `build_request` 没有被手写覆盖/重定义；上传等特例是对 `build_request()` 返回值继续 `.header(...)`, `.body(...)` / `.multipart(...)` 后交 `client.execute(...)`。
- [ ] 端点文件内**没有**状态码检查、重试、认证逻辑（这些在 `Client::execute`，见 `src/client.rs:266-295`）；`/ping` 用 `execute_text`（`src/client.rs:301-311`）。
- [ ] `into_stream` 只经 `#[cfg_attr(feature = "into-stream", endpoint(into_stream = true, stream_item = ...))]` 引入；Request 有 `page: Option<i32>` 字段且通常标 `#[query]`；model 为 `PageResp<T>` 形态。
- [ ] `#[must_use]`：Request 结构体（带理由文案）与句柄访问器方法都有。
- [ ] 句柄访问器在 `impl<'a> super::<父句柄><'a>` 块内、直接读 `self.client`，没有新增 `client()` 访问器或多余状态。
- [ ] **过渡标记清理**：访问器开始读取 `self.client` 后，句柄字段上的 `#[expect(dead_code)]` 与其上方注释行必须已删除（如 `src/endpoint/auth.rs:33-36` 原样）；`src/client.rs:20` 的模块级 `#![cfg_attr(not(test), allow(dead_code))]` 应保持原样（由收尾代理移除）。删除 expect 的组必须编译无 dead_code 警告。
- [ ] 所有权之外的过渡标记**未被误删**（其他组还没实现，expect 必须还在）。

## 3. rustdoc 质量标准

- [ ] 文件头 `//!`：中文模块文档，说明对应 API 路径与响应形状；schema 文件头注明数据来源（openapi 分组 + Go 文件）。
- [ ] 全部公开项中文文档；句柄访问器含 `# Arguments` / `# Returns` / `# Errors` / `# Examples` 四节。
- [ ] `# Examples` 用 ```no_run + `# async fn example() -> alist_client::Result<()>` 隐藏包装；示例里的链式调用与实际签名一致（setter 参数类型、await 方式）。
- [ ] setter 文档由字段 `///` 携带——确认字段文档质量（不是复述字段名，而是说明业务含义与取值）。
- [ ] 特例端点（`/ping`、上传）的 rustdoc 明确说明了与默认 `send`/`IntoFuture` 的差异：`/ping` 用 `send_text`；上传的发送方法用别名并警告**不要直接 `.await`**（派生 IntoFuture 不带 body），且说明流式 body 无法 401 重试（`docs/design.md` §8）。
- [ ] `cargo test --all-features` 的 doctest 必须全绿（no_run 也要编译通过）。

## 4. 测试覆盖要求

- [ ] 每个端点文件**至少一个** URL/方法断言测试（`build_request().build()` 断言 method 与 URL 含路径/查询串），参照 `src/endpoint.rs:126-155`。
- [ ] 有 body 或 `#[query]` 字段的端点：补序列化断言（请求体 JSON 键、None 查询参数被跳过）。
- [ ] 列表类端点且启用 `into-stream`：补翻页测试（`#[cfg(feature = "into-stream")]`，断言逐页 `page=N` 与空页停止的确认请求），参照 `src/endpoint.rs:242-275`。
- [ ] 走完整收发路径的测试使用 `crate::test_support::spawn_mock_server`（`src/test_support.rs:34-74`），不得引入新的 mock 依赖或真实网络。
- [ ] schema 文件至少三类钉扎测试：openapi/Go 示例正向反序列化、缺失/`null` 字段兼容、`serde_json::to_value` 键名钉扎；示例 JSON 必须标注来源。
- [ ] 测试与被测代码在同一文件 `#[cfg(test)] mod tests`；没有跨文件共享测试辅助（除 `test_support`）。

## 5. Feature 门控验证

- [ ] 端点文件内部无多余 `#[cfg]`（模块级门控已在 `src/endpoint.rs` / `src/endpoint/admin.rs` / `src/schema.rs` 完成）；唯一允许的 cfg 是 into-stream 相关（`cfg_attr` 与测试的 `#[cfg(feature = "into-stream")]`）。
- [ ] 端点引用 `crate::schema::<域>` 类型时不加 cfg（`X` feature 自动拉 `X-schema`，Cargo.toml:40-51）。
- [ ] 单独启用子 feature 不产生编译错误（admin 子 feature 可脱离 `admin` 启用）。

## 6. 必须运行的确切命令（评审结论以此为准，不得用构建代替测试）

```bash
git status --porcelain                         # 改动面核对（只读）
cargo build --all-features
cargo test --all-features                      # 含 doctest；基线：30 单测 + 4 doctest 全绿（评审者须复跑确认）
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --check                     # 禁止裸跑 cargo fmt
cargo check --no-default-features
# 按被审组逐个运行（替换 <组>；admin 各子域用 admin 聚合即可）
cargo test --no-default-features --features <组>
cargo test --no-default-features --features admin
cargo test --no-default-features --features into-stream,admin
cargo test -p alist-client-derive              # 基线 11 个单测；仅当本次改动涉及宏 crate 才需要复跑
```

每条命令记录「运行了 / 通过与否」；任何一条失败都直接构成 issue。评审者实际执行了哪些命令、哪些因环境未能执行，必须在输出中如实说明。

## 7. 评审输出格式

输出三段式，全部中文，issue 必须给出文件与行号：

```
## approved
<本次改动满足上述全部清单的结论；逐条列出已运行的命令与结果>

## issues
<必须修复才能合入的问题，每条：`路径:行号` + 问题 + 依据（openapi/router.go/design.md 的具体出处）。无则写「无」>

## notes
<不阻塞的观察：与既定模板的偏差及理由、建议的后续改进、需要收尾代理或架构层跟进的事项（如 common.rs 需求、任务类别维度待定）。无则写「无」>
```

判定规则：issues 非空 → 不通过；issues 为空且六条命令全绿 → approved；仅 notes → approved 并附带说明。
