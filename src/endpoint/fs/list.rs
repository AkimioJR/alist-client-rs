//! fs 端点：列出目录内容。
//!
//! 对应 `POST /api/fs/list`；分页参数 `page`/`per_page` 位于 JSON 请求体
//! （`per_page = -1` 表示返回全部条目，`0` 或缺省由服务端回退为默认页大小
//! `DefaultPerPage = 200`，超过 `MaxPerPage = 500` 会被收敛为 500，
//! 见 `examples/alist/server/handles/fsread.go` 的 `normalizeListPage`，
//! fsread.go:84-88、253-269），响应含 content/total/readme/header/write/provider，
//! 新版服务端还返回 page/per_page/has_more/pages_total/filtered_total
//! （schema 层以 `#[serde(default)]` + `null_to_default` 兼容新旧版本）。
//! 端点文件模板与命名约定见 `docs/design.md`；
//! API 路径以 `docs/api/alistv3.openapi.yaml` 与 `examples/alist/server/router.go` 为准。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::FsListResp;

/// 列出目录内容请求构建器。
///
/// 通过 [`Fs::list`](super::Fs::list) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/list", model = FsListResp)]
#[cfg_attr(
    feature = "into-stream",
    endpoint(
        into_stream = true,
        stream_item = crate::schema::fs::ObjLabelResp
    )
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标目录路径（必选；相对于某存储的完整路径）。
    path: String,
    /// 目录密码（可选；目录受密码保护时必填）。
    password: Option<String>,
    /// 页码（可选，从 1 开始；缺省或非正数由服务端回退为第 1 页）。
    ///
    /// 启用 `into-stream` feature 时，[`Request::into_stream`] 会自动从当前页
    /// （缺省第 1 页）起逐页递增翻页。
    page: Option<i32>,
    /// 每页条数（可选）。
    ///
    /// `-1` 表示返回全部条目（Go `AllPerPage`）；`0` 或缺省由服务端回退为
    /// 默认页大小 200；大于 500 时服务端收敛为 500。
    ///
    /// 注意：`per_page = -1` 时服务端对任意页码都返回完整列表，
    /// 请勿与 [`Request::into_stream`] 搭配使用（流不会终止）。
    per_page: Option<i32>,
    /// 是否强制刷新存储缓存（可选；无写权限时刷新请求被服务端以 403 拒绝）。
    refresh: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            password: None,
            page: None,
            per_page: None,
            refresh: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 列出目录内容。
    ///
    /// 对应 AList `POST /api/fs/list`；返回目录下的文件/目录条目、分页总数、
    /// 元信息（readme/header）、写权限与存储驱动名
    /// （[`FsListResp`](crate::schema::fs::FsListResp)）。新版服务端额外返回
    /// `page`/`per_page`/`has_more`/`pages_total`/`filtered_total` 分页元信息，
    /// 老版本缺失时对应字段归约为零值。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/fs/list` 与
    /// `examples/alist/server/handles/fsread.go`（实现为 `fs.FsList`，
    /// 请求 `ListReq` fsread.go:22-27，响应 `FsListResp` fsread.go:52-64）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标目录路径（相对于某存储的完整路径）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`FsListResp`](crate::schema::fs::FsListResp)。
    ///
    /// 可选参数（链式 setter）：`password`（目录密码）、`page`（页码，从 1 开始）、
    /// `per_page`（每页条数，`-1` 表示全部）、`refresh`（强制刷新）。
    /// 启用 `into-stream` feature 时可调用 [`Request::into_stream`] 获得
    /// 自动翻页的条目流：从当前页（缺省第 1 页）逐页请求，服务端翻页耗尽后
    /// `content` 为空时终止（对应新版 `has_more = false`、旧版 `total` 计数用尽）。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200）
    /// 时，返回 [`crate::Error`]；目录密码错误或无权限时服务端以 403 返回。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 第 1 页、每页 20 条
    /// let page = client.fs().list("/t").page(1).per_page(20).await?;
    /// for obj in &page.content {
    ///     println!("{}（{} 字节）", obj.name, obj.size);
    /// }
    /// println!("共 {} 条", page.total);
    /// // per_page = -1 一次取回全部
    /// let all = client.fs().list("/t").per_page(-1).await?;
    /// assert_eq!(all.content.len() as i64, all.total);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn list(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/list，分页参数进入 JSON 请求体。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/dir")
            .password("secret".to_owned())
            .page(2)
            .per_page(30)
            .refresh(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/list"), "URL 应包含路径: {url}");
        assert!(
            built.url().query().is_none(),
            "分页参数应位于请求体而非查询串"
        );
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/dir");
        assert_eq!(json["password"], "secret");
        assert_eq!(json["page"], 2);
        assert_eq!(json["per_page"], 30);
        assert_eq!(json["refresh"], true);
    }

    /// 2) 请求体序列化断言：未设置的可选字段应被跳过（服务端按零值回退）。
    #[test]
    fn build_request_skips_unset_options() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/dir")
            .per_page(-1) // -1 表示全部
            .build_request()
            .build()
            .unwrap();
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/dir");
        assert_eq!(json["per_page"], -1);
        assert!(json.get("password").is_none(), "未设置的可选字段应被跳过");
        assert!(json.get("page").is_none(), "未设置的可选字段应被跳过");
        assert!(json.get("refresh").is_none(), "未设置的可选字段应被跳过");
    }

    /// 3) 收发路径断言：mock 服务器返回 openapi `/api/fs/list` 示例
    /// （老版本形状），断言 POST 方法、认证头注入与 schema 兼容解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_list() {
        use std::sync::{Arc, Mutex};

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"content":[{"name":"Alist V3.md","size":1592,"is_dir":false,"modified":"2024-05-17T13:47:55.4174917+08:00","created":"2024-05-17T13:47:47.5725906+08:00","sign":"","thumb":"","type":4,"hashinfo":"null","hash_info":null}],"total":1,"readme":"","header":"","write":true,"provider":"Local"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let resp = client.fs().list("/t").per_page(10).send().await.unwrap();
        assert_eq!(resp.total, 1);
        assert_eq!(resp.content[0].name, "Alist V3.md");
        assert_eq!(resp.content[0].r#type, 4);
        assert!(resp.write);
        assert_eq!(resp.provider, "Local");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/fs/list "),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains(r#""path":"/t""#), "{}", recorded[0]);
        assert!(recorded[0].contains(r#""per_page":10"#), "{}", recorded[0]);
        assert!(
            recorded[0].contains("authorization: token-1"),
            "应注入认证头: {}",
            recorded[0]
        );
    }

    /// 4) 收发路径断言：新版服务端返回分页元信息（filtered_total/has_more 等）
    /// 与对象新增字段时可解码。
    #[tokio::test]
    async fn send_decodes_new_server_pagination_fields() {
        use crate::test_support::{ok_json, spawn_mock_server};

        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"content":[{"id":"local-1","path":"/data/a.txt","virtual_path":"/local/a.txt","name":"a.txt","size":12,"is_dir":false,"modified":"2024-05-17T13:47:55.4174917+08:00","created":"2024-05-17T13:47:47.5725906+08:00","sign":"","thumb":"","type":4,"hashinfo":"sha1:abc","hash_info":{"sha1":"abc"},"label_list":[],"storage_class":"STANDARD"}],"total":1,"filtered_total":1,"page":1,"per_page":200,"has_more":false,"pages_total":1,"readme":"","header":"","write":true,"provider":"Local"}}"#,
            )],
            None,
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let resp = client.fs().list("/local").send().await.unwrap();
        assert_eq!(resp.filtered_total, 1);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.per_page, 200);
        assert!(!resp.has_more);
        assert_eq!(resp.pages_total, 1);
        assert_eq!(resp.content[0].id, "local-1");
        assert_eq!(resp.content[0].storage_class.as_deref(), Some("STANDARD"));
    }

    /// 5) into-stream 翻页断言：请求体 `page` 从 1 逐页递增，服务端翻页耗尽
    /// 返回空 `content`（新版 `has_more = false` 之后的确认页）时停止。
    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use std::sync::{Arc, Mutex};

        use futures::StreamExt;

        use crate::test_support::{ok_json, spawn_mock_server};

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"name":"a.txt","size":1,"is_dir":false,"modified":"2024-05-17T13:47:55.4174917+08:00","sign":"","thumb":"","type":4}],"total":1,"readme":"","header":"","write":true,"provider":"Local","has_more":false}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":null,"total":1,"readme":"","header":"","write":true,"provider":"Local"}}"#,
                ),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let names: Vec<String> = Request::new(&client, "/t")
            .per_page(1)
            .into_stream()
            .map(|item| item.unwrap().name)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(names, vec!["a.txt".to_owned()]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 2, "空 content 页会多一次确认请求");
        assert!(recorded[0].contains(r#""page":1"#), "{}", recorded[0]);
        assert!(recorded[1].contains(r#""page":2"#), "{}", recorded[1]);
    }
}
