//! fs 端点：获取单个文件/目录信息。
//!
//! 对应 `POST /api/fs/get`；返回对象详情（含直链原始 URL `raw_url`、签名 `sign`、
//! 相关文件 `related` 与存储驱动 `provider`）。请求体仅 `path`/`password` 两个字段
//! （openapi 文档多列出的 `page`/`per_page`/`refresh` 在 Go 侧 `FsGetReq`
//! 中不存在，以 Go 源码为准，fsread.go:341-344）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::FsGetResponse;

/// 获取单个文件/目录信息请求构建器。
///
/// 通过 [`Fs::get`](super::Fs::get) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/get", model = FsGetResponse)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件/目录路径（必选）。
    path: String,
    /// 目录密码（可选；目录受密码保护时必填）。
    password: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            password: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 获取单个文件/目录信息。
    ///
    /// 对应 AList `POST /api/fs/get`；返回对象详情
    /// （[`FsGetResponse`]）：名称、大小、时间、类型、
    /// 签名与哈希信息等基础字段（`obj`），外加直链 `raw_url`（目录恒为空串）、
    /// 元信息 `readme`/`header`、存储驱动 `provider`、是否 Web 代理 `web_proxy`
    /// 以及同目录同前缀的相关文件 `related`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/get` 与
    /// `examples/alist/server/handles/fsread.go`（实现为 `fs.FsGet`，
    /// 请求 `FsGetReq` fsread.go:341-344，响应 `FsGetResponse` fsread.go:346-354）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标文件/目录路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`FsGetResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；目录密码错误或对象不存在时返回对应错误。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let info = client.fs().get("/t/Alist V3.md")
    ///     .password("secret") // 可选：目录密码，String 字段可直接传 &str
    ///     .await?;
    /// println!("{}（{} 字节）直链：{}", info.obj.name, info.obj.size, info.raw_url);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn get(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/get，请求体仅含 path/password。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/t/Alist V3.md")
            .password("secret".to_owned())
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/get"), "URL 应包含路径: {url}");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "path": "/t/Alist V3.md", "password": "secret" })
        );
    }

    /// 2) 请求体序列化断言：未设置的 password 应被跳过。
    #[test]
    fn build_request_skips_unset_password() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/t").build_request().build().unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json, serde_json::json!({ "path": "/t" }));
    }

    /// 3) 收发路径断言：mock 服务器返回 openapi `/api/fs/get` 示例，
    /// 断言 POST 方法与 schema 兼容解码（`related: null`、无 `web_proxy` 的老形状）。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_obj() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"name":"Alist V3.md","size":2618,"is_dir":false,"modified":"2024-05-17T16:05:36.4651534+08:00","created":"2024-05-17T16:05:29.2001008+08:00","sign":"","thumb":"","type":4,"hashinfo":"null","hash_info":null,"raw_url":"http://127.0.0.1:5244/p/local/Alist%20V3.md","readme":"","header":"","provider":"Local","related":null}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client.fs().get("/t/Alist V3.md").send().await.unwrap();
        assert_eq!(resp.obj.name, "Alist V3.md");
        assert_eq!(resp.obj.size, 2618);
        assert_eq!(resp.raw_url, "http://127.0.0.1:5244/p/local/Alist%20V3.md");
        assert_eq!(resp.provider, "Local");
        assert!(resp.related.is_empty());
        assert!(!resp.web_proxy);

        let recorded = requests.lock().unwrap();
        assert!(recorded[0].contains("POST /api/fs/get "), "{}", recorded[0]);
        assert!(
            recorded[0].contains(r#""path":"/t/Alist V3.md""#),
            "{}",
            recorded[0]
        );
    }
}
