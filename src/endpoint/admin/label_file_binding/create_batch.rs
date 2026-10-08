//! admin-label-file-binding 端点：批量创建标签绑定。
//!
//! 对应 `POST /api/admin/label_file_binding/create_batch`；请求体为
//! `{"items":[...]}`（每项形状见
//! [`CreateItem`]），
//! 响应 `data` 为 `{"total":...,"succeed":...,"failed":...,"results":[...]}`，
//! 以 [`CreateBatchResp`]
//! 作为端点模型。该分组未收录进 openapi 文档；路由见
//! `examples/alist/server/router.go:212`，处理逻辑见
//! `examples/alist/server/handles/label_file_binding.go:208`
//! （实现为 `handles.CreateLabelFileBinDingBatch`；空 `items` 会被服务端拒绝，返回 400）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::label_file_binding::{CreateBatchResp, CreateItem};

/// 批量创建标签绑定请求构建器。
///
/// 通过 [`LabelFileBinding::create_batch`](super::LabelFileBinding::create_batch) 创建。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/label_file_binding/create_batch",
    model = CreateBatchResp
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 批量创建项（必选；服务端要求 `items` 非空）。
    ///
    /// 单项处理为逐条尽力而为：某项失败（如为目录创建绑定）不会中断其余项，
    /// 结果以 [`CreateBatchResp::results`](crate::schema::admin::label_file_binding::CreateBatchResp)
    /// 逐项给出。
    items: Vec<CreateItem>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, items: Vec<CreateItem>) -> Self {
        Self { client, items }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 批量创建标签绑定。
    ///
    /// 对应 AList `POST /api/admin/label_file_binding/create_batch`；成功时响应
    /// `data` 为总数/成功数/失败数与逐项结果
    /// （[`CreateBatchResp`]）。
    /// 数据来源：`examples/alist/server/router.go:212`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:208`
    /// （实现为 `handles.CreateLabelFileBinDingBatch`）。
    ///
    /// # Arguments
    ///
    /// * `items` - 批量创建项列表；服务端要求非空，单项失败不影响其余项。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`CreateBatchResp`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]；`items` 为空时服务端返回 400。
    /// 注意：单项失败（如为目录创建绑定）不算端点错误，体现在返回值的 `failed`/`results` 中。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::label_file_binding::CreateItem;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let resp = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .create_batch(vec![
    ///         CreateItem {
    ///             name: Some("movie.mp4".to_owned()),
    ///             label_ids: Some("1,2".to_owned()),
    ///             ..Default::default()
    ///         },
    ///         CreateItem {
    ///             name: Some("song.flac".to_owned()),
    ///             label_ids: Some("3".to_owned()),
    ///             ..Default::default()
    ///         },
    ///     ])
    ///     .await?;
    /// println!("成功 {} / 共 {}", resp.succeed, resp.total);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn create_batch(&self, items: Vec<CreateItem>) -> Request<'a> {
        Request::new(self.client, items)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状断言：方法/URL 与 `items` 数组请求体。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            vec![
                CreateItem {
                    name: Some("movie.mp4".to_owned()),
                    label_ids: Some("1,2".to_owned()),
                    ..Default::default()
                },
                CreateItem {
                    name: Some("song.flac".to_owned()),
                    size: Some(2048),
                    ..Default::default()
                },
            ],
        )
        .build_request()
        .build()
        .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label_file_binding/create_batch"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains('?'),
            "本端点无查询参数，URL 不应携带查询串: {url}"
        );

        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"items\":["), "{body}");
        assert!(body.contains("\"name\":\"movie.mp4\""), "{body}");
        assert!(body.contains("\"label_ids\":\"1,2\""), "{body}");
        assert!(body.contains("\"name\":\"song.flac\""), "{body}");
        assert!(body.contains("\"size\":2048"), "{body}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码响应内批量结果。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_batch_result() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"total":2,"succeed":1,"failed":1,"results":[{"name":"movie.mp4","ok":true},{"name":"folder","ok":false,"errMsg":"Unable to bind folder"}]}}"#,
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
                CreateItem {
                    name: Some("movie.mp4".to_owned()),
                    label_ids: Some("1,2".to_owned()),
                    ..Default::default()
                },
                CreateItem {
                    name: Some("folder".to_owned()),
                    is_dir: Some(true),
                    ..Default::default()
                },
            ],
        )
        .send()
        .await
        .unwrap();
        assert_eq!(resp.total, 2);
        assert_eq!(resp.succeed, 1);
        assert_eq!(resp.failed, 1);
        assert!(!resp.results[1].ok);
        assert_eq!(
            resp.results[1].err_msg.as_deref(),
            Some("Unable to bind folder")
        );

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label_file_binding/create_batch "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"items\":["), "{}", recorded[0]);
        assert!(recorded[0].contains("\"is_dir\":true"), "{}", recorded[0]);
    }
}
