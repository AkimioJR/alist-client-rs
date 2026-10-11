//! fs 端点：获取特殊文件的附加信息。
//!
//! 对应 `POST /api/fs/other`（AList v3 OpenAPI 规范未收录该端点；
//! OpenList v4 规范记载为 POST + JSON 请求体。路由为 `g.Any`，服务端以
//! `ShouldBind` 绑定 JSON 请求体；本组同为 `g.Any` 读端点的 list/get/dirs/
//! search 亦均按 POST + JSON 请求体实现，且仅 JSON 请求体能承载 `data`
//! 字段的结构化载荷——服务端 GET 表单绑定无法绑定 Go `interface{}` 字段，
//! 故采用 POST）。响应 `data` 为存储驱动返回的任意 JSON——AList 服务端
//! `fs.Other` 直接透传驱动实现（Go `interface{}`），具体形状由存储驱动与
//! `method` 参数共同决定（如阿里云盘系列 `doc_preview`/`video_preview`
//! 返回的预览信息对象、S3/OSS `archive`/`thaw` 返回的过渡任务信息），
//! 因此本端点模型为 [`serde_json::Value`]，由调用方按驱动语义解读。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.54.0` 起，S3/OSS 存储对 `archive`/`thaw` 方法改为返回
//!   后台过渡任务信息（`task_id`）；更早版本不支持这两个方法。

use alist_client_derive::EndpointRequest;

/// 获取特殊文件附加信息请求构建器。
///
/// 通过 [`Fs::other`](super::Fs::other) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/other", model = serde_json::Value)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件路径（必选参数）。
    ///
    /// 相对于某存储挂载点的完整路径。
    path: String,
    /// 附加操作方法名（必选参数）。
    ///
    /// 由存储驱动定义的操作名（如阿里云盘系列的 `doc_preview`/`video_preview`、
    /// S3/OSS 的 `archive`/`thaw`）；驱动不支持时服务端返回错误。
    method: String,
    /// 目标路径的目录密码（可选参数）。
    ///
    /// 路径受元信息密码保护时必填，错误时服务端返回 403。
    password: Option<String>,
    /// 附加操作的载荷数据（可选参数）。
    ///
    /// 任意 JSON 值，随 JSON 请求体原样传递（服务端以 Go `interface{}`
    /// 接收）；语义由存储驱动与 `method` 决定，多数操作无需提供，
    /// 个别驱动方法以对象载荷传参。
    data: Option<serde_json::Value>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        path: impl Into<String>,
        method: impl Into<String>,
    ) -> Self {
        Self {
            client,
            path: path.into(),
            method: method.into(),
            password: None,
            data: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 获取特殊文件的附加信息。
    ///
    /// 对应 AList `POST /api/fs/other`；响应 `data` 为存储驱动返回的任意 JSON
    /// （AList 服务端 `fs.Other` 透传驱动实现，形状由驱动与 `method` 决定），
    /// 以 [`serde_json::Value`] 承载。数据来源：AList 服务端路由定义与
    /// fsread 模块的 `FsOther` 处理器（该端点不在 AList v3 OpenAPI 规范中，
    /// 以 AList 服务端实现为准）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标文件路径。
    /// * `method` - 附加操作方法名（由存储驱动定义，如 `video_preview`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回驱动相关的
    /// [`serde_json::Value`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code`
    /// 非 200，例如驱动不支持该方法）时，返回 [`crate::Error`]。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.54.0` 起，S3/OSS 存储对 `archive`/`thaw` 方法改为返回
    ///   后台过渡任务信息（`task_id`）；更早版本不支持这两个方法。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let info = client.fs().other("/aliyun/movie.mp4", "video_preview").await?;
    /// println!("{info}");
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn other(&self, path: impl Into<String>, method: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path, method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 请求形状：POST 方法、路径与 JSON 请求体（必选与可选参数齐全时）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/aliyun/movie.mp4", "video_preview")
            .password("secret")
            .data(serde_json::json!({ "quality": "1080" }))
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/other"), "URL 应包含路径: {url}");
        assert!(
            built.url().query().is_none(),
            "本端点参数在 JSON 请求体，不应有查询串: {url}"
        );
        let body = built.body().unwrap().as_bytes().unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "path": "/aliyun/movie.mp4",
                "method": "video_preview",
                "password": "secret",
                "data": { "quality": "1080" }
            }),
            "JSON 请求体应包含 path/method/password/data"
        );
    }

    /// 请求形状：可选参数缺省时不进入 JSON 请求体。
    #[test]
    fn build_request_skips_absent_optional_body_fields() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/file.txt", "doc_preview")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().unwrap().as_bytes().unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "path": "/file.txt", "method": "doc_preview" }),
            "缺省的 password/data 不应出现在请求体"
        );
    }

    /// 收发路径：mock 返回 S3/OSS 过渡任务形状的驱动 JSON，断言请求行、请求体与 `Value` 解码。
    #[tokio::test]
    async fn send_decodes_driver_specific_value() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        // 示例形状取自 AList 服务端 fs.Other 的 S3/OSS 过渡任务返回
        // （Go map[string]string，键为 task_id）
        let body = r#"{"code":200,"message":"success","data":{"task_id":"s3-transition-1"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let data = Request::new(&client, "/oss/file.rar", "archive")
            .send()
            .await
            .unwrap();
        assert_eq!(data["task_id"], "s3-transition-1");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/fs/other "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#"{"path":"/oss/file.rar","method":"archive"}"#),
            "{}",
            recorded[0]
        );
    }

    /// 收发路径：驱动无返回时 `data` 为 `null`，解码为 [`serde_json::Value::Null`]。
    #[tokio::test]
    async fn send_tolerates_null_data() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let data = Request::new(&client, "/file.txt", "doc_preview")
            .send()
            .await
            .unwrap();
        assert_eq!(data, serde_json::Value::Null);
    }
}
