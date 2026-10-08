//! fs 端点：批量重命名。
//!
//! 对应 `POST /api/fs/batch_rename`；请求体为 `src_dir` 与 `rename_objects`
//! 数组（Go `BatchRenameReq`，fsbatch.go:147-153），服务端逐条尽力改写
//! （空名项被跳过），响应 `data: null`。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::RenameObject;

/// 批量重命名请求构建器。
///
/// 通过 [`Fs::batch_rename`](super::Fs::batch_rename) 创建。本端点无可选参数，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/batch_rename", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源目录（必选；`rename_objects` 中的名称均相对该目录）。
    src_dir: String,
    /// 重命名项列表（必选；每项含 `src_name`/`new_name`，
    /// 服务端跳过空名项并逐条校验新名称合法性）。
    rename_objects: Vec<RenameObject>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        src_dir: impl Into<String>,
        rename_objects: Vec<RenameObject>,
    ) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
            rename_objects,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 批量重命名。
    ///
    /// 对应 AList `POST /api/fs/batch_rename`；对 `src_dir` 下的多个文件按
    /// `rename_objects` 逐条改写，成功时响应 `data` 为 `null`。
    /// 注意服务端为逐条尽力而为：某条失败（新名称非法、路径受限等）会直接
    /// 以错误结束请求，但此前已完成的条目不会回滚。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `fs/batch_rename` 与
    /// `examples/alist/server/handles/fsbatch.go`（实现为 `fs.FsBatchRename`，
    /// 请求 `BatchRenameReq` fsbatch.go:147-153）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 源目录。
    /// * `rename_objects` - 重命名项列表，每项含 `src_name`（原文件名）与
    ///   `new_name`（新文件名）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；某条的新名称非法或路径受限时服务端返回 400/403。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::fs::RenameObject;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// client.fs().batch_rename("/m2", vec![
    ///     RenameObject { src_name: "test.txt".into(), new_name: "aaas2.txt".into() },
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn batch_rename(
        &self,
        src_dir: impl Into<String>,
        rename_objects: Vec<RenameObject>,
    ) -> Request<'a> {
        Request::new(self.client, src_dir, rename_objects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/batch_rename，请求体含 src_dir/rename_objects。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            "/m2",
            vec![RenameObject {
                src_name: "test.txt".to_owned(),
                new_name: "aaas2.txt".to_owned(),
            }],
        )
        .build_request()
        .build()
        .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/batch_rename"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/m2");
        assert_eq!(
            json["rename_objects"],
            serde_json::json!([{ "src_name": "test.txt", "new_name": "aaas2.txt" }])
        );
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

        Request::new(
            &client,
            "/m2",
            vec![RenameObject {
                src_name: "test.txt".to_owned(),
                new_name: "aaas2.txt".to_owned(),
            }],
        )
        .send()
        .await
        .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/batch_rename "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0]
                .contains(r#""rename_objects":[{"src_name":"test.txt","new_name":"aaas2.txt"}]"#),
            "{}",
            recorded[0]
        );
    }
}
