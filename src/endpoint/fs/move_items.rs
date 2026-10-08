//! fs 端点：移动文件。
//!
//! 对应 `POST /api/fs/move`；方法名取 `move_items` 以避开 Rust 关键字 `move`。
//! 请求体为 `src_dir`/`dst_dir`/`names`/`overwrite`（对应 AList 服务端 `MoveCopyReq`；
//! `overwrite` 为新版服务端新增的覆盖开关），响应 `data: null`。跨存储移动会被服务端拒绝
//! （`MoveBetweenTwoStorages`）。
//! API 路径以 AList OpenAPI 规范与 AList 服务端路由定义为准。

use alist_client_derive::EndpointRequest;

/// 移动文件请求构建器。
///
/// 通过 [`Fs::move_items`](super::Fs::move_items) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/move", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源目录路径（必选参数）。
    ///
    /// 待移动文件所在的源目录。
    src_dir: String,
    /// 目标目录路径（必选参数）。
    ///
    /// 须与源目录位于同一存储，跨存储移动会被服务端拒绝。
    dst_dir: String,
    /// 待移动的文件或目录名列表（必选参数）。
    ///
    /// 各项均相对于 `src_dir`；为空时服务端返回 400。
    names: Vec<String>,
    /// 是否允许覆盖目标同名文件（可选参数）。
    ///
    /// 缺省不允许，目标同名文件已存在时服务端返回 403。
    overwrite: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
        names: Vec<String>,
    ) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
            dst_dir: dst_dir.into(),
            names,
            overwrite: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 移动文件。
    ///
    /// 对应 AList `POST /api/fs/move`（方法名取 `move_items` 以避开 Rust
    /// 关键字 `move`）；将 `src_dir` 下的 `names` 移动到 `dst_dir`，
    /// 成功时响应 `data` 为 `null`。
    /// 数据来源：AList 服务端路由定义中的 `/api/fs/move` 与
    /// AList 服务端 `handles.FsMove`（请求结构 `MoveCopyReq`）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 源目录。
    /// * `dst_dir` - 目标目录（须与源目录位于同一存储）。
    /// * `names` - 待移动的文件/目录名列表（相对 `src_dir`），为空时服务端返回 400。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；跨存储移动、目标已存在且未设置 `overwrite`
    /// 时服务端返回 403。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().move_items("/m1", "/m2", vec!["a.txt".to_owned(), "b".to_owned()])
    ///     .overwrite(false) // 可选：是否覆盖目标同名文件
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn move_items(
        &self,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
        names: Vec<String>,
    ) -> Request<'a> {
        Request::new(self.client, src_dir, dst_dir, names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/move，请求体含 src_dir/dst_dir/names。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m1", "/m2", vec!["a.txt".to_owned()])
            .overwrite(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.ends_with("/api/fs/move"),
            "URL 应为 /api/fs/move: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/m1");
        assert_eq!(json["dst_dir"], "/m2");
        assert_eq!(json["names"], serde_json::json!(["a.txt"]));
        assert_eq!(json["overwrite"], true);
    }

    /// 2) 请求体序列化断言：未设置的 overwrite 应被跳过。
    #[test]
    fn build_request_skips_unset_overwrite() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m1", "/m2", vec!["a".to_owned()])
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "src_dir": "/m1", "dst_dir": "/m2", "names": ["a"] })
        );
    }

    /// 3) 收发路径断言：mock 服务器返回 `data: null`，断言请求原文与解码。
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

        Request::new(&client, "/m1", "/m2", vec!["a.txt".to_owned()])
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/move "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""names":["a.txt"]"#),
            "{}",
            recorded[0]
        );
    }
}
