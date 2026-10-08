//! admin-user 端点：创建用户。
//!
//! 对应 `POST /api/admin/user/create`；请求体为用户对象（JSON，仅 `username`
//! 必填，其余字段可选），响应 `data: null`，以 `()` 作为端点模型。
//!
//! 请求体 `role` 按当前服务端（Go `model.Roles []int`）以数组发送；
//! openapi 文档示例中的 `"role": 0` 为老版本单值形状（服务端旧模型 `Role int`），
//! 两者不兼容时以 `examples/alist` Go 源码为准。

use alist_client_derive::EndpointRequest;

/// 创建用户请求构建器。
///
/// 通过 [`User::create`](super::User::create) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/user/create", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 用户名（必选，全局唯一）。
    username: String,
    /// 用户 ID（可选）。
    ///
    /// 通常无需设置（文档示例传 `0`）；服务端以数据库自增主键落库。
    id: Option<i64>,
    /// 初始密码（可选）。
    ///
    /// 服务端会对该明文密码加盐散列后存储，并清空落库记录中的 `password` 字段
    /// （`examples/alist/server/handles/user.go` 的 `CreateUser`）。
    password: Option<String>,
    /// 用户根目录路径（可选）。
    ///
    /// 注意：当前服务端在创建时强制将其重置为 `/`
    /// （`examples/alist/internal/op/user.go` 的 `CreateUser`）。
    base_path: Option<String>,
    /// 角色 ID 列表（可选）。
    ///
    /// 缺省或为空时，服务端回退为系统默认角色
    /// （`examples/alist/server/handles/user.go` 的 `CreateUser`）。
    /// 内置角色：`0` 普通用户、`1` 访客、`2` 管理员；不得创建 admin/guest 用户。
    role: Option<Vec<i32>>,
    /// 权限位掩码（可选）。
    ///
    /// 按位控制可见隐藏文件、免密码访问、离线下载、上传、重命名/移动/复制/删除、
    /// WebDAV 与 FTP 读写、压缩包读取与解压、路径限制、MCP 读写等能力，
    /// 位定义见 `examples/alist/internal/model/user.go` 的 `Can*` 系列方法。
    permission: Option<i32>,
    /// 是否禁用该用户（可选，缺省为启用）。
    disabled: Option<bool>,
    /// SSO 平台唯一标识（可选）。
    sso_id: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, username: impl Into<String>) -> Self {
        Self {
            client,
            username: username.into(),
            id: None,
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
    /// 创建用户。
    ///
    /// 对应 AList `POST /api/admin/user/create`；成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/user/create`、
    /// `docs/api/alistv3.md` 的 `# admin/user` 分组与
    /// `examples/alist/server/handles/user.go`（实现为 `CreateUser`）。
    ///
    /// 注意：服务端拒绝创建 admin/guest 用户（`CreateUser` 中的 `IsAdmin`/`IsGuest`
    /// 检查）；`base_path` 会被强制重置为 `/`；`role` 缺省时回退为系统默认角色。
    ///
    /// # Arguments
    ///
    /// * `username` - 新用户的用户名（全局唯一）。
    /// * `id` - 可选：用户 ID，通常无需设置。
    /// * `password` - 可选：初始密码。
    /// * `base_path` - 可选：用户根目录路径（服务端会重置为 `/`）。
    /// * `role` - 可选：角色 ID 列表，缺省时服务端回退为默认角色。
    /// * `permission` - 可选：权限位掩码。
    /// * `disabled` - 可选：是否禁用，缺省为启用。
    /// * `sso_id` - 可选：SSO 平台唯一标识。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如用户名已存在或试图创建 admin/guest 用户）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// client.admin().user().create("alice")
    ///     .password("secret") // 可选参数链式 setter；String 字段可直接传 &str
    ///     .permission(60)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn create(&self, username: impl Into<String>) -> Request<'a> {
        Request::new(self.client, username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST /api/admin/user/create，JSON 请求体键名与文档示例一致，
    /// 未设置的可选字段不出现在请求体中。
    #[test]
    fn build_request_serializes_body_with_api_field_names() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "a")
            .password("123456")
            .role(vec![0])
            .permission(60)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/user/create"),
            "URL 应包含路径: {url}"
        );
        assert!(!url.contains('?'), "请求体字段不应出现在查询串: {url}");
        let content_type = built
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .expect("JSON 请求体应携带 Content-Type");
        assert!(
            content_type
                .to_str()
                .unwrap()
                .starts_with("application/json"),
            "{content_type:?}"
        );

        let body = built
            .body()
            .expect("应携带 JSON 请求体")
            .as_bytes()
            .expect("请求体应为字节缓冲");
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        // 键名钉扎：与 docs/api/alistv3.md `# admin/user` POST create 的 Body 示例一致
        assert_eq!(json["username"], "a");
        assert_eq!(json["password"], "123456");
        assert_eq!(json["permission"], 60);
        // role 按当前服务端 Go model.Roles []int 以数组发送
        assert_eq!(json["role"], serde_json::json!([0]));
        // 未设置的可选字段不应出现
        assert!(json.get("id").is_none(), "未设置的可选字段不应出现: {json}");
        assert!(
            json.get("base_path").is_none(),
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

        Request::new(&client, "a")
            .disabled(true)
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/user/create "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"username\":\"a\""),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"disabled\":true"), "{}", recorded[0]);
    }
}
