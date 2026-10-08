//! fs 端点：添加离线下载任务。
//!
//! 对应 `POST /api/fs/add_offline_download`；请求体为 `urls`/`path`/`tool`/
//! `delete_policy`（Go `AddOfflineDownloadReq`，offline_download.go:338-343）。
//! 响应是 `{"tasks": [...]}` 任务数组（每个 URL 至多一个后台任务，
//! offline_download.go:383-385，openapi 示例一致），以 [`OfflineDownloadResponse`]
//! 建模。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::OfflineDownloadResponse;

/// 添加离线下载任务请求构建器。
///
/// 通过 [`Fs::add_offline_download`](super::Fs::add_offline_download) 创建。
/// 本端点无可选参数，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/fs/add_offline_download",
    model = OfflineDownloadResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 下载目标目录（必选；须具备离线下载权限）。
    path: String,
    /// 下载链接列表（必选；每个 URL 至多产生一个后台任务）。
    urls: Vec<String>,
    /// 离线下载工具名（必选；如 `aria2`、`SimpleHttp`、`qBittorrent`，
    /// 或网盘内置工具 `115 Cloud`、`PikPak` 等）。
    tool: String,
    /// 临时文件删除策略（必选；`delete_on_upload_succeed` 上传成功后删除、
    /// `delete_on_upload_failed` 上传失败后删除、`delete_never` 从不删除、
    /// `delete_always` 总是删除，见 `internal/offline_download/tool/add.go:27-31`）。
    delete_policy: String,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；本端点无可选参数
    /// （openapi 将 `path`/`urls`/`tool`/`delete_policy` 全部标记为必填）。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        path: impl Into<String>,
        urls: Vec<String>,
        tool: impl Into<String>,
        delete_policy: impl Into<String>,
    ) -> Self {
        Self {
            client,
            path: path.into(),
            urls,
            tool: tool.into(),
            delete_policy: delete_policy.into(),
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 添加离线下载任务。
    ///
    /// 对应 AList `POST /api/fs/add_offline_download`；为 `urls` 中的每个链接
    /// 使用 `tool` 创建离线下载任务，下载到临时目录后转存到 `path`。
    /// 成功时响应 `data` 为 `{"tasks": [TaskInfo]}` 任务数组
    /// （[`OfflineDownloadResponse`]；
    /// 每个 URL 至多一个任务，单个 URL 创建失败会直接报错）。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/add_offline_download`
    /// 与 `examples/alist/server/handles/offline_download.go`（实现为
    /// `handles.AddOfflineDownload`，请求 `AddOfflineDownloadReq`
    /// offline_download.go:338-343，响应 offline_download.go:383-385）。
    ///
    /// # Arguments
    ///
    /// * `path` - 下载目标目录。
    /// * `urls` - 下载链接列表。
    /// * `tool` - 离线下载工具名（如 `aria2`、`SimpleHttp`、`qBittorrent`）。
    /// * `delete_policy` - 临时文件删除策略（`delete_on_upload_succeed` /
    ///   `delete_on_upload_failed` / `delete_never` / `delete_always`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`OfflineDownloadResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；无离线下载权限时服务端返回 403，
    /// 工具未配置或初始化失败时返回 500。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let resp = client.fs().add_offline_download(
    ///     "/local",
    ///     vec!["https://www.example.com/demo.png".to_owned()],
    ///     "SimpleHttp",
    ///     "delete_on_upload_succeed",
    /// ).await?;
    /// for task in &resp.tasks {
    ///     println!("离线下载任务：{}（{}）", task.name, task.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn add_offline_download(
        &self,
        path: impl Into<String>,
        urls: Vec<String>,
        tool: impl Into<String>,
        delete_policy: impl Into<String>,
    ) -> Request<'a> {
        Request::new(self.client, path, urls, tool, delete_policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/add_offline_download，请求体含全部必选字段。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            "/local",
            vec!["https://www.example.com/demo.png".to_owned()],
            "SimpleHttp",
            "delete_on_upload_succeed",
        )
        .build_request()
        .build()
        .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/fs/add_offline_download"),
            "URL 应包含路径: {url}"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/local");
        assert_eq!(
            json["urls"],
            serde_json::json!(["https://www.example.com/demo.png"])
        );
        assert_eq!(json["tool"], "SimpleHttp");
        assert_eq!(json["delete_policy"], "delete_on_upload_succeed");
    }

    /// 2) 收发路径断言：mock 服务器返回 openapi 示例响应，断言请求原文与任务解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_tasks() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"tasks":[{"id":"jwy7BrfZRzbI2xWg7-y","name":"download https://www.baidu.com/img/20d6cf.png to (/local)","state":0,"status":"","progress":0,"error":""}]}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client
            .fs()
            .add_offline_download(
                "/local",
                vec!["https://www.baidu.com/img/20d6cf.png".to_owned()],
                "SimpleHttp",
                "delete_on_upload_succeed",
            )
            .send()
            .await
            .unwrap();
        assert_eq!(resp.tasks.len(), 1);
        assert_eq!(resp.tasks[0].id, "jwy7BrfZRzbI2xWg7-y");
        assert_eq!(resp.tasks[0].state, 0);

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/add_offline_download "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""tool":"SimpleHttp""#),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains(r#""delete_policy":"delete_on_upload_succeed""#),
            "{}",
            recorded[0]
        );
    }
}
