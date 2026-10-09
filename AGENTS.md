# Repository Guidelines

本指南适用于人类贡献者与 AI 编码代理（Agents），旨在统一 `alist-client-rs` 仓库的工程标准与开发工作流。

---

## 1. 项目结构与模块划分

工作区（Workspace）采用「客户端主包 + 过程宏子包」布局：

- **`alist-client-derive/`**：过程宏包，提供 `#[derive(EndpointRequest)]`，自动生成请求构建、`send()`、`IntoFuture` 与 setter。
- **`src/client/` 与 `src/client.rs`**：客户端核心，处理 HTTP 连接、Token/密码凭据、401 自动重试、请求限速及错误映射。
- **`src/endpoint/`**：业务端点句柄与请求构建器，按领域子模块组织：
  - `auth`（认证）、`fs`（文件系统与上传/归档）、`public`（公开端点）、`task`（通用任务管理）、`admin`（管理后台）。
  - 每个端点操作对应独立文件（如 `fs/mkdir.rs`），内含专职的 `Request<'a>` 构建器。
- **`src/schema/`**：强类型数据模型，与服务端 API 响应及实体严格对齐。
- **`docs/`**：`docs/design.md`（架构设计约定）、`docs/api/`（OpenAPI 规范文档，*仅供辅助参考*）。
- **`examples/`**：上游 Go 源码副本（`alist`、`OpenList`，*唯一权威事实来源*，在 `.gitignore` 中忽略）。
- **`scripts/`**：工程自动化脚本（如 `update-examples.py`、`update-openapi.py`）。

---

## 2. API 事实来源与参考优先级（Source of Truth）

⚠️ **核心原则：以 Go 源码为准，文档仅供参考**。

- **唯一的权威事实来源（Ground Truth）**：
  端点的真实 HTTP 路径、请求方法（GET/POST/PUT 等）、参数传递方式（Query 还是 JSON Body）、字段命名及数据类型、状态码与错误处理逻辑，**必须 100% 以 `examples/alist`（及 `examples/OpenList`）的 Go 源码为准**（路由定位见 `server/router.go`，处理器实现见 `server/handles/`，数据结构见 `internal/model/`）。
- **`docs/api/` 文档存在滞后与疏漏**：
  由于上游社区更新不及时，`docs/api/` 下的文档（包括 OpenAPI YAML 与 Markdown）经常存在疏漏，例如：
  - 路由未同步迁移（如任务端点已迁移至 `/api/task`，但文档仍停留在 `/api/admin/task`）；
  - 新增字段未录入（如 `virtual_path`、`index`、`header` 等字段在文档中缺失）；
  - 整个子模块未收录（如角色管理、标签文件绑定等端点在 OpenAPI 中完全未记载）。
- **决策底线**：当 `docs/api/` 文档与 Go 源码发生冲突或文档缺失时，**严禁盲信文档，最终一律以 Go 源码实现为准**。

---

## 3. 构建、测试与开发命令

所有开发必须保证以下关键命令全绿通过：

```bash
# 全量构建与单测（含 doctests）
cargo build --all-features
cargo test --all-features

# 严格 Clippy 静态检查
cargo clippy --all-features --all-targets -- -D warnings

# 格式规范检测（禁止全局裸跑 cargo fmt）
cargo +nightly fmt --check

# 特性门控完整性检查（验证独立 feature）
cargo check --no-default-features
cargo test --no-default-features --features fs
cargo test --no-default-features --features task
cargo test --no-default-features --features admin

# 文档生成与死链检查
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

---

## 4. 编码风格与 rustdoc 规范

### 3.1 格式化纪律
- **禁止全局裸跑 `cargo fmt`**（防止波及并行工作区的文件）。
- 格式化时必须定向传入修改的文件：`cargo +nightly fmt -- <files...>`。

### 3.2 rustdoc 注释结构
- **字段与参数文档**：首行必须为独立的简要说明，行末统一注明 `（必选参数）。` 或 `（可选参数）。`；首行与详细说明之间**必须保留一个 `///` 空注释行**，严禁在首行使用分号（`；`）强行拼接长句。
- **杜绝死链与脆弱代码行号**：严禁在注释中出现指向本地文件树的相对路径（如 `docs/api/...`、`examples/alist/...`）以及上游 Go 代码的具体行号（如 `.go:123`），统一采用纯文本语义化表述（如 `AList OpenAPI 规范`、`AList 服务端 handles.Search`）。
- **跨版本演进字段标注**：新增或有版本行为差异的字段，须在文档中注明首次引入该特性的 Git Tag（如 `从 AList \`v3.61.0\` 起新增...; 老版本不返回该字段时为 [\`None\`]`）。

### 3.3 数据模型（Schema）设计准则
- **标量演进字段必须使用 `Option<T>`**：严格区分「老版本服务端未提供（`None`）」与「字段存在且值为有效零值（`Some(0)` / `Some(false)`）」，严禁使用 `#[serde(default)]` 直接回退为基本类型零值导致逻辑误判。
- **容器类字段的人体工程学维持**：`Vec<T>` 与 `HashMap<K, V>` 统一使用 `#[serde(default, deserialize_with = "null_to_default")]` 归约为空集合，免除调用方在迭代时的多余解包。
- **零 dead_code 豁免**：代码库处于全链路调用状态，严禁保留或引入 `allow(dead_code)`。

---

## 5. 测试指南

- **单元测试内联组织**：每个端点与模型文件均需内联 `#[cfg(test)] mod tests`。
- **双重断言覆盖**：
  1. *请求形状断言*：验证 HTTP Method、URL 路径构造、Query 串与 JSON 请求体序列化。
  2. *收发与解码断言*：使用 `crate::test_support::spawn_mock_server` 模拟预设响应并验证全流程。
- **Schema 钉扎测试**：每个模型须包含正向与老版本缺失字段的钉扎测试（缺失字段断言为 `None`，新版本断言为 `Some(...)`），键名使用 `serde_json::to_value` 钉住。

---

## 6. Git 提交规范

遵循 Conventional Commits 本地约定，格式为 `type(scope): description`：

- **常用类型**：`feat`（新特性/新 API）、`perfect`（现有能力完善与优化）、`fix`（修复 Bug）、`refactor`（重构）、`docs`（文档）、`style`（格式）、`test`（测试）、`chore`（构建/脚本/维护）。
- **常用作用域**：`fs`、`task`、`admin`、`auth`、`public`、`client`、`schema`、`scripts`。
- **要求**：首行简明扼要（英文，推荐 50 字符以内，句末不加句号）。
- **示例**：
  - `feat(task): support all categories via generic builders`
  - `refactor(fs): use Option for evolving fields with git tag provenance`
  - `chore(scripts): add script to update openapi specs`
