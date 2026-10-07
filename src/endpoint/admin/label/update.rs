//! admin-label 端点：更新标签。
//!
//! 对应 `POST /api/admin/label/update`（openapi 未收录该分组，路由以
//! `examples/alist/server/router.go` 的 admin `label` 分组为准；实现为
//! `handles.UpdateLabel`）。请求体为完整标签字段（`id` + `name` 必填，JSON
//! 绑定到 Go `model.Label`）；成功时响应 `data` 为更新后的标签条目。
//! 注意服务端以整体保存（gorm `Save`）语义更新：未提供（被跳过）的可选字段
//! 会落为 Go 零值，需要保留的字段应显式传入原值。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label::Label;

/// 更新标签请求构建器。
///
/// 通过 [`Label::update`](super::Label::update) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/label/update", model = Label)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 标签 ID（必选）；服务端按它定位待更新的标签。
    id: u64,
    /// 标签名称（必选）。
    name: String,
    /// 标签类型（可选）；JSON 键为保留字 `type`，故以 raw identifier 命名字段。
    r#type: Option<i32>,
    /// 标签描述（可选）；缺省时服务端按空串保存。
    description: Option<String>,
    /// 标签背景色（可选），例如 `#FF0000`；缺省时服务端按空串保存。
    bg_color: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数（标签 ID 与名称）；其余字段走派生 setter。
    ///
    /// 返回类型 [`Request`] 已整体标记 `#[must_use]`，此处不再重复标注。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client, id: u64, name: impl Into<String>) -> Self {
        Self {
            client,
            id,
            name: name.into(),
            r#type: None,
            description: None,
            bg_color: None,
        }
    }
}

impl<'a> super::Label<'a> {
    /// 更新标签。
    ///
    /// 对应 AList `POST /api/admin/label/update`；请求体为完整标签字段（JSON 绑定到
    /// Go `model.Label`），成功时响应 `data` 为更新后的标签条目，解码为
    /// [`Label`]。标签不存在时服务端返回 `500` 错误信封。
    /// 数据来源：`examples/alist/server/router.go` 的 admin `label` 路由与
    /// `examples/alist/server/handles/label.go`（实现为 `UpdateLabel`）；
    /// openapi 文档未收录该分组。
    ///
    /// # Arguments
    ///
    /// * `id` - 待更新标签的 ID。
    /// * `name` - 新的标签名称。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Label`](crate::schema::admin::label::Label)。
    /// 可选字段（标签类型 `type`、描述 `description`、背景色 `bg_color`）
    /// 通过 [`Request`] 的链式 setter 设置。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如标签不存在时服务端返回 `500` 错误信封）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let label = client
    ///     .admin()
    ///     .label()
    ///     .update(1, "新名称")
    ///     .description("新描述")
    ///     .await?;
    /// println!("更新后名称: {}", label.name);
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

    /// 标签条目示例 JSON（按 `examples/alist/internal/model/label.go` 的 JSON tag 构造）。
    const LABEL_JSON: &str = r##"{"id":1,"type":2,"name":"电影","description":"电影相关文件","bg_color":"#FF0000","create_time":"2024-06-01T12:00:00Z"}"##;

    /// 1) URL/方法/请求体断言：build_request().build() 检查 method、URL 与 JSON body。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, 1, "新名称")
            .r#type(3)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().contains("/api/admin/label/update"),
            "URL 应包含路径: {}",
            built.url()
        );
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"id\":1"), "{body}");
        assert!(body.contains("\"name\":\"新名称\""), "{body}");
        // JSON 键名钉扎：Go `model.Label.Type` 的 JSON tag 为保留字 `type`
        assert!(body.contains("\"type\":3"), "{body}");
        assert!(
            !body.contains("description") && !body.contains("bg_color"),
            "None 可选字段应被跳过: {body}"
        );
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文，响应解码为更新后的 Label。
    #[tokio::test]
    async fn send_updates_label_and_returns_it() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = format!(r#"{{"code":200,"message":"success","data":{LABEL_JSON}}}"#);
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let label = Request::new(&client, 1, "电影")
            .description("电影相关文件")
            .bg_color("#FF0000")
            .send()
            .await
            .unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.name, "电影");
        assert_eq!(label.description, "电影相关文件");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label/update "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"id\":1"), "{}", recorded[0]);
        assert!(recorded[0].contains("\"name\":\"电影\""), "{}", recorded[0]);
        assert!(
            recorded[0].contains("\"bg_color\":\"#FF0000\""),
            "{}",
            recorded[0]
        );
    }
}
