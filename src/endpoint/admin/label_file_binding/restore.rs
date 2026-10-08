//! admin-label-file-binding 端点：恢复标签绑定记录。
//!
//! 对应 `POST /api/admin/label_file_binding/restore`；请求体为
//! `{"keep_ids":bool,"override":bool,"bindings":[LabelFileBinding]}`，
//! 响应 `data` 为 `{"msg":"restored N rows"}`，以
//! [`RestoreResp`]
//! 作为端点模型。该分组未收录进 openapi 文档；路由见
//! `examples/alist/server/router.go:214`，处理逻辑见
//! `examples/alist/server/handles/label_file_binding.go:172`
//! （实现为 `handles.RestoreLabelFileBinding`，请求体为文件内
//! `restoreLabelBindingsReq{keep_ids, override, bindings}`，handler:26-30）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label_file_binding::{LabelFileBinding, RestoreResp};

/// 恢复标签绑定记录请求构建器。
///
/// 通过 [`LabelFileBinding::restore`](super::LabelFileBinding::restore) 创建。
/// 可选参数使用链式 setter，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/label_file_binding/restore",
    model = RestoreResp
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 要恢复的绑定记录列表（必选；服务端要求 `bindings` 非空）。
    ///
    /// 每条记录的 `user_id` 为 0 时服务端以当前认证用户填充；
    /// `label_id` 为 0 或 `file_name` 为空的记录会被服务端整体拒绝（返回 400）。
    bindings: Vec<LabelFileBinding>,
    /// 是否沿用记录中的绑定 ID（可选，缺省 `false`；对应 Go `KeepIDs`）。
    keep_ids: Option<bool>,
    /// 是否覆盖既有同名绑定（可选，缺省 `false`；对应 Go `Override`，
    /// JSON 键为 `override`，Rust 中为保留字故写作 `r#override`）。
    r#override: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, bindings: Vec<LabelFileBinding>) -> Self {
        Self {
            client,
            bindings,
            keep_ids: None,
            r#override: None,
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 恢复（批量导入）标签绑定记录。
    ///
    /// 对应 AList `POST /api/admin/label_file_binding/restore`；成功时响应 `data`
    /// 为 `{"msg":"restored N rows"}`（N 为提交的记录条数）。
    /// 数据来源：`examples/alist/server/router.go:214`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:172`
    /// （实现为 `handles.RestoreLabelFileBinding`）。
    ///
    /// # Arguments
    ///
    /// * `bindings` - 要恢复的绑定记录列表；服务端要求非空。记录中 `user_id`
    ///   为 0 时以当前认证用户填充；`label_id` 为 0 或 `file_name` 为空时
    ///   服务端返回 400。
    /// * `keep_ids` - 可选：是否沿用记录中的绑定 ID（缺省 `false`）。
    /// * `r#override` - 可选：是否覆盖既有同名绑定（缺省 `false`；JSON 键为
    ///   `override`，Rust 中为保留字故写作 `r#override`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`RestoreResp`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`bindings` 为空时服务端返回 400「empty bindings」，
    /// 记录缺少有效 `label_id`/`file_name` 时返回 400。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::label_file_binding::LabelFileBinding;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let binding = LabelFileBinding {
    ///     id: 7,
    ///     user_id: 0, // 0 表示由服务端以当前认证用户填充
    ///     label_id: 3,
    ///     file_name: "movie.mp4".to_owned(),
    ///     create_time: chrono::Utc::now(),
    /// };
    /// let resp = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .restore(vec![binding])
    ///     .keep_ids(true)      // 可选：沿用记录中的绑定 ID
    ///     .r#override(true)    // 可选：覆盖既有同名绑定（JSON 键为 "override"）
    ///     .await?;
    /// println!("{}", resp.msg);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn restore(&self, bindings: Vec<LabelFileBinding>) -> Request<'a> {
        Request::new(self.client, bindings)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::{DateTime, Utc};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 构造一条绑定记录（按 `model.LabelFileBinding` 形状）。
    fn sample_binding(file_name: &str, label_id: u64) -> LabelFileBinding {
        LabelFileBinding {
            id: 7,
            user_id: 0, // 服务端以当前认证用户填充
            label_id,
            file_name: file_name.to_owned(),
            create_time: DateTime::parse_from_rfc3339("2024-06-01T08:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        }
    }

    /// 请求形状断言：方法/URL 与 JSON 请求体键名（含 `override` 保留字键）。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, vec![sample_binding("movie.mp4", 3)])
            .keep_ids(true)
            .r#override(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label_file_binding/restore"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains('?'),
            "本端点无查询参数，URL 不应携带查询串: {url}"
        );

        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"keep_ids\":true"), "{body}");
        // r#override 字段按 Go restoreLabelBindingsReq.Override 的 JSON 键 "override" 序列化
        assert!(body.contains("\"override\":true"), "{body}");
        assert!(body.contains("\"bindings\":["), "{body}");
        assert!(body.contains("\"file_name\":\"movie.mp4\""), "{body}");
        assert!(body.contains("\"label_id\":3"), "{body}");
        assert!(body.contains("\"user_id\":0"), "{body}");
    }

    /// 请求形状断言：可选参数缺省时请求体不含 `keep_ids`/`override` 键。
    #[test]
    fn build_request_skips_absent_optional_flags() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, vec![sample_binding("movie.mp4", 3)])
            .build_request()
            .build()
            .unwrap();
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(!body.contains("keep_ids"), "{body}");
        assert!(!body.contains("\"override\""), "{body}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码响应内恢复结果。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_msg() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"msg":"restored 2 rows"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let resp = Request::new(
            &client,
            vec![
                sample_binding("movie.mp4", 3),
                sample_binding("song.flac", 4),
            ],
        )
        .keep_ids(true)
        .send()
        .await
        .unwrap();
        assert_eq!(resp.msg, "restored 2 rows");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label_file_binding/restore "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"bindings\":["), "{}", recorded[0]);
        assert!(recorded[0].contains("\"keep_ids\":true"), "{}", recorded[0]);
        assert!(
            recorded[0].contains("\"file_name\":\"movie.mp4\""),
            "{}",
            recorded[0]
        );
    }
}
