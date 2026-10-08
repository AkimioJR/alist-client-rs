//! admin-user 端点：更新用户。
//!
//! 对应 `POST /api/admin/user/update`；请求体为用户对象（JSON，`id` 与
//! `username` 必填，其余字段可选），响应 `data: null`，以 `()` 作为端点模型。
//!
//! 请求体 `role` 按当前服务端（Go `model.Roles []int`）以数组发送；
//! openapi 文档示例中的 `"role": 0` 为老版本单值形状（服务端旧模型 `Role int`），
//! 两者不兼容时以 `examples/alist` Go 源码为准。

use alist_client_derive::EndpointRequest;

/// 更新用户请求构建器。
///
/// 通过 [`User::update`](super::User::update) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/user/update", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标用户 ID（必选，定位待更新的用户）。
    id: i64,
    /// 用户名（必选）。
    username: String,
    /// 新密码（可选）。
    ///
    /// 为空或缺省时服务端保留原密码哈希
    /// （`examples/alist/server/handles/user.go` 的 `UpdateUser`）。
    password: Option<String>,
    /// 用户根目录路径（可选）。
    base_path: Option<String>,
    /// 角色 ID 列表（可选）。
    ///
    /// 内置角色：`0` 普通用户、`1` 访客、`2` 管理员；服务端禁止把 admin/guest
    /// 角色授予普通用户，也禁止修改 admin 用户自身的角色。
    role: Option<Vec<i32>>,
    /// 权限位掩码（可选）。
    ///
    /// 按位控制可见隐藏文件、免密码访问、离线下载、上传、重命名/移动/复制/删除、
    /// WebDAV 与 FTP 读写、压缩包读取与解压、路径限制、MCP 读写等能力，
    /// 位定义见 `examples/alist/internal/model/user.go` 的 `Can*` 系列方法。
    permission: Option<i32>,
    /// 是否禁用该用户（可选）。
    ///
    /// 服务端保证至少保留一个启用的管理员账号。
    disabled: Option<bool>,
    /// SSO 平台唯一标识（可选）。
    sso_id: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, id: i64, username: impl Into<String>) -> Self {
        Self {
            client,
            id,
            username: username.into(),
            password: None,
            base_path: None,
            role: None,
            permission: None,
            disabled: None,
            sso_id: None,
        }
    }
}

impl<'a> super::User<'a> {
    /// 更新用户信息。
    ///
    /// 对应 AList `POST /api/admin/user/update`；成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/user/update`、
    /// `docs/api/alistv3.md` 的 `# admin/user` 分组与
    /// `examples/alist/server/handles/user.go`（实现为 `UpdateUser`）。
    ///
    /// 注意：服务端按整体覆盖保存（`internal/db/user.go` 的 `UpdateUser` 落库为
    /// `db.Save`），请求中缺省的可选字段会以零值覆盖原值（仅 `password` 为空时
    /// 保留原密码哈希）；因此调用方应传入完整的目标状态。此外服务端禁止修改
    /// admin 用户的角色、禁止把 admin/guest 角色授予普通用户，且保证至少保留
    /// 一个启用的管理员账号。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标用户 ID。
    /// * `username` - 用户名。
    /// * `password` - 可选：新密码；缺省或为空时保留原密码。
    /// * `base_path` - 可选：用户根目录路径。
    /// * `role` - 可选：角色 ID 列表。
    /// * `permission` - 可选：权限位掩码。
    /// * `disabled` - 可选：是否禁用。
    /// * `sso_id` - 可选：SSO 平台唯一标识。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如用户不存在或角色变更被拒绝）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().user().update(3, "alice")
    ///     .base_path("/data") // 可选参数链式 setter；String 字段可直接传 &str
    ///     .permission(60)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn update(&self, id: i64, username: impl Into<String>) -> Request<'a> {
        Request::new(self.client, id, username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST /api/admin/user/update，请求体携带必选的 id 与 username，
    /// 未设置的可选字段不出现在请求体中。
    #[test]
    fn build_request_serializes_required_and_optional_fields() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3, "alice")
            .base_path("/data")
            .role(vec![0])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/user/update"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "请求体字段不应出现在查询串: {url}");

        let body = built
            .body()
            .expect("应携带 JSON 请求体")
            .as_bytes()
            .expect("请求体应为字节缓冲");
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        // 键名钉扎：与 docs/api/alistv3.md `# admin/user` POST update 的 Body 示例一致
        assert_eq!(json["id"], 3);
        assert_eq!(json["username"], "alice");
        assert_eq!(json["base_path"], "/data");
        assert_eq!(json["role"], serde_json::json!([0]));
        assert!(
            json.get("password").is_none(),
            "未设置的可选字段不应出现: {json}"
        );
        assert!(
            json.get("permission").is_none(),
            "未设置的可选字段不应出现: {json}"
        );
        assert!(
            json.get("disabled").is_none(),
            "未设置的可选字段不应出现: {json}"
        );
        assert!(
            json.get("sso_id").is_none(),
            "未设置的可选字段不应出现: {json}"
        );
    }

    /// 收发路径：mock 服务器 + 请求原文记录，验证完整请求与 `data: null` 解码。
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

        Request::new(&client, 3, "alice")
            .password("new-pass")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/user/update "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"id\":3"), "{}", recorded[0]);
        assert!(
            recorded[0].contains("\"username\":\"alice\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"password\":\"new-pass\""),
            "{}",
            recorded[0]
        );
    }
}
