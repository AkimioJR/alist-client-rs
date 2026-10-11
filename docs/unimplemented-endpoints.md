# 未实现的 AList 端点清单

本文档对照 vendored 参考源码 `examples/alist/server/router.go`（AList v3）逐条记录本 crate **有意不建模、暂缓实现或范围外**的路由。功能性 `/api` JSON 端点已全量实现；核对方法：`grep -rn 'path = "' src/endpoint` 与 router.go 全量路由逐一比对（任务接口由 `src/endpoint/task` 的通用构建器按「前缀 × 类别 × 动作」参数化生成路径，`/api/fs/put`、`/api/fs/form`、`/api/fs/lark/export/download` 为手写请求路径，均需单独核对），未实现项的 handler 位置以 `grep -rn "func <Handler>(" examples/alist/server/handles` 定位。

- 清单基线：`examples/alist`（vendored 参考 AList v3 源码）。
- HTTP 方法一栏中 `Any` 表示 router.go 使用 `g.Any(...)`；按仓库约定读端点视为 GET、写端点视为 POST（此处多数为读端点）。
- 「位置」格式：`router.go:<行>`（路由注册）+ handler 实现文件与行号。

## 1. SSO 登录（4 条，无鉴权回调）

**有意不建模**（与第 3 节同理）：SSO 四端点是浏览器弹窗回调流的中间环节，
成功响应均不走 JSON 信封。依据 AList 服务端 `handles/ssologin.go`
（`SSOLoginRedirect`、`SSOLoginCallback`，全文件无任何 `c.JSON` 成功应答）：

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/auth/sso` | router.go:88；`handles/ssologin.go:58`（`SSOLoginRedirect`） | 重定向到 SSO 服务发起单点登录。 |
| GET | `/api/auth/sso_callback` | router.go:89；`handles/ssologin.go:284`（`SSOLoginCallback`） | 处理 SSO 回调并签发会话。 |
| GET | `/api/auth/get_sso_id` | router.go:90；`handles/ssologin.go:284`（`SSOLoginCallback`） | 复用同一回调处理获取 SSO 身份 ID。 |
| GET | `/api/auth/sso_get_token` | router.go:91；`handles/ssologin.go:284`（`SSOLoginCallback`） | 复用同一回调处理通过 SSO 换取 token。 |

- `/api/auth/sso` 恒为 302 重定向到 OAuth2 授权页；
- `/api/auth/get_sso_id` 与 `/api/auth/sso_get_token` 复用同一 handler（按 `method`
  查询参数或兼容模式下的 URL 路径区分）：启用 SSO 兼容模式时返回 302 重定向
  （携带 `sso_id` 或 `token` 查询参数），否则返回 `text/html` 页面，
  由内嵌脚本经 `window.opener.postMessage({"sso_id": ...})` /
  `window.opener.postMessage({"token": ...})` 交付载荷；
- 服务端不存在「SSO 身份信息 / token 的 JSON 成功响应」结构体（错误应答仍为
  JSON 信封），故不为其建模端点与 schema。

## 2. WebAuthn（6 条，暂缓实现）

**暂缓实现**（维护者决定，非 API 形状问题）：六端点的成功/错误应答均走 AList 标准
JSON 信封（服务端统一经 `common.SuccessResp`/`common.ErrorResp` 应答，见
`handles/webauthn.go`），形状本身可正常建模，暂缓仅为维护者的排期取舍：

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/authn/webauthn_begin_login` | router.go:94；`handles/webauthn.go:21`（`BeginAuthnLogin`） | 生成 WebAuthn 登录挑战。 |
| POST | `/api/authn/webauthn_finish_login` | router.go:95；`handles/webauthn.go:62`（`FinishAuthnLogin`） | 校验挑战应答并完成登录。 |
| GET | `/api/authn/webauthn_begin_registration` | router.go:96；`handles/webauthn.go:122`（`BeginAuthnRegistration`） | 生成 WebAuthn 注册挑战。 |
| POST | `/api/authn/webauthn_finish_registration` | router.go:97；`handles/webauthn.go:152`（`FinishAuthnRegistration`） | 保存认证器注册结果。 |
| POST | `/api/authn/delete_authn` | router.go:98；`handles/webauthn.go:198`（`DeleteAuthnLogin`） | 删除已绑定的 WebAuthn 凭据。 |
| GET | `/api/authn/getcredentials` | router.go:99；`handles/webauthn.go:222`（`GetAuthnCredentials`） | 列出当前用户的 WebAuthn 凭据。 |

## 3. 范围外：非 JSON API 路由（未实现，多数也超出本 crate 定位）

以下路由由 router.go 注册，但它们是**内容分发/网关类路由**，不走 JSON 响应包装，通常由客户端直接拼 URL 或专用协议访问，本次未建模：

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/favicon.ico` | router.go:33（`handles.Favicon`） | 站点图标。 |
| GET | `/robots.txt` | router.go:34（`handles.Robots`） | 爬虫规则。 |
| GET | `/i/:link_name` | router.go:35（`handles.Plist`） | iOS ipa 安装所需的 plist 描述。 |
| GET/HEAD | `/d/*path` | router.go:46-48（`handles.Down`） | 带签名的直接下载（302 或流式）。 |
| GET/HEAD | `/p/*path` | router.go:47-49（`handles.Proxy`） | 带签名的代理下载（服务端中转流）。 |
| GET | `/s/:share_id`、`/s/:share_id/*path` | router.go:50-51（`handles.GetSharePage`） | 分享落地页。 |
| GET/HEAD | `/sd/:share_id`、`/sd/:share_id/*path` | router.go:52-55（`handles.ShareDown`） | 分享文件直接下载。 |
| GET/HEAD | `/sp/:share_id`、`/sp/:share_id/*path` | router.go:56-59（`handles.ShareProxy`） | 分享文件代理下载。 |
| GET/HEAD | `/ad/*path`、`/ap/*path`、`/ae/*path` | router.go:61-66（`handles.ArchiveDown`/`ArchiveProxy`/`ArchiveInternalExtract`） | 压缩包下载/代理/内部解压。 |
| — | `/dav` | router.go:41（`server/webdav.go`，`WebDav`） | WebDAV 网关。 |
| — | `/s3` | router.go:42（S3 网关） | S3 兼容 API 网关。 |
| — | 其余静态资源 | router.go:125-127（`static.Static`） | 前端静态文件与 NoRoute 兜底。 |
| Any | `/ping` | router.go:30-32 | **已实现**（`src/endpoint/public/ping.rs`，纯文本应答）；列于此处仅为完整性。 |

## 汇总

router.go 共注册 **294 条 `/api` 路由**（含任务接口 `/api/task` 与 `/api/admin/task`
两前缀 × 7 类别 × 12 动作共 168 条），本 crate 已实现其中 **284 条**；剩余 10 条均为
有意保留未建模：SSO 4 条（浏览器回调流，见第 1 节）与 WebAuthn 6 条（暂缓实现，
见第 2 节）。另有一批非 JSON API 的下载/网关路由见第 3 节（其中 `/ping` 已实现，
列于表中仅为完整性）。
