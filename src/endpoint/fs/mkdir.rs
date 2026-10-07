//! fs 端点：新建文件夹。
//!
//! 对应 `POST /api/fs/mkdir`；请求体仅 `path` 一个字段（Go `MkdirOrLinkReq`
// 只声明 `Path`，fsmanage.go:25-27；openapi 请求示例同样只有 `path`），
//! 响应 `data: null`，以 `()` 作为端点模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

/// 新建文件夹请求构建器。
///
/// 通过 [`Fs::mkdir`](super::Fs::mkdir) 创建。本端点无可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/mkdir", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 新目录完整路径（必选；父目录必须已存在）。
    path: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 新建文件夹。
    ///
    /// 对应 AList `POST /api/fs/mkdir`；成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `fs/mkdir` 与
    /// `examples/alist/server/handles/fsmanage.go`（实现为 `fs.FsMkdir`，
    /// 请求 `MkdirOrLinkReq` fsmanage.go:25-27）。
    ///
    /// # Arguments
    ///
    /// * `path` - 新目录路径（相对于某存储的完整路径，父目录必须已存在）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）
    /// 时，返回 [`crate::Error`]；无写权限或目录已存在时返回对应错误。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().mkdir("/new-dir").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn mkdir(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/mkdir，请求体仅含 path。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/tt")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/mkdir"), "URL 应包含路径: {url}");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json, serde_json::json!({ "path": "/tt" }));
    }

    /// 2) 收发路径断言：mock 服务器返回 `data: null`，解码为 `()`。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "/tt").send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/mkdir "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""path":"/tt""#), "{}", recorded[0]);
    }
}
