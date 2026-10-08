//! fs 端点：复制文件。
//!
//! 对应 `POST /api/fs/copy`；请求体为 `src_dir`/`dst_dir`/`names`/`overwrite`/
//! `skip_existing`（对应 AList 服务端 `MoveCopyReq`，`skip_existing` 仅对复制
//! 生效）。新版服务端返回 `{"tasks": [...]}` 后台任务信息，
//! 老版本成功时 `data` 为 `null`，因此端点模型为 `Option<CopyResponse>`。
//! API 路径以 AList OpenAPI 规范与 AList 服务端路由定义为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::CopyResponse;

/// 复制文件请求构建器。
///
/// 通过 [`Fs::copy`](super::Fs::copy) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/copy", model = Option<CopyResponse>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 源目录路径（必选参数）。
    ///
    /// 待复制文件所在的源目录。
    src_dir: String,
    /// 目标目录路径（必选参数）。
    ///
    /// 复制目标目录。
    dst_dir: String,
    /// 待复制的文件或目录名列表（必选参数）。
    ///
    /// 各项均相对于 `src_dir`；为空时服务端返回 400。
    names: Vec<String>,
    /// 是否允许覆盖目标同名文件（可选参数）。
    ///
    /// 缺省不允许，目标同名文件已存在时服务端返回 403。
    overwrite: Option<bool>,
    /// 是否跳过目标已存在且大小相同的文件（可选参数）。
    ///
    /// 仅对复制操作生效；启用后不再返回 403，改为静默跳过。
    skip_existing: Option<bool>,
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
            skip_existing: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 复制文件。
    ///
    /// 对应 AList `POST /api/fs/copy`；将 `src_dir` 下的 `names` 复制到
    /// `dst_dir`。新版服务端为每个需要后台处理的条目创建复制任务，响应 `data`
    /// 为 `{"tasks": [TaskInfo]}`；全部同步完成或老版本
    /// 服务端成功时 `data` 为 `null`，因此端点模型为 `Option<CopyResponse>`。
    /// 数据来源：AList 服务端路由定义中的 `/api/fs/copy` 与
    /// AList 服务端 `handles.FsCopy`（请求结构 `MoveCopyReq`）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 源目录。
    /// * `dst_dir` - 目标目录。
    /// * `names` - 待复制的文件/目录名列表（相对 `src_dir`），为空时服务端返回 400。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// `Option<`[`CopyResponse`]`>`：
    /// `Some` 携带后台复制任务列表（新版服务端），`None` 表示 `data` 为 `null`
    /// （无后台任务或老版本服务端）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；目标已存在且未设置 `overwrite`/`skip_existing`
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
    /// let resp = client.fs().copy("/m1", "/m2", vec!["a.txt".to_owned()])
    ///     .skip_existing(true) // 可选：跳过目标已存在且大小相同的文件
    ///     .await?;
    /// if let Some(tasks) = resp {
    ///     for task in &tasks.tasks {
    ///         println!("后台复制任务：{}", task.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn copy(
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

    /// 1) 纯 URL/方法断言：POST /api/fs/copy，请求体含复制专用字段。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/m1", "/m2", vec!["a.txt".to_owned()])
            .overwrite(true)
            .skip_existing(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/copy"), "URL 应包含路径: {url}");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/m1");
        assert_eq!(json["dst_dir"], "/m2");
        assert_eq!(json["names"], serde_json::json!(["a.txt"]));
        assert_eq!(json["overwrite"], true);
        assert_eq!(json["skip_existing"], true);
    }

    /// 2) 请求体序列化断言：未设置的可选字段应被跳过。
    #[test]
    fn build_request_skips_unset_options() {
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

    /// 3) 收发路径断言：新版服务端返回 `{"tasks": [...]}`，解码为 `Some(CopyResponse)`。
    #[tokio::test]
    async fn send_decodes_copy_tasks() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"tasks":[{"id":"abc","name":"copy [/m1](/a.txt) to [/m2](/m2/a.txt)","state":0,"status":"","progress":0,"error":""}]}}"#,
            )],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client
            .fs()
            .copy("/m1", "/m2", vec!["a.txt".to_owned()])
            .send()
            .await
            .unwrap();
        let tasks = resp.expect("新版服务端应返回任务信息");
        assert_eq!(tasks.tasks.len(), 1);
        assert_eq!(tasks.tasks[0].id, "abc");
    }

    /// 4) 收发路径断言：老版本服务端 `data: null`，解码为 `None`。
    #[tokio::test]
    async fn send_decodes_null_data_as_none() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client
            .fs()
            .copy("/m1", "/m2", vec!["a.txt".to_owned()])
            .send()
            .await
            .unwrap();
        assert_eq!(resp, None, "data: null 应解码为 None");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/copy "),
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
