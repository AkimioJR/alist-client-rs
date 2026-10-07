//! public端点：获取站点设置。
//!
//! 对应 `GET /api/public/settings`；响应信封 `data` 为字符串键值映射
//! （服务端 `op.GetPublicSettingsMap()` 以 `map[string]string` 返回全部公共设置项，
//! 所有值均为字符串形式），端点模型为 [`PublicSettings`]。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::public::PublicSettings;

/// 获取站点设置请求构建器。
///
/// 通过 [`Public::settings`](super::Public::settings) 创建。本端点无业务参数
/// （也无可选参数），直接 `.await` 执行强类型解码，
/// 或 [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = GET, path = "/api/public/settings", model = PublicSettings)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无必选业务参数，也无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Public<'a> {
    /// 获取站点设置。
    ///
    /// 对应 AList `GET /api/public/settings`；无需认证，返回站点全部公共设置的
    /// 字符串键值映射（布尔值是 `"true"`/`"false"`，数字是 `"30"` 这样的十进制文本）。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `public/settings`、
    /// `examples/alist/server/router.go:102-103`
    /// （`public.Any("/settings", handles.PublicSettings)`，读端点按 GET 处理）与
    /// `examples/alist/server/handles/setting.go:223-225`（`op.GetPublicSettingsMap()`）。
    ///
    /// # Arguments
    ///
    /// 无参数；本端点不接收任何业务参数，请求不含查询串与请求体。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`PublicSettings`]（`HashMap<String, String>`）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）时，
    /// 返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::Client;
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?;
    /// let settings = client.public().settings().await?;
    /// println!("站点标题: {}", settings["site_title"]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn settings(&self) -> Request<'a> {
        Request::new(self.client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) URL/方法断言：`GET /api/public/settings`，无查询串、无请求体。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client).build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::GET);
        assert!(
            built.url().as_str().ends_with("/api/public/settings"),
            "URL 应为 /api/public/settings: {}",
            built.url()
        );
        assert_eq!(built.url().query(), None, "本端点无查询参数");
        assert!(built.body().is_none(), "本端点无请求体");
    }

    /// 2) 收发路径断言：mock 服务器 + 记录请求原文 + 解码设置映射。
    #[tokio::test]
    async fn send_gets_settings_map() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{"site_title":"AList","default_page_size":"30"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let settings = Request::new(&client).send().await.unwrap();
        assert_eq!(
            settings.get("site_title").map(String::as_str),
            Some("AList")
        );
        assert_eq!(
            settings.get("default_page_size").map(String::as_str),
            Some("30")
        );

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("GET /api/public/settings "),
            "{}",
            recorded[0]
        );
    }
}
