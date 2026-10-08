//! fs 端点：解压压缩包内容。
//!
//! 对应 `POST /api/fs/archive/decompress`；响应 `data` 为
//! `{task: [...]}`（后台解压任务列表，同步完成的条目不会出现在其中，
//! 全部同步完成时 `task` 为空数组）。该端点未收录于
//! `docs/api/alistv3.openapi.yaml`，请求/响应形状以
//! `examples/alist/server/handles/archive.go` 的 `ArchiveDecompressReq`
//! （archive.go:240-248）与 `FsArchiveDecompress`（archive.go:318-320，
//! `gin.H{"task": getTaskInfos(tasks)}`）为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::ArchiveDecompressResp;

/// 解压压缩包内容请求构建器。
///
/// 通过 [`Archive::decompress`](super::Archive::decompress) 创建。
/// 可选参数使用链式 setter，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/fs/archive/decompress",
    model = ArchiveDecompressResp
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 压缩包所在目录（必选）；对应 Go `ArchiveDecompressReq.SrcDir`。
    src_dir: String,
    /// 解压目标目录（必选）；对应 Go `ArchiveDecompressReq.DstDir`。
    dst_dir: String,
    /// 待解压条目在压缩包内的名称列表（必选）；对应 Go
    /// `ArchiveDecompressReq.Name`（JSON 键 `name`，服务端 `StringOrArray`
    /// 同时接受单个字符串与字符串数组，本客户端统一发送数组形状）。
    name: Vec<String>,
    /// 加密压缩包的解压密码（可选）；对应 Go `ArchiveDecompressReq.ArchivePass`。
    archive_pass: Option<String>,
    /// 压缩包内部路径（可选）；限定解压范围，缺省从压缩包根开始。
    inner_path: Option<String>,
    /// 解压前是否缓存完整压缩包（可选）；对应 Go `ArchiveDecompressReq.CacheFull`。
    cache_full: Option<bool>,
    /// 是否将解压结果放入以压缩包名命名的新目录（可选）；对应 Go
    /// `ArchiveDecompressReq.PutIntoNewDir`。
    put_into_new_dir: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(
        client: &'a crate::Client,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
        name: Vec<String>,
    ) -> Self {
        Self {
            client,
            src_dir: src_dir.into(),
            dst_dir: dst_dir.into(),
            name,
            archive_pass: None,
            inner_path: None,
            cache_full: None,
            put_into_new_dir: None,
        }
    }
}

impl<'a> super::Archive<'a> {
    /// 解压压缩包内容。
    ///
    /// 对应 AList `POST /api/fs/archive/decompress`；将 `name` 指定的
    /// 压缩包内条目解压到 `dst_dir`。驱动以后台任务方式解压时，响应
    /// `data.task` 列出创建的任务（[`TaskInfo`](crate::schema::common::TaskInfo)）；
    /// 条目被同步解压时不出现在其中，全部同步完成时 `task` 为空数组。
    /// 压缩包密码错误时 AList 以响应 `code: 202` 返回。
    ///
    /// 数据来源：`examples/alist/server/router.go:249`
    /// （`a.POST("/decompress", handles.FsArchiveDecompress)`）与
    /// `examples/alist/server/handles/archive.go:250-321`（实现
    /// `FsArchiveDecompress`）。
    ///
    /// # Arguments
    ///
    /// * `src_dir` - 压缩包所在目录。
    /// * `dst_dir` - 解压目标目录。
    /// * `name` - 待解压条目在压缩包内的名称列表（相对压缩包根，
    ///   如 `["docs/a.txt", "images"]`）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`ArchiveDecompressResp`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 如压缩包密码错误、目标路径越权）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let resp = client.fs().archive().decompress(
    ///         "/data",
    ///         "/data/extracted",
    ///         vec!["demo.zip".to_owned()],
    ///     )
    ///     .archive_pass("zip-pass".to_owned()) // 可选：压缩包密码
    ///     .put_into_new_dir(true)              // 可选：解压到同名新目录
    ///     .await?;
    /// for task in &resp.task {
    ///     println!("解压任务: {} ({})", task.name, task.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn decompress(
        &self,
        src_dir: impl Into<String>,
        dst_dir: impl Into<String>,
        name: Vec<String>,
    ) -> Request<'a> {
        Request::new(self.client, src_dir, dst_dir, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) URL/方法断言：`POST /api/fs/archive/decompress`，无查询串。
    #[test]
    fn build_request_composes_method_and_url() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data", "/data/out", vec!["a.txt".to_owned()])
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().ends_with("/api/fs/archive/decompress"),
            "URL 应为 /api/fs/archive/decompress: {}",
            built.url()
        );
        assert_eq!(built.url().query(), None, "本端点无查询参数");
    }

    /// 2) 请求体序列化断言：`name` 以数组形状发送（服务端 StringOrArray 兼容）。
    #[test]
    fn build_request_serializes_names_as_array() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            "/data",
            "/data/out",
            vec!["a.txt".to_owned(), "images".to_owned()],
        )
        .archive_pass("zip-pass".to_owned())
        .inner_path("docs".to_owned())
        .cache_full(true)
        .put_into_new_dir(true)
        .build_request()
        .build()
        .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/data");
        assert_eq!(json["dst_dir"], "/data/out");
        assert_eq!(json["name"], serde_json::json!(["a.txt", "images"]));
        assert_eq!(json["archive_pass"], "zip-pass");
        assert_eq!(json["inner_path"], "docs");
        assert_eq!(json["cache_full"], true);
        assert_eq!(json["put_into_new_dir"], true);
    }

    /// 3) 请求体序列化断言：未设置的可选字段应被跳过。
    #[test]
    fn build_request_skips_unset_options() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/data", "/data/out", vec!["a.txt".to_owned()])
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["src_dir"], "/data");
        assert_eq!(json["dst_dir"], "/data/out");
        assert_eq!(json["name"], serde_json::json!(["a.txt"]));
        assert!(
            json.get("archive_pass").is_none(),
            "未设置的可选字段应被跳过"
        );
        assert!(json.get("inner_path").is_none(), "未设置的可选字段应被跳过");
        assert!(json.get("cache_full").is_none(), "未设置的可选字段应被跳过");
        assert!(
            json.get("put_into_new_dir").is_none(),
            "未设置的可选字段应被跳过"
        );
    }

    /// 4) 收发路径断言：mock 服务器 + 记录请求原文 + 解码任务列表。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_tasks() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{
            "task":[{"id":"sdH2LbjyWRk","name":"decompress demo.zip to [/data/out](/alist)",
                "state":0,"status":"running","progress":0,"error":""}]}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = Request::new(&client, "/data", "/data/out", vec!["demo.zip".to_owned()])
            .send()
            .await
            .unwrap();
        assert_eq!(resp.task.len(), 1);
        assert_eq!(resp.task[0].id, "sdH2LbjyWRk");
        assert_eq!(resp.task[0].status, "running");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/archive/decompress "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"src_dir\":\"/data\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"dst_dir\":\"/data/out\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"name\":[\"demo.zip\"]"),
            "{}",
            recorded[0]
        );
    }

    /// 5) 兼容钉扎：服务端同步完成（`task` 为空数组）与 `task` 缺失/`null` 时均可解码。
    #[test]
    fn decodes_empty_and_missing_task_arrays() {
        let empty: ArchiveDecompressResp = serde_json::from_value(serde_json::json!({
            "task": []
        }))
        .unwrap();
        assert!(empty.task.is_empty());

        let none: ArchiveDecompressResp =
            serde_json::from_value(serde_json::json!({ "task": null })).unwrap();
        assert!(none.task.is_empty());
    }
}
