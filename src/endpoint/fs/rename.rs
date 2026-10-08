//! fs 端点：重命名文件。
//!
//! 对应 `POST /api/fs/rename`；请求体为 `path`/`name`/`overwrite`
//! （对应 AList 服务端 `RenameReq`；`overwrite` 为新版服务端新增的
//! 覆盖开关），响应 `data: null`。
//! API 路径以 AList OpenAPI 规范与 AList 服务端路由定义为准。

use alist_client_derive::EndpointRequest;

/// 重命名文件请求构建器。
///
/// 通过 [`Fs::rename`](super::Fs::rename) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/rename", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源文件或目录完整路径（必选参数）。
    ///
    /// 指向需要被重命名的目标。
    path: String,
    /// 目标文件名（必选参数）。
    ///
    /// 不支持 `/`，服务端会执行名称合法性校验。
    name: String,
    /// 是否允许覆盖同名目标（可选参数）。
    ///
    /// 缺省不允许，目标已存在时服务端返回 403。
    overwrite: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        path: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            path: path.into(),
            name: name.into(),
            overwrite: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 重命名文件。
    ///
    /// 对应 AList `POST /api/fs/rename`；将 `path` 指向的文件/目录改名为
    /// `name`，成功时响应 `data` 为 `null`。
    /// 数据来源：AList OpenAPI 规范的 `/api/fs/rename` 与
    /// AList 服务端 `handles.FsRename`（请求结构 `RenameReq`）。
    ///
    /// # Arguments
    ///
    /// * `path` - 源文件/目录完整路径。
    /// * `name` - 目标文件名（不含目录，不支持 `/`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；目标已存在且未设置 `overwrite` 时服务端以 403 返回。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().rename("/阿里云盘/test2", "test3")
    ///     .overwrite(true) // 可选：允许覆盖同名目标
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn rename(&self, path: impl Into<String>, name: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/rename，请求体含 path/name/overwrite。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/阿里云盘/test2", "test3")
            .overwrite(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/rename"), "URL 应包含路径: {url}");
        assert!(
            !url.contains("batch_rename") && !url.contains("regex_rename"),
            "URL 不应误匹配其他重命名端点: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/阿里云盘/test2");
        assert_eq!(json["name"], "test3");
        assert_eq!(json["overwrite"], true);
    }

    /// 2) 请求体序列化断言：未设置的 overwrite 应被跳过。
    #[test]
    fn build_request_skips_unset_overwrite() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/a", "b")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json, serde_json::json!({ "path": "/a", "name": "b" }));
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

        Request::new(&client, "/a/test2", "test3")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/rename "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""name":"test3""#), "{}", recorded[0]);
    }
}
