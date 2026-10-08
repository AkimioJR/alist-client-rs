//! fs 端点：正则重命名。
//!
//! 对应 `POST /api/fs/regex_rename`；请求体为 `src_dir`/`src_name_regex`/
//! `new_name_regex`（对应 AList 服务端 `RegexRenameReq`），服务端对源目录下
//! 名称匹配正则的文件逐条以正则替换结果改名，响应 `data: null`。
//! API 路径以 AList OpenAPI 规范与 AList 服务端路由定义为准。

use alist_client_derive::EndpointRequest;

/// 正则重命名请求构建器。
///
/// 通过 [`Fs::regex_rename`](super::Fs::regex_rename) 创建。本端点无可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/regex_rename", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源目录路径（必选参数）。
    ///
    /// 服务端枚举该目录下所有条目并逐条执行正则匹配。
    src_dir: String,
    /// 源文件名匹配正则（必选参数）。
    ///
    /// 遵循 Go `regexp` 正则表达式语法，非法正则服务端返回 500。
    src_name_regex: String,
    /// 新文件名替换模板（必选参数）。
    ///
    /// 作为 `ReplaceAllString` 的替换模板，支持 `$1` 等正则分组引用。
    new_name_regex: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        src_dir: impl Into<String>,
        src_name_regex: impl Into<String>,
        new_name_regex: impl Into<String>,
    ) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
            src_name_regex: src_name_regex.into(),
            new_name_regex: new_name_regex.into(),
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 正则重命名。
    ///
    /// 对应 AList `POST /api/fs/regex_rename`；对 `src_dir` 下名称匹配
    /// `src_name_regex` 的条目，以 `new_name_regex` 作为替换模板逐条改名，
    /// 成功时响应 `data` 为 `null`。
    /// 数据来源：AList OpenAPI 规范的 `/api/fs/regex_rename` 与
    /// AList 服务端 `handles.FsRegexRename`（请求结构 `RegexRenameReq`）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 源目录。
    /// * `src_name_regex` - 源文件名匹配正则（Go `regexp` 语法）。
    /// * `new_name_regex` - 新文件名正则（替换模板，支持分组引用）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；正则非法时服务端返回 500，
    /// 替换结果名称非法时返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 把 S01E01 这类季集编号统一改为 S01E01.mp4 风格的前缀
    /// client.fs().regex_rename("/m2", r"S(\d+)E(\d+)", "S0$1E0$2").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn regex_rename(
        &self,
        src_dir: impl Into<String>,
        src_name_regex: impl Into<String>,
        new_name_regex: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, src_dir, src_name_regex, new_name_regex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/regex_rename，请求体含三个正则字段。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m2", r"S(\d+)E(\d+)", "S0$1E0$2")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/regex_rename"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/m2");
        assert_eq!(json["src_name_regex"], r"S(\d+)E(\d+)");
        assert_eq!(json["new_name_regex"], "S0$1E0$2");
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

        Request::new(&client, "/m2", "test", "demo")
            .send()
            .await
            .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/regex_rename "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""src_name_regex":"test""#),
            "{}",
            recorded[0]
        );
    }
}
