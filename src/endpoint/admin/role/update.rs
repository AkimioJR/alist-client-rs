//! admin-role 端点：更新角色。
//!
//! 对应 `POST /api/admin/role/update`（openapi 未收录该分组，路由见
//! `examples/alist/server/router.go:153`）。处理函数 `handles.UpdateRole`
//! 先按 `id` 读取现角色再整体覆盖（内置 `admin` 角色被服务端拒绝，
//! `guest` 角色不可改名），成功时响应 `data: null`
//! （见 `examples/alist/server/handles/role.go:58-94`）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::role::PermissionEntry;

/// 更新角色请求构建器。
///
/// 通过 [`Role::update`](super::Role::update) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/role/update", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标角色 ID（必选；服务端据此读取现角色）。
    id: u64,
    /// 新角色名（必选；服务端 `binding:"required"`，`guest` 角色不可改名）。
    name: String,
    /// 新角色描述（可选）。
    ///
    /// 注意：省略时服务端会将描述置为空字符串（Go 零值整体覆盖，非增量更新）。
    description: Option<String>,
    /// 新的路径级权限条目（可选）。
    ///
    /// 注意：省略或传空数组时服务端都会**清空**该角色的全部权限条目
    /// （Go 侧以请求切片整体覆盖后 `BeforeSave` 置空存储）；
    /// 增量修改请先 [`Role::get`](super::Role::get) 读取现值，合并后再提交完整列表。
    permission_scopes: Option<Vec<PermissionEntry>>,
    /// 是否设为默认角色（可选）。
    ///
    /// 该字段在服务端为三态指针（Go `*bool`）：省略时保持现值不变。
    default: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, id: u64, name: impl Into<String>) -> Self {
        Self {
            client,
            id,
            name: name.into(),
            description: None,
            permission_scopes: None,
            default: None,
        }
    }
}

impl<'a> super::Role<'a> {
    /// 更新角色。
    ///
    /// 对应 AList `POST /api/admin/role/update`；成功时响应 `data` 为 `null`。
    /// 服务端按 `id` 读取现角色后整体覆盖：`default` 省略时保持现值，
    /// `description`/`permission_scopes` 省略时分别置空/清空（见下）。
    /// 数据来源：`examples/alist/server/router.go:153`（`handles.UpdateRole`）与
    /// `examples/alist/server/handles/role.go:58-94`；该分组不在 openapi 中，以 Go 源码为准。
    ///
    /// # Arguments
    ///
    /// * `id` - 目标角色 ID。
    /// * `name` - 新角色名（`guest` 角色不可改名）。
    /// * `description` - 可选：新角色描述；省略时服务端置为空字符串。
    /// * `permission_scopes` - 可选：新的权限条目列表；省略或为空时服务端清空全部权限。
    /// * `default` - 可选：是否设为默认角色；省略时保持服务端现值。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如角色不存在或修改内置 `admin` 角色被拒绝）时，返回 [`crate::Error`]。
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
    /// client.admin().role().update(3, "editor")
    ///     .description("只可读写 /data/public")
    ///     .permission_scopes(vec![PermissionEntry {
    ///         path: "/data/public".to_owned(),
    ///         permission: 0b0110,
    ///     }])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn update(&self, id: u64, name: impl Into<String>) -> Request<'a> {
        Request::new(self.client, id, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径；请求体含必选的 id/name，可选字段缺省时跳过。
    #[test]
    fn build_request_sends_required_fields_and_skips_unset_optionals() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3, "editor")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/role/update"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert_eq!(
            body, r#"{"id":3,"name":"editor"}"#,
            "可选字段缺省时不应出现: {body}"
        );
    }

    /// 请求形状：可选字段（含三态 `default`）按 API 键名序列化。
    #[test]
    fn build_request_serializes_optional_fields_with_api_names() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 3, "editor")
            .description("只可读写 /data/public")
            .permission_scopes(vec![PermissionEntry {
                path: "/data/public".to_owned(),
                permission: 0b0110,
            }])
            .default(true)
            .build_request()
            .build()
            .unwrap();
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(
            body.contains(r#""id":3"#)
                && body.contains(r#""name":"editor""#)
                && body.contains(r#""description":"只可读写 /data/public""#)
                && body.contains(r#""permission_scopes":[{"path":"/data/public","permission":6}]"#)
                && body.contains(r#""default":true"#),
            "请求体应包含全部字段且键名与 API 一致: {body}"
        );
    }

    /// 收发路径：`data: null` 解码为 `()`，请求行与请求体符合预期。
    #[tokio::test]
    async fn send_posts_update_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, 3, "editor")
            .default(false)
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/role/update "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""id":3"#)
                && recorded[0].contains(r#""name":"editor""#)
                && recorded[0].contains(r#""default":false"#),
            "{}",
            recorded[0]
        );
    }
}
