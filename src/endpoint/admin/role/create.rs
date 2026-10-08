//! admin-role 端点：创建角色。
//!
//! 对应 `POST /api/admin/role/create`（openapi 未收录该分组，路由见
//! `examples/alist/server/router.go:152`）。处理函数 `handles.CreateRole`
//! 绑定 `model.Role`（`name` 带 `binding:"required"`，角色 ID 由数据库自增分配），
//! 成功时响应 `data: null`（见 `examples/alist/server/handles/role.go:45-56`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::role::PermissionEntry;

/// 创建角色请求构建器。
///
/// 通过 [`Role::create`](super::Role::create) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/role/create", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 角色名（必选；服务端要求唯一，Go 侧 `binding:"required"`）。
    name: String,
    /// 角色描述（可选；缺省为空字符串）。
    description: Option<String>,
    /// 是否为默认角色（可选；缺省为 `false`，新用户注册时自动绑定默认角色）。
    default: Option<bool>,
    /// 各路径前缀上的权限条目（可选；缺省时角色不含任何权限）。
    permission_scopes: Option<Vec<PermissionEntry>>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
            description: None,
            default: None,
            permission_scopes: None,
        }
    }
}

impl<'a> super::Role<'a> {
    /// 创建角色。
    ///
    /// 对应 AList `POST /api/admin/role/create`；成功时响应 `data` 为 `null`。
    /// 数据来源：`examples/alist/server/router.go:152`（`handles.CreateRole`）与
    /// `examples/alist/server/handles/role.go:45-56`（绑定 `model.Role`）；
    /// 该分组不在 openapi 中，以 Go 源码为准。
    ///
    /// # Arguments
    ///
    /// * `name` - 角色名；服务端要求唯一。
    /// * `description` - 可选：角色描述。
    /// * `default` - 可选：是否设为默认角色（缺省 `false`）。
    /// * `permission_scopes` - 可选：各路径前缀上的权限条目；缺省时角色不含任何权限。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如角色名重复）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::role::PermissionEntry;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.admin().role().create("editor")
    ///     .description("可读写 /data")
    ///     .permission_scopes(vec![PermissionEntry {
    ///         path: "/data".to_owned(),
    ///         permission: 0b0110,
    ///     }])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn create(&self, name: impl Into<String>) -> Request<'a> {
        Request::new(self.client, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径；仅必选字段时可选字段不序列化。
    #[test]
    fn build_request_sends_only_required_body_when_optionals_unset() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "editor")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/role/create"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert_eq!(
            body, r#"{"name":"editor"}"#,
            "可选字段缺省时不应出现: {body}"
        );
    }

    /// 请求形状：可选字段全部设置时按 API 键名序列化。
    #[test]
    fn build_request_serializes_optional_fields_with_api_names() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "editor")
            .description("可读写 /data")
            .default(true)
            .permission_scopes(vec![PermissionEntry {
                path: "/data".to_owned(),
                permission: 0b0110,
            }])
            .build_request()
            .build()
            .unwrap();
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(
            body.contains(r#""name":"editor""#)
                && body.contains(r#""description":"可读写 /data""#)
                && body.contains(r#""default":true"#)
                && body.contains(r#""permission_scopes":[{"path":"/data","permission":6}]"#),
            "请求体应包含全部字段且键名与 API 一致: {body}"
        );
    }

    /// 收发路径：`data: null` 解码为 `()`，请求行与请求体符合预期。
    #[tokio::test]
    async fn send_posts_role_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "editor")
            .permission_scopes(vec![PermissionEntry {
                path: "/data".to_owned(),
                permission: 15,
            }])
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/role/create "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""name":"editor""#)
                && recorded[0]
                    .contains(r#""permission_scopes":[{"path":"/data","permission":15}]"#),
            "{}",
            recorded[0]
        );
    }
}
