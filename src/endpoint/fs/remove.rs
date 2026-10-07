//! fs 端点：删除文件或文件夹。
//!
//! 对应 `POST /api/fs/remove`；请求体为 `dir` 与 `names` 数组（Go `RemoveReq`，
//! fsmanage.go:290-293），服务端对每个名称逐条递归删除，响应 `data: null`。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

/// 删除文件或文件夹请求构建器。
///
/// 通过 [`Fs::remove`](super::Fs::remove) 创建。本端点无可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/remove", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待删除项所在的父目录（必选；对应 Go `RemoveReq.Dir`）。
    dir: String,
    /// 待删除的文件/目录名列表（必选，相对 `dir`；为空时服务端返回 400）。
    names: Vec<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        dir: impl Into<String>,
        names: Vec<String>,
    ) -> Self {
        Self {
            client,
            dir: dir.into(),
            names,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 删除文件或文件夹。
    ///
    /// 对应 AList `POST /api/fs/remove`；删除 `dir` 下 `names` 列出的
    /// 文件/目录（目录递归删除），成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `fs/remove` 与
    /// `examples/alist/server/handles/fsmanage.go`（实现为 `fs.FsRemove`，
    /// 请求 `RemoveReq` fsmanage.go:290-293）。
    ///
    /// # Arguments
    ///
    /// * `dir` - 待删除项所在的父目录。
    /// * `names` - 待删除的文件/目录名列表（相对 `dir`），为空时服务端返回 400。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）
    /// 时，返回 [`crate::Error`]；无删除权限时服务端返回 403。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().remove("/t", vec!["a.txt".to_owned(), "tmp".to_owned()]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn remove(&self, dir: impl Into<String>, names: Vec<String>) -> Request<'a> {
        Request::new(self.client, dir, names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/remove，请求体含 dir/names。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/t", vec!["a.txt".to_owned()])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/remove"), "URL 应包含路径: {url}");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["dir"], "/t");
        assert_eq!(json["names"], serde_json::json!(["a.txt"]));
    }

    /// 2) 收发路径断言：mock 服务器返回 `data: null`，断言请求原文与解码。
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

        Request::new(&client, "/t", vec!["a.txt".to_owned()])
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/remove "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""dir":"/t""#), "{}", recorded[0]);
    }
}
