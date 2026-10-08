//! API 端点句柄模块。
//!
//! 提供按功能域划分的轻量无状态 API 路由句柄（如 [`auth::Auth`]、[`fs::Fs`]、
//! [`public::Public`]、[`admin::Admin`]），避免将大量端点方法直接平铺在 [`Client`](crate::Client) 上。
//!
//! ## 设计说明
//!
//! - 句柄本身仅持有客户端引用，不包含任何业务状态或预绑定参数，
//!   **无需也不建议单独声明变量持有**，推荐即建即用的链式调用：
//!
//!   ```no_run
//!   use alist_client::{Authentication, Client};
//!
//!   # async fn example() -> alist_client::Result<()> {
//!   let client = Client::new("https://alist.example.com")?
//!       .with_authentication(Authentication::token("TOKEN".to_owned()));
//!   // 即建即用：句柄不持有状态，直接链式调用端点方法，
//!   // 例如 `client.auth().me().await?`、`client.fs().mkdir("/new")`、`client.admin().meta()`。
//!   # Ok(())
//!   # }
//!   ```
//!
//! - 顶层句柄（`Auth`/`Fs`/`Public`/`Admin`）由 `Client` 的同名访问器创建；
//!   子句柄（如 [`admin::meta::Meta`]）由父句柄的访问器创建。
//! - 每个端点文件（如 `fs/list.rs`）定义一个 `Request` 构建器，
//!   通过 `alist-client-derive` 的 `#[derive(EndpointRequest)]` 生成
//!   `build_request`/`send`/`IntoFuture`/可选参数 setter；完整模板见 `docs/design.md`。
//! - 模块与 feature 的对应关系：`auth`/`fs`/`public` 单 feature；
//!   `admin` 聚合全部 `admin-*` 子 feature，子 feature 也可单独启用
//!   （此时 [`admin::Admin`] 句柄仍可用，但只包含已启用子域的访问器）。

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "public")]
pub mod public;

#[cfg(any(
    feature = "admin",
    feature = "admin-meta",
    feature = "admin-user",
    feature = "admin-storage",
    feature = "admin-driver",
    feature = "admin-setting",
    feature = "admin-task",
    feature = "admin-role",
    feature = "admin-label",
    feature = "admin-label-file-binding"
))]
pub mod admin;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use alist_client_derive::EndpointRequest;
    use serde::Deserialize;

    use crate::test_support::{ok_json, spawn_mock_server};

    // ---------------------------------------------------------------------------
    // 派生宏冒烟测试：在主 crate 中以真实 Client 驱动派生生成的
    // build_request / send / send_raw / IntoFuture / setter / into_stream，
    // 保证宏生成面在编译期与运行期均可用（宏 crate 自身无法依赖 reqwest）。
    // ---------------------------------------------------------------------------

    /// 带查询参数与 JSON 请求体的测试端点。
    #[derive(EndpointRequest)]
    #[endpoint(method = POST, path = "/api/test/echo", model = EchoResp)]
    #[cfg_attr(
        feature = "into-stream",
        endpoint(into_stream = true, stream_item = EchoItem)
    )]
    struct Request<'a> {
        #[endpoint(skip)]
        client: &'a crate::Client,
        /// 页码查询参数。
        #[query]
        page: Option<i32>,
        /// 目标路径（请求体必选字段）。
        path: String,
        /// 可选密码（请求体可选字段）。
        password: Option<String>,
    }

    impl<'a> Request<'a> {
        fn new(client: &'a crate::Client, path: String) -> Self {
            Self {
                client,
                page: None,
                path,
                password: None,
            }
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct EchoResp {
        content: Vec<EchoItem>,
        total: i64,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct EchoItem {
        id: u64,
    }

    /// 无请求体、响应 `data: null` 的测试端点（对应 `model = ()`）。
    #[derive(EndpointRequest)]
    #[endpoint(method = POST, path = "/api/test/noop", model = ())]
    struct NoopRequest<'a> {
        #[endpoint(skip)]
        client: &'a crate::Client,
    }

    impl<'a> NoopRequest<'a> {
        fn new(client: &'a crate::Client) -> Self {
            Self { client }
        }
    }

    #[test]
    fn build_request_composes_method_path_query_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let request = Request::new(&client, "/data".to_string()).page(2);

        let built = request.build_request().build().unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/test/echo"), "URL 应包含路径: {url}");
        assert!(url.contains("page=2"), "URL 应包含查询参数: {url}");

        let client = crate::Client::new("https://alist.example").unwrap();
        let request = Request::new(&client, "/data".to_string()).password("secret");
        let built = request.build_request().build().unwrap();
        assert!(
            !built.url().as_str().contains("password"),
            "请求体字段不应出现在查询串"
        );
    }

    #[test]
    fn build_request_skips_none_query() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let request = Request::new(&client, "/data".to_string());
        let built = request.build_request().build().unwrap();
        assert!(
            !built.url().as_str().contains("page="),
            "None 查询参数应被跳过: {}",
            built.url()
        );
    }

    #[tokio::test]
    async fn send_posts_json_body_and_decodes_response() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":1}],"total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], Some(Arc::clone(&requests))).await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_string()));

        let resp: EchoResp = Request::new(&client, "/data".to_string())
            .page(2)
            .password("secret")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.total, 1);
        assert_eq!(resp.content, vec![EchoItem { id: 1 }]);

        let recorded = requests.lock().unwrap();
        let request = &recorded[0];
        assert!(request.contains("POST /api/test/echo"), "{request}");
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: token-1"),
            "send 应触发认证头注入: {request}"
        );
        assert!(request.contains("\"path\":\"/data\""), "{request}");
        assert!(request.contains("\"password\":\"secret\""), "{request}");
    }

    #[tokio::test]
    async fn request_is_awaitable_via_into_future() {
        let body = r#"{"code":200,"message":"success","data":{"content":[{"id":7}],"total":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = crate::Client::new(base_url).unwrap();

        let resp: EchoResp = Request::new(&client, "/data".to_string())
            .page(1)
            .await
            .unwrap();
        assert_eq!(resp.content[0].id, 7);
    }

    #[tokio::test]
    async fn send_raw_decodes_to_custom_type() {
        let body = r#"{"code":200,"message":"success","data":{"content":[],"total":0}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = crate::Client::new(base_url).unwrap();

        let raw: serde_json::Value = Request::new(&client, "/data".to_string())
            .send_raw()
            .await
            .unwrap();
        assert_eq!(raw["total"], 0);
    }

    #[tokio::test]
    async fn no_body_request_sends_no_json_content_type() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(r#"{"code":200,"message":"success","data":null}"#)],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        NoopRequest::new(&client).send().await.unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/test/noop"),
            "{}",
            recorded[0]
        );
        assert!(
            !recorded[0]
                .to_ascii_lowercase()
                .contains("content-type: application/json"),
            "无请求体字段的请求不应携带 JSON Content-Type: {}",
            recorded[0]
        );
    }

    #[cfg(feature = "into-stream")]
    #[tokio::test]
    async fn into_stream_walks_pages_until_empty_content() {
        use futures::StreamExt;

        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":1},{"id":2}],"total":3}}"#,
                ),
                ok_json(
                    r#"{"code":200,"message":"success","data":{"content":[{"id":3}],"total":3}}"#,
                ),
                ok_json(r#"{"code":200,"message":"success","data":{"content":[],"total":3}}"#),
            ],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url).unwrap();

        let items: Vec<u64> = Request::new(&client, "/data".to_string())
            .into_stream()
            .map(|item| item.unwrap().id)
            .collect::<Vec<_>>()
            .await;
        assert_eq!(items, vec![1, 2, 3]);

        let recorded = requests.lock().unwrap();
        assert_eq!(recorded.len(), 3);
        assert!(recorded[0].contains("page=1"), "{}", recorded[0]);
        assert!(recorded[1].contains("page=2"), "{}", recorded[1]);
        assert!(recorded[2].contains("page=3"), "{}", recorded[2]);
    }
}
