//! admin-setting 端点：配置并重启 FRP。
//!
//! 对应 `POST /api/admin/setting/set_frp`；请求体为设置项 **JSON 数组**（而非对象包装），
//! 响应 `data` 为 FRP 客户端状态字符串，以 `String` 作为端点模型。
//!
//! 由于 [`EndpointRequest`] 派生宏生成的请求体
//! 固定为对象包装结构，本端点将载荷字段标记为 `#[endpoint(skip)]`，发送时经
//! [`Request::send_settings`] 手动附加数组请求体（与 `save` 端点同一处理方式）；
//! **请勿使用派生生成的 `send`/`.await`**——它们不携带请求体，服务端会因绑定空请求体而报错。
//!
//! # Version
//!
//! - 自 AList `v3.60.0` 起提供本接口。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::setting::Setting;

/// 配置并重启 FRP 请求构建器。
///
/// 通过 [`Setting::set_frp`](super::Setting::set_frp) 创建；
/// **发送请使用 [`Request::send_settings`]**（请求体为设置项 JSON 数组）。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/setting/set_frp", model = String)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.send_settings().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待保存的 FRP 设置项数组（必选参数）。
    ///
    /// 通常包含 `frp.server_addr`、`frp.server_port` 等键，经
    /// [`Request::send_settings`] 以 JSON 数组附加到请求体。
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
    /// 见 AList 服务端 handles.Setting 模块的 `SetFRP`）；保存后服务端会重启 FRP
    /// 客户端，成功时响应 `data` 为当前 FRP 状态字符串（`running`、`stopped` 或
    /// `error: <错误信息>`；FRP 重启失败时响应仍为成功，状态串中携带错误信息）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    pub async fn send_settings(self) -> crate::Result<String> {
        let builder = self.build_request().json(&self.settings);
        self.client.execute(builder).await
    }
}

impl<'a> super::Setting<'a> {
    /// 配置并重启 FRP 内网穿透。
    ///
    /// 对应 AList `POST /api/admin/setting/set_frp`；保存设置项数组后服务端会
    /// 重启 FRP 客户端，响应 `data` 为重启后的 FRP 状态字符串。
    /// 数据来源：AList 服务端 handles.Setting 模块（实现为 `SetFRP`）。
    ///
    /// # Arguments
    ///
    /// * `settings` - 待保存的 FRP 设置项数组；通常先经 [`Setting::list`](super::Setting::list)
    ///   筛选 FRP 分组（[`SettingGroup::Frp`](crate::schema::admin::setting::SettingGroup::Frp)）
    ///   后修改 `value` 字段再整体回存。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；**发送请调用
    /// [`.send_settings().await`](Request::send_settings)**，成功时返回 FRP 状态 `String`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.60.0` 起提供本接口。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// // 读取全部设置并修改 FRP 服务地址后回存（示意）
    /// let frp_settings = client
    ///     .admin()
    ///     .setting()
    ///     .list()
    ///     .await?;
    /// let status = client.admin().setting().set_frp(frp_settings).send_settings().await?;
    /// println!("FRP 状态: {status}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn set_frp(&self, settings: Vec<Setting>) -> Request<'a> {
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
            url.contains("/api/admin/setting/set_frp"),
            "URL 应包含路径: {url}"
        );
        assert!(built.body().is_none(), "派生宏不应设置请求体");
    }

    /// 收发路径：`send_settings` 应以 JSON 数组作为请求体发送，并解码状态字符串。
    #[tokio::test]
    async fn send_settings_posts_array_body_and_returns_status() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":"running"}"#;
        let mock = server
            .mock("POST", "/api/admin/setting/set_frp")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        use crate::schema::admin::setting::{SettingFlag, SettingGroup};

        let settings = vec![Setting {
            key: "frp.server_addr".to_owned(),
            value: "frp.example.com".to_owned(),
            help: String::new(),
            r#type: "string".to_owned(),
            options: String::new(),
            group: SettingGroup::Frp,
            flag: SettingFlag::Private,
            index: Some(0),
        }];
        let status = Request::new(&client, settings)
            .send_settings()
            .await
            .unwrap();
        assert_eq!(status, "running");

        mock.assert_async().await;
    }
}
