//! fs 端点：删除空文件夹。
//!
//! 对应 `POST /api/fs/remove_empty_directory`；请求体为 `src_dir`
//! （Go `RemoveEmptyDirectoryReq`，fsmanage.go:336-338），服务端广度优先递归
//! 清理该目录下的空目录（删除子级后会回查父级），响应 `data: null`。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

/// 删除空文件夹请求构建器。
///
/// 通过 [`Fs::remove_empty_directory`](super::Fs::remove_empty_directory) 创建。
/// 本端点无可选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/remove_empty_directory", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 起始目录（必选；服务端递归清理其下的空目录，不处理文件）。
    src_dir: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, src_dir: impl Into<String>) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 删除空文件夹。
    ///
    /// 对应 AList `POST /api/fs/remove_empty_directory`；递归删除 `src_dir`
    /// 下的全部空目录（仅目录，文件不受影响），成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `fs/remove_empty_directory` 与
    /// `examples/alist/server/handles/fsmanage.go`（实现为 `fs.FsRemoveEmptyDirectory`，
    /// 请求 `RemoveEmptyDirectoryReq` fsmanage.go:336-338）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 起始目录。
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
    /// client.fs().remove_empty_directory("/t").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn remove_empty_directory(&self, src_dir: impl Into<String>) -> Request<'a> {
        Request::new(self.client, src_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/remove_empty_directory，请求体含 src_dir。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/t").build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/remove_empty_directory"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json, serde_json::json!({ "src_dir": "/t" }));
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

        Request::new(&client, "/t").send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/remove_empty_directory "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""src_dir":"/t""#), "{}", recorded[0]);
    }
}
