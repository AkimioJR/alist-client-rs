//! admin-setting 端点：保存设置。
//!
//! 对应 `POST /api/admin/setting/save`；请求体为设置项 **JSON 数组**（而非对象包装），
//! 响应 `data` 为 `null`，以 `()` 作为端点模型。
//!
//! 由于 [`EndpointRequest`] 派生宏生成的请求体
//! 固定为对象包装结构，本端点将载荷字段标记为 `#[endpoint(skip)]`，发送时经
//! [`Request::send_settings`] 手动附加数组请求体（与上传特例同一处理方式，见 `docs/design.md` §4）；
//! **请勿使用派生生成的 `send`/`.await`**——它们不携带请求体，服务端会因绑定空请求体而报错。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::Setting;

/// 保存设置请求构建器。
///
/// 通过 [`Setting::save`](super::Setting::save) 创建；
/// **发送请使用 [`Request::send_settings`]**（请求体为设置项 JSON 数组）。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/save", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.send_settings().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待保存的设置项数组（必选；经 [`Request::send_settings`] 以 JSON 数组附加到请求体）。
    #[endpoint(skip)]
    settings: Vec<Setting>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, settings: Vec<Setting>) -> Self {
        Self { client, settings }
    }

    /// 附加设置项数组请求体并发送。
    ///
    /// 请求体为设置项 JSON 数组（对应服务端 `c.ShouldBind(&[]model.SettingItem)` 的绑定形态，
    /// 见 `examples/alist/server/handles/setting.go` 的 `SaveSettings`）；
    /// 成功时响应 `data` 为 `null`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    pub async fn send_settings(self) -> crate::Result<()> {
        let builder = self.build_request().json(&self.settings);
        self.client.execute(builder).await
    }
}

impl<'a> super::Setting<'a> {
    /// 批量保存设置项。
    ///
    /// 对应 AList `POST /api/admin/setting/save`；请求体为设置项数组，
    /// 成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `admin/setting/save` 与
    /// `examples/alist/server/handles/setting.go`（实现为 `SaveSettings`）。
    ///
    /// # Arguments
    ///
    /// * `settings` - 待保存的设置项数组；通常先经 [`Setting::list`](super::Setting::list)
    ///   读取后修改 `value` 字段再整体回存。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；**发送请调用
    /// [`.send_settings().await`](Request::send_settings)**，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::setting::Setting;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let items = vec![Setting {
    ///     key: "site_title".to_owned(),
    ///     value: "AList".to_owned(),
    ///     help: String::new(),
    ///     value_type: "string".to_owned(),
    ///     options: String::new(),
    ///     group: 1,
    ///     flag: 0,
    ///     index: 0,
    /// }];
    /// client.admin().setting().save(items).send_settings().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn save(&self, settings: Vec<Setting>) -> Request<'a> {
        Request::new(self.client, settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法与路径断言；派生宏的 `build_request` 不携带请求体
    /// （数组载荷由 `send_settings` 附加）。
    #[test]
    fn build_request_composes_method_and_url_without_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, Vec::new())
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/setting/save"),
            "URL 应包含路径: {url}"
        );
        assert!(built.body().is_none(), "派生宏不应设置请求体");
    }

    /// 收发路径：`send_settings` 应以 JSON 数组作为请求体发送，并解码 `data: null`。
    #[tokio::test]
    async fn send_settings_posts_array_body() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let settings = vec![Setting {
            key: "site_title".to_owned(),
            value: "AList".to_owned(),
            help: String::new(),
            value_type: "string".to_owned(),
            options: String::new(),
            group: 1,
            flag: 0,
            index: 0,
        }];
        Request::new(&client, settings)
            .send_settings()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/setting/save "),
            "{}",
            recorded[0]
        );
        // 请求体必须是设置项 JSON 数组（含 `type` 键的 rename 形状）
        assert!(
            recorded[0].contains(
                r#"[{"key":"site_title","value":"AList","help":"","type":"string","options":"","group":1,"flag":0,"index":0}]"#,
            ),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("application/json"), "{}", recorded[0]);
    }
}
