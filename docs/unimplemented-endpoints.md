# 未实现的 AList 端点清单

本文档对照 vendored 参考源码 `examples/alist/server/router.go`（AList v3，`g.Any(...)` 读端点按 GET、写端点按 POST 处理，见 router.go:223-228 注释约定）逐条列出**已注册但本 crate 本次未实现**的端点。已实现的端点不在本清单中；核对方法：`grep -rn 'path = "' src/endpoint` 与 router.go 全量路由逐一比对，未实现项的 handler 位置以 `grep -rn "func <Handler>(" examples/alist/server/handles` 定位。

- 清单基线：`examples/alist`（vendored 参考 AList v3 源码）。
- HTTP 方法一栏中 `Any` 表示 router.go 使用 `g.Any(...)`；按仓库约定读端点视为 GET、写端点视为 POST（此处多数为读端点）。
- 「位置」格式：`router.go:<行>`（路由注册）+ handler 实现文件与行号。

## 1. auth 域遗漏（8 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| POST | `/api/auth/login/ldap` | router.go:74；`handles/ldap_login.go:20`（`LoginLdap`） | 使用 LDAP 目录凭据登录。 |
| POST | `/api/me/update` | router.go:77；`handles/auth.go:193`（`UpdateCurrent`） | 修改当前登录用户的资料（昵称、密码等）。 |
| GET | `/api/me/sshkey/list` | router.go:78；`handles/sshkey.go:48`（`ListMyPublicKey`） | 列出当前用户的 SSH 公钥。 |
| POST | `/api/me/sshkey/add` | router.go:79；`handles/sshkey.go:17`（`AddMyPublicKey`） | 为当前用户新增 SSH 公钥。 |
| POST | `/api/me/sshkey/delete` | router.go:80；`handles/sshkey.go:57`（`DeleteMyPublicKey`） | 删除当前用户的 SSH 公钥。 |
| GET | `/api/auth/logout` | router.go:83；`handles/auth.go:273`（`LogOut`） | 退出登录并注销当前会话。 |
| GET | `/api/me/sessions` | router.go:84；`handles/session.go:19`（`ListMySessions`） | 列出当前用户的登录会话。 |
| POST | `/api/me/sessions/evict` | router.go:85；`handles/session.go:43`（`EvictMySession`） | 踢出当前用户的指定会话。 |

## 2. SSO 登录（4 条，无鉴权回调）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/auth/sso` | router.go:88；`handles/ssologin.go:58`（`SSOLoginRedirect`） | 重定向到 SSO 服务发起单点登录。 |
| GET | `/api/auth/sso_callback` | router.go:89；`handles/ssologin.go:284`（`SSOLoginCallback`） | 处理 SSO 回调并签发会话。 |
| GET | `/api/auth/get_sso_id` | router.go:90；`handles/ssologin.go:284`（`SSOLoginCallback`） | 复用同一回调处理获取 SSO 身份 ID。 |
| GET | `/api/auth/sso_get_token` | router.go:91；`handles/ssologin.go:284`（`SSOLoginCallback`） | 复用同一回调处理通过 SSO 换取 token。 |

## 3. WebAuthn（6 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/authn/webauthn_begin_login` | router.go:94；`handles/webauthn.go:21`（`BeginAuthnLogin`） | 生成 WebAuthn 登录挑战。 |
| POST | `/api/authn/webauthn_finish_login` | router.go:95；`handles/webauthn.go:62`（`FinishAuthnLogin`） | 校验挑战应答并完成登录。 |
| GET | `/api/authn/webauthn_begin_registration` | router.go:96；`handles/webauthn.go:122`（`BeginAuthnRegistration`） | 生成 WebAuthn 注册挑战。 |
| POST | `/api/authn/webauthn_finish_registration` | router.go:97；`handles/webauthn.go:152`（`FinishAuthnRegistration`） | 保存认证器注册结果。 |
| POST | `/api/authn/delete_authn` | router.go:98；`handles/webauthn.go:198`（`DeleteAuthnLogin`） | 删除已绑定的 WebAuthn 凭据。 |
| GET | `/api/authn/getcredentials` | router.go:99；`handles/webauthn.go:222`（`GetAuthnCredentials`） | 列出当前用户的 WebAuthn 凭据。 |

## 4. public 域遗漏（6 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| Any | `/api/public/offline_download_tools` | router.go:104；`handles/offline_download.go:333`（`OfflineDownloadTools`） | 查询服务端支持哪些离线下载工具（用于前端按能力展示）。 |
| Any | `/api/public/archive_extensions` | router.go:105；`handles/archive.go:429`（`ArchiveExtensions`） | 查询服务端支持在线预览/解压的压缩包扩展名。 |
| GET | `/api/public/share/info` | router.go:106；`handles/share_public.go:15`（`GetPublicShareInfo`） | 获取分享的公开元信息（是否需要密码、是否启用等）。 |
| POST | `/api/public/share/auth` | router.go:107；`handles/share_public.go:55`（`AuthPublicShare`） | 校验分享密码，换取分享访问 token。 |
| POST | `/api/public/share/list` | router.go:108；`handles/share_public.go:92`（`ListPublicShare`） | 浏览分享内的目录列表。 |
| POST | `/api/public/share/get` | router.go:109；`handles/share_public.go:155`（`GetPublicShare`） | 获取分享内文件的详情与下载地址。 |

## 5. share 分享管理（5 条，`/api/share`，需非游客登录）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| POST | `/api/share/create` | router.go:113；`handles/share.go:434`（`CreateShare`） | 创建文件分享链接。 |
| POST | `/api/share/update` | router.go:114；`handles/share.go:506`（`UpdateShare`） | 修改分享的密码/有效期等属性。 |
| POST | `/api/share/disable` | router.go:115；`handles/share.go:605`（`DisableShare`） | 禁用分享。 |
| GET | `/api/share/list` | router.go:116；`handles/share.go:582`（`ListShares`） | 分页列出自己创建的分享。 |
| POST | `/api/share/delete` | router.go:117；`handles/share.go:623`（`DeleteShare`） | 删除分享。 |

## 6. fs 域遗漏（3 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| Any | `/api/fs/other` | router.go:226；`handles/fsread.go:515`（`FsOther`） | 获取特殊类型文件（Office 在线预览、ipa plist 等）的附加信息。 |
| GET | `/api/fs/lark/export/download` | router.go:227；`handles/lark.go:26`（`LarkExportDownload`） | 下载飞书（Lark）云文档的导出文件。 |
| POST | `/api/fs/link` | router.go:241（`middlewares.AuthAdmin`）；`handles/fsmanage.go:441`（`Link`） | 管理员获取文件的原始直链。 |

## 7. admin/user 遗漏（2 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/admin/user/sshkey/list` | router.go:146；`handles/sshkey.go:81`（`ListPublicKeys`） | 管理员列出任意用户的 SSH 公钥。 |
| POST | `/api/admin/user/sshkey/delete` | router.go:147；`handles/sshkey.go:95`（`DeletePublicKey`） | 管理员删除指定 SSH 公钥。 |

## 8. admin/setting 遗漏（10 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| POST | `/api/admin/setting/set_token` | router.go:177；`handles/setting.go:48`（`SetToken`） | 设置站点访问令牌（`token` 设置项）。 |
| POST | `/api/admin/setting/set_transmission` | router.go:180；`handles/offline_download.go:87`（`SetTransmission`） | 配置 Transmission 离线下载器。 |
| POST | `/api/admin/setting/set_115` | router.go:181；`handles/offline_download.go:117`（`Set115`） | 配置 115 网盘离线下载。 |
| POST | `/api/admin/setting/set_pikpak` | router.go:182；`handles/offline_download.go:161`（`SetPikPak`） | 配置 PikPak 离线下载。 |
| POST | `/api/admin/setting/set_thunder` | router.go:183；`handles/offline_download.go:205`（`SetThunder`） | 配置迅雷离线下载。 |
| POST | `/api/admin/setting/set_guangyapan` | router.go:184；`handles/offline_download.go:249`（`SetGuangYaPan`） | 配置光环云盘离线下载。 |
| POST | `/api/admin/setting/set_123_open` | router.go:185；`handles/offline_download.go:293`（`SetOpen123`） | 配置 123 云盘开放平台离线下载。 |
| POST | `/api/admin/setting/set_frp` | router.go:186；`handles/setting.go:189`（`SetFRP`） | 配置并启动 FRP 内网穿透。 |
| POST | `/api/admin/setting/stop_frp` | router.go:187；`handles/setting.go:207`（`StopFRP`） | 停止 FRP 内网穿透。 |
| GET | `/api/admin/setting/frp_runtime` | router.go:188；`handles/setting.go:213`（`GetFRPRuntime`） | 查询 FRP 运行时状态。 |

## 9. admin/message（2 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| POST | `/api/admin/message/get` | router.go:194；`internal/message/http.go:20`（`Http.GetHandle`） | 从内置消息队列拉取消息。 |
| POST | `/api/admin/message/send` | router.go:195；`internal/message/http.go:29`（`Http.SendHandle`） | 向内置消息队列发送消息。 |

## 10. admin/index 搜索索引（5 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| POST | `/api/admin/index/build` | router.go:198；`handles/index.go:21`（`BuildIndex`） | 全量构建搜索索引。 |
| POST | `/api/admin/index/update` | router.go:199；`handles/index.go:42`（`UpdateIndex`） | 增量更新搜索索引。 |
| POST | `/api/admin/index/stop` | router.go:200；`handles/index.go:74`（`StopIndex`） | 停止正在运行的索引任务。 |
| POST | `/api/admin/index/clear` | router.go:201；`handles/index.go:87`（`ClearIndex`） | 清空搜索索引。 |
| GET | `/api/admin/index/progress` | router.go:202；`handles/index.go:102`（`GetProgress`） | 查询索引构建进度。 |

## 11. admin/session（2 条）

| 方法 | 路径 | 位置 | 说明 |
|---|---|---|---|
| GET | `/api/admin/session/list` | router.go:217；`handles/session.go:61`（`ListSessions`） | 管理员列出全部用户会话。 |
| POST | `/api/admin/session/evict` | router.go:218；`handles/session.go:81`（`EvictSession`） | 管理员踢出任意用户的会话。 |

## 12. task 任务接口（按「前缀 × 类别 × 动作」矩阵展开）

`handles.SetupTaskRoute`（router.go:118 `_task(auth.Group("/task", ...))` 与 router.go:191 `_task(g.Group("/task"))`）调用通用注册函数 `taskRoute`（`handles/task.go:125-217`），在**两个前缀 × 七个类别**下注册同一组 **12 个动作**：

- 前缀（router.go:118、191）：
  - `/api/task/<类别>` —— 面向普通用户（auth + AuthNotGuest）；
  - `/api/admin/task/<类别>` —— 管理员（本 crate 已实现其中 `upload` 类别的 8 个基础动作）。
- 类别（`handles/task.go:219-225`）：`upload`、`copy`、`offline_download`、`offline_download_transfer`、`s3_transition`、`decompress`、`decompress_upload`。
- 动作（`handles/task.go:126-217`）：
  - GET `/undone`（task.go:126）、GET `/done`（task.go:139）；
  - POST `/info`（task.go:150）、`/cancel`（task.go:153）、`/delete`（task.go:156）、`/retry`（task.go:163）、`/cancel_some`（task.go:166）、`/delete_some`（task.go:169）、`/retry_some`（task.go:172）、`/clear_done`（task.go:175）、`/clear_succeeded`（task.go:188）、`/retry_failed`（task.go:201）。

**未实现组合**（已实现仅为 `/api/admin/task/upload/{undone,done,info,cancel,delete,retry,clear_done,clear_succeeded}`）：

| 范围 | 数量 | 说明 |
|---|---|---|
| `/api/admin/task/upload` 的批量动作 `/cancel_some`、`/delete_some`、`/retry_some`、`/retry_failed` | 4 | 已实现类别的批量操作变体。 |
| `/api/admin/task/<其余 6 类别>/*`（copy、offline_download、offline_download_transfer、s3_transition、decompress、decompress_upload × 12 动作） | 72 | 管理端其余任务类别的完整动作集。 |
| `/api/task/<7 类别>/*`（7 × 12 动作） | 84 | 面向普通用户的任务查询/管理接口（非管理员前缀）。 |

实现这些端点时可复用现有 `task` 句柄的 Request 模板，仅需按类别参数化路径前缀；响应模型为 `PageResponse<TaskInfo>` 形态（`taskRoute` 内 `getTaskInfos`/`getTaskInfo` 返回 `TaskInfo` 结构，`handles/task.go:31、62`）。

## 13. 范围外：非 JSON API 路由（未实现，多数也超出本 crate 定位）

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

未实现的 `/api` JSON 端点合计约 **213 条路由**：auth 18（含 SSO 4、WebAuthn 6）、public 6、share 5、fs 3、admin/user 2、admin/setting 10、admin/message 2、admin/index 5、admin/session 2、task 160（4 + 72 + 84）。另有一批非 JSON API 的下载/网关路由见第 13 节。
