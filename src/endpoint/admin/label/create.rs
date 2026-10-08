//! admin-label 端点：创建标签。
//!
//! 对应 `POST /api/admin/label/create`（AList OpenAPI 规范未收录该分组，路由以
//! AList 服务端路由定义的 admin `label` 分组为准；实现为
//! `handles.CreateLabel`）。请求体为标签字段（名称必填，JSON 绑定到 AList
//! 服务端标签模型）；成功时响应 `data` 为 `{ "id": N }`。
//! 服务端在入库时自动生成 `id` 并以当前时间覆盖 `create_time`
//! （见 AList 服务端数据库实现中的 `CreateLabel`），故本构建器不暴露这两个字段。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label::CreateLabelResponse;

/// 创建标签请求构建器。
///
/// 通过 [`Label::create`](super::Label::create) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/label/create",
    model = CreateLabelResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 标签名称（必选参数）。
    ///
    /// 服务端校验同名标签已存在时返回错误。
    name: String,
    /// 标签类型（可选参数）。
    ///
    /// JSON 键为保留字 `type`，故以 raw identifier 命名字段。
    r#type: Option<i32>,
    /// 标签描述（可选参数）。
    description: Option<String>,
    /// 标签背景色（可选参数）。
    ///
    /// 例如十六进制颜色值 `#FF0000`。
    bg_color: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数（标签名称）；其余字段走派生 setter。
    ///
    /// 返回类型 [`Request`] 已整体标记 `#[must_use]`，此处不再重复标注。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
            r#type: None,
            description: None,
            bg_color: None,
        }
    }
}

impl<'a> super::Label<'a> {
    /// 创建标签。
    ///
    /// 对应 AList `POST /api/admin/label/create`；请求体为标签字段（JSON 绑定到
    /// AList 服务端标签模型），成功时响应 `data` 为 `{ "id": N }`，解码为
    /// [`CreateLabelResponse`]。同名标签已存在时服务端返回 `401` 错误响应。
    /// 数据来源：AList 服务端路由定义的 admin `label` 路由与
    /// AList 服务端 label 模块（实现为 `CreateLabel`）；
    /// AList OpenAPI 规范未收录该分组。
    ///
    /// # Arguments
    ///
    /// * `name` - 标签名称，服务端内唯一。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`CreateLabelResponse`]（新建标签的 ID）。
    /// 可选字段（标签类型 `type`、描述 `description`、背景色 `bg_color`）
    /// 通过 [`Request`] 的链式 setter 设置。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如标签名已存在时服务端返回 `401` 错误响应）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let resp = client
    ///     .admin()
    ///     .label()
    ///     .create("电影")
    ///     .r#type(2)
    ///     .bg_color("#FF0000")
    ///     .await?;
    /// println!("新标签 ID: {}", resp.id);
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

    /// 1) URL/方法/请求体断言：build_request().build() 检查 method、URL 与 JSON body。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "电影")
            .r#type(2)
            .bg_color("#FF0000")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().contains("/api/admin/label/create"),
            "URL 应包含路径: {}",
            built.url()
        );
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"name\":\"电影\""), "{body}");
        // JSON 键名钉扎：服务端标签类型的 JSON 键为保留字 `type`
        assert!(body.contains("\"type\":2"), "{body}");
        assert!(body.contains("\"bg_color\":\"#FF0000\""), "{body}");
        assert!(
            !body.contains("description"),
            "None 可选字段应被跳过: {body}"
        );
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文，响应解码为 CreateLabelResponse。
    #[tokio::test]
    async fn send_creates_label_and_returns_id() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"id":42}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "电影")
            .description("电影相关文件")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.id, 42);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label/create "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"name\":\"电影\""), "{}", recorded[0]);
        assert!(
            recorded[0].contains("\"description\":\"电影相关文件\""),
            "{}",
            recorded[0]
        );
    }
}
