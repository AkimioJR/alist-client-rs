//! fs 端点：聚合移动。
//!
//! 对应 `POST /api/fs/recursive_move`；请求体为 `src_dir`/`dst_dir`/
//! `conflict_policy`（Go `RecursiveMoveReq`，fsbatch.go:19-22），服务端递归枚举
//! 源目录下全部文件并逐条移动；成功时以 `SuccessWithMsgResp` 返回
//! 「Successfully moved N file(s)」消息，`data` 为 `null`，以 `()` 作为端点模型。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

pub use crate::schema::fs::ConflictPolicy;

/// 聚合移动请求构建器。
///
/// 通过 [`Fs::recursive_move`](super::Fs::recursive_move) 创建。可选参数使用链式
/// setter，直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/recursive_move", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源目录（必选；服务端递归枚举其下全部文件）。
    src_dir: String,
    /// 目标目录（必选）。
    dst_dir: String,
    /// 冲突处理策略（可选）：直接覆盖（[`ConflictPolicy::Overwrite`]）、
    /// 目标已存在时取消并返回 403（[`ConflictPolicy::Cancel`]）、
    /// 跳过已存在文件（[`ConflictPolicy::Skip`]）；缺省时非 `overwrite` 策略同样先检查目标。
    conflict_policy: Option<ConflictPolicy>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
    ) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
            dst_dir: dst_dir.into(),
            conflict_policy: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 聚合移动。
    ///
    /// 对应 AList `POST /api/fs/recursive_move`；将 `src_dir` 下（含子目录）
    /// 的全部文件移动到 `dst_dir`。与 [`Fs::move_items`](super::Fs::move_items)
    /// 不同，本端点由服务端递归枚举文件并逐条 `fs.Move`，成功时响应 `data`
    /// 为 `null`、`message` 为移动结果摘要（如 `Successfully moved 3 files`）。
    /// 数据来源：`examples/alist/server/router.go:234`（路由注册）与
    /// `examples/alist/server/handles/fsbatch.go`（实现为 `fs.FsRecursiveMove`，
    /// 请求 `RecursiveMoveReq` fsbatch.go:19-22，响应 fsbatch.go:144）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 源目录。
    /// * `dst_dir` - 目标目录。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// 可选参数（链式 setter）：`conflict_policy`
    /// （[`ConflictPolicy::Overwrite`] 直接覆盖 / [`ConflictPolicy::Cancel`] 取消 /
    /// [`ConflictPolicy::Skip`] 跳过）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；`conflict_policy = ConflictPolicy::Cancel` 且目标已存在时
    /// 服务端以 403 返回。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{
    ///     Authentication, Client,
    ///     schema::fs::ConflictPolicy,
    /// };
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().recursive_move("/m1", "/m2")
    ///     .conflict_policy(ConflictPolicy::Overwrite) // 可选：overwrite / cancel / skip
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn recursive_move(
        &self,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, src_dir, dst_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/recursive_move，请求体含冲突策略。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m1", "/m2")
            .conflict_policy(ConflictPolicy::Overwrite)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/recursive_move"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/m1");
        assert_eq!(json["dst_dir"], "/m2");
        assert_eq!(json["conflict_policy"], "overwrite");
    }

    /// 2) 请求体序列化断言：未设置的 conflict_policy 应被跳过。
    #[test]
    fn build_request_skips_unset_conflict_policy() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m1", "/m2")
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "src_dir": "/m1", "dst_dir": "/m2" })
        );
    }

    /// 3) 收发路径断言：`SuccessWithMsgResp` 的 `data` 为 `null`（fsbatch.go:144），
    /// 解码为 `()`；断言请求原文。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_null_data() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"Successfully moved 2 files","data":null}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        Request::new(&client, "/m1", "/m2").send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/recursive_move "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""src_dir":"/m1""#),
            "{}",
            recorded[0]
        );
    }
}
