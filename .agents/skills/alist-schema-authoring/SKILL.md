---
name: alist-schema-authoring
description: "在 alist-client crate 中实现单个域的数据模型（src/schema/<域>.rs）。涵盖 serde 约定（#[serde(default)]、rename、skip_serializing_if、null→默认值、跨版本兼容与 untagged 兼容）、从 docs/api 与 examples/alist Go 源码提取示例 JSON 做钉扎测试的完整模板、命名约定、与端点代理的复用边界、自验命令清单。实现 auth/fs/public/admin-* 任一 schema 文件之前必读。"
---

# AList Schema 实现指南

## Goal

为 `src/schema/<域>.rs` 的空壳文件补全该域的全部数据模型，并用示例 JSON 钉扎测试钉住字段形状。schema 文件是纯数据定义：中文模块文档 + 模型 + 测试，不写实现逻辑。

## 硬性禁令（违反即返工）

1. **禁止执行任何 git 操作**（add/commit/push 等）。提交由工作流统一完成。
2. **禁止裸跑 `cargo fmt`**——它会把整个 crate（包括其他并行代理正在修改的文件）全部重排。检测用 `cargo +nightly fmt --check`；确需格式化时只格式化自己的文件：`cargo +nightly fmt -- <你的文件>...`。
3. **禁止修改**：`examples/`、`docs/api/`、根 `Cargo.toml`、`src/client.rs`、`src/error.rs`、`src/endpoint.rs`、`src/endpoint/**`、`src/schema.rs`、`src/schema/common.rs`（共享模型已完成，只复用）。
4. **所有权边界**：负责 `<域>` 的代理只允许改 `src/schema/<域>.rs`；若发现必须改 `common.rs` 才能继续，把需求写进交付说明（notes），不要自行改动。

## 事实来源与核对顺序

1. `docs/api/alistv3.openapi.yaml`：以路径为索引（`grep -n '^  /' docs/api/alistv3.openapi.yaml`），每个路径下有请求/响应示例 JSON 与字段表；
2. `docs/api/alistv3.md`：按 `# auth`、`# fs`、`# public`、`# admin/<子域>`、`# 数据模型` 分组；`## GET/POST/PUT <描述>` 一节内有 `> Body 请求参数` 与 `> 返回示例` 的 JSON 代码块，**这些代码块就是钉扎测试的素材**。注意该文件中文是 GBK 乱码，不要复制中文注释，只取 JSON；
3. **与 Go 源码冲突时以 Go 为准**（`docs/design.md` §1 明确）：
   - 路由与信封：`examples/alist/server/router.go`、`server/common/resp.go`；
   - 各域请求/响应结构体：`examples/alist/internal/model/`（如 `req.go`、`user.go`、`obj.go`、`label.go`）与 `server/handles/`（fs 读取 `fsread.go`、上传 `fsup.go`、任务 `task.go`）；
   - `admin/role`、`admin/label`、`admin/label_file_binding` **不在 openapi 中**，模型只能从 Go 结构体（如 `internal/model/label.go`）与 handler 反推。

## serde 约定（逐条执行）

参考实现：`src/schema/common.rs`（信封/分页/任务）与 git 历史 `git show 9a91a15:src/models/auth.rs`（`null_to_default` 与 `deserialize_role_ids` 的原始写法）。

1. **derive 清单**：所有模型至少 `Debug, Clone, PartialEq, Serialize, Deserialize`；纯响应模型可省 `Serialize`（无法确定时都保留）。eq 需要浮点时去掉 `Eq`（参照 `TaskInfo`，`src/schema/common.rs:82`）。
2. **可选字段**：一律 `Option<T>` 并加 `#[serde(default)]`；请求体中的可选字段额外加 `#[serde(skip_serializing_if = "Option::is_none")]`（响应模型不需要）。参照 `src/schema/common.rs:88-92`（TaskInfo）与历史 `LoginReq.otp_code`。
3. **集合/标量字段容忍显式 `null`**：服务端（尤其老版本）会对新字段返回 `null`，用 `#[serde(default, deserialize_with = "null_to_default")]` 接住：

   ```rust
   /// 服务端新增字段；老版本不返回，新版本可能返回 null。
   #[serde(default, deserialize_with = "null_to_default")]
   pub role_names: Vec<String>,
   ```

   `null_to_default` 为文件私有辅助函数（一个 schema 文件放一份或集中放域文件顶部，不要动 common.rs）：

   ```rust
   fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
   where
       D: serde::Deserializer<'de>,
       T: Default + serde::Deserialize<'de>,
   {
       let opt = Option::<T>::deserialize(deserializer)?;
       Ok(opt.unwrap_or_default())
   }
   ```

4. **单值/数组双形状**：已知案例 `/api/me` 的 `role`——Go 侧 `model.Roles []int`（数组），老服务器可能返回单值。用 `#[serde(untagged)]` 兼容（历史 `src/models/auth.rs` 的 `deserialize_role_ids`）：

   ```rust
   /// 角色 ID；新服务器返回数组，老服务器可能返回单值。
   #[serde(deserialize_with = "deserialize_role_ids")]
   pub role: Vec<i32>,

   fn deserialize_role_ids<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
   where
       D: serde::Deserializer<'de>,
   {
       #[derive(serde::Deserialize)]
       #[serde(untagged)]
       enum RoleIds {
           Many(Vec<i32>),
           One(i32),
       }
       match RoleIds::deserialize(deserializer)? {
           RoleIds::Many(values) => Ok(values),
           RoleIds::One(value) => Ok(vec![value]),
       }
   }
   ```

5. **键名**：Rust 字段名用 snake_case；JSON 键与字段名一致时**不要**写 `rename`。仅当 JSON 键不是合法/惯用 Rust 标识符（如 Go 导出名与 JSON tag 不一致、键名含保留字）时才 `#[serde(rename = "...")]`，并用钉扎测试锁住。
6. **时间**：Go `time.Time` → `chrono::DateTime<chrono::Utc>`（chrono 已启用 serde feature，Cargo.toml:14）；Go `*time.Time` → `Option<DateTime<Utc>>` + `#[serde(default)]`。秒/毫秒时间戳例外：上传的 `Last-Modified` 头是 epoch 毫秒（fsup.go:21-27），若模型里出现时间戳字段，按 openapi 示例的实际形状（字符串 vs 数字）决定类型并钉住。
7. **数值与形状陷阱**（Go 源码优先，已核实）：
   - `TaskInfo.progress` 是 `f64` 不是整数（`src/schema/common.rs:99`）；
   - `TaskInfo` 含 `creator`/`creator_role`/`start_time`/`end_time`/`total_bytes`（不要照抄老文档精简版）；
   - `/api/me` 的 `role` 是数组（见第 4 条）；
   - Go `[]int`/`[]string` → `Vec<i32>`/`Vec<String>`；位掩码类（`permission`）→ `i32`；
   - 公共设置等「值为字符串化的设置项」端点（`/api/public/settings` 的 `data` 是 `map[string]string` 形态）按 openapi 示例建模（如 `HashMap<String, String>` 或具名结构 + `flatten`），以示例 JSON 为准钉扎。
8. **向后兼容原则**：服务端新增字段（如 `role_names`、`permissions`、`device_key`）必须以 `#[serde(default)]` 的 `Option`/`Vec` 接住——反序列化对未知字段默认宽容（serde 行为），但对**缺失**与**显式 null** 必须显式处理（第 2、3 条）。
9. **复用共享模型**，不要重复定义：分页响应 `crate::schema::common::PageResp<T>`、分页请求 `PageReq`、任务体 `TaskInfo`、上传响应 `UploadResp`、信封 `Envelope<T>`（`src/schema/common.rs:27-124`）。端点模型里 `model = PageResp<Xxx>` 时，本文件只需定义元素类型 `Xxx`。

## 命名与文件组织约定

- 文件头：中文模块文档 `//!`，说明本域覆盖哪些端点、字段形状的数据来源（openapi 分组 + Go 文件路径），参照 `src/schema/common.rs:1-16` 的写法。
- 类型命名沿用历史约定（`git show 9a91a15:src/models/auth.rs`）：请求体 `XxxReq`，响应/数据 `XxxResp`，条目类型用领域名词（如 `ObjResp`、`PermissionEntry`）；同一 JSON 形状被端点以不同名称引用时可用 `pub type` 别名（如 `pub type MeResp = UserResp;`）。
- 一个域一个文件；按端点分组排布模型（请求模型、响应模型、条目模型），不拆新文件。
- schema 模块已由 `src/schema.rs` 按 `<域>-schema` feature 门控、端点 feature 自动拉取镜像（Cargo.toml:40-51），**文件内部不需要也不得再加 `#[cfg]`**。
- 字段文档：公开字段逐个写 `///` 中文文档；能对应到 Go 字段/openapi 字段名的，在文档里注明来源（如「对应 Go `model.User.BasePath`」）。

## 示例 JSON 钉扎测试（完整模板）

每个 schema 文件在 `#[cfg(test)] mod tests` 中至少覆盖三类断言；示例 JSON 一律取自 `docs/api/alistv3.openapi.yaml` / `alistv3.md` 的真实示例或 Go handler 的实际返回，不要自造。参照 `src/schema/common.rs:126-264` 的既有写法。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 正向钉扎：openapi/Go 示例 JSON 反序列化，逐字段断言。
    #[test]
    fn me_resp_decodes_openapi_example() {
        // 示例来源：docs/api/alistv3.md `GET 获取当前用户信息` 的返回示例
        let me: MeResp = serde_json::from_value(serde_json::json!({
            "id": 1,
            "username": "admin",
            "password": "",
            "base_path": "/",
            "role": 2,                       // 钉住老服务器单值形状
            "disabled": false,
            "permission": 0,
            "otp": false,
            "sso_id": "",
            "role_names": ["admin"], // 钉住新服务器新增字段
            "permissions": []
        }))
        .unwrap();
        assert_eq!(me.username, "admin");
        assert_eq!(me.role, vec![2]); // 单值被展开为数组
        assert_eq!(me.role_names, vec!["admin".to_owned()]);
    }

    /// 2) 兼容钉扎：新增字段缺失、集合字段为空时都不炸（跨版本兼容）。
    #[test]
    fn me_resp_tolerates_missing_optional_fields() {
        // 老服务器响应：不含 role_names/permissions/sso_id 等新增字段
        let me: MeResp = serde_json::from_value(serde_json::json!({
            "id": 2,
            "username": "guest",
            "base_path": "/",
            "role": [],
            "disabled": false,
            "permission": 0,
            "otp": false
        }))
        .unwrap();
        assert!(me.role.is_empty());
        assert_eq!(me.role_names, Vec::<String>::new()); // 缺失 → default 空 Vec
        assert_eq!(me.sso_id, None); // 缺失 → default None
    }

    /// 2b) 显式 null 兼容（仅对加了 null_to_default 的字段成立；untagged 字段对 null 会失败）。
    #[test]
    fn me_resp_tolerates_null_collection_fields() {
        let me: MeResp = serde_json::from_value(serde_json::json!({
            "id": 2, "username": "guest", "base_path": "/", "role": [],
            "disabled": false, "permission": 0, "otp": false,
            "role_names": null, "permissions": null
        }))
        .unwrap();
        assert_eq!(me.role_names, Vec::<String>::new());
        assert!(me.permissions.is_empty()); // null_to_default 把 null 归约为空 Vec
        // 注：Option<T> 字段 serde 原生把 null 解为 None，无需辅助函数
    }

    /// 3) 序列化键名钉扎：请求模型用 serde_json::to_value 断言 JSON 键与 API 一致。
    #[test]
    fn login_req_serializes_with_api_field_names() {
        let req = LoginReq {
            username: "admin".to_owned(),
            password: "pw".to_owned(),
            otp_code: None,
        };
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({ "username": "admin", "password": "pw" }) // otp_code 被跳过
        );
    }
}
```

写测试时注意：

- `serde_json::json!` 数字与字段的类型要对齐（`i64` vs `u64` vs `f64`），先看 Go 结构体的类型再选；
- 每个钉扎测试的 doc 注释要写明示例 JSON 的来源（文件 + 端点）；
- 列表类模型同时钉 `PageResp<T>` 包裹形态（`{"content": [...], "total": N}`）下元素能解出。

## 自验命令清单（全部必须通过）

以下每条命令已在本仓库当前基线（骨架 + 已完成模块）逐条实测通过；实现 schema 后必须保持全绿。

```bash
cargo build --all-features
cargo test --all-features                      # 含 doctest；实测 30 单测 + 4 doctest 全绿
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --check                     # 实测 clean；禁止裸跑 cargo fmt
cargo check --no-default-features              # 实测 Finished；feature 门控完整性
# 按域验证（实现某域 schema 后至少跑对应组合）
cargo test --no-default-features --features <你的域>          # 如 auth / fs / public / admin-meta
cargo test --no-default-features --features admin            # admin 各子域
```

格式化只针对自己的文件：`cargo +nightly fmt -- src/schema/<域>.rs`。改完提交前自查：`git status` 中不得出现你所有权范围之外的文件名改动（只看不提交）。
