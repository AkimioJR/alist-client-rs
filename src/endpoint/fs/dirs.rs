//! fs 端点：获取目录列表。
//!
//! 对应 `POST /api/fs/dirs`；仅返回目录下的子目录（`name`/`modified`），
//! 请求体为 `path`/`password`/`force_root`（对应 AList 服务端 `DirReq`；
//! `force_root = true` 供管理员忽略根路径限制）。
//! API 路径以 AList OpenAPI 规范与 AList 服务端路由定义为准。
//!
//! # Version
//!
//! - 自 AList `v3.0.0` 起提供本接口。
//! - 从 AList `v3.6.0` 起，请求路径包含 `../` 或以 `..` 结尾时返回 403；老版本将其静默归一化拼接至用户根目录后继续处理。
//! - 从 AList `v3.6.0` 起，访问校验纳入元信息隐藏规则，命中隐藏项的路径返回 403（错误消息为 `password is incorrect or you have no permission`）；老版本仅校验元信息密码，错误消息为 `password is incorrect`。
//! - 从 AList `v3.46.0` 起，目录访问校验改用基于角色的权限合并判定，可见范围、隐藏豁免与免密访问均由用户角色的合并权限决定；老版本按用户自身权限位与元信息密码判定。
//! - 从 AList `v3.50.0` 起，响应仅包含用户角色可读的子目录（逐项按角色读取权限过滤）；老版本返回目录下的全部子目录。

use alist_client_derive::EndpointRequest;

use crate::schema::fs::DirResponse;

/// 获取目录列表请求构建器。
///
/// 通过 [`Fs::dirs`](super::Fs::dirs) 创建。可选参数使用链式 setter，
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/fs/dirs", model = Vec<DirResponse>)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标目录路径（必选参数）。
    ///
    /// 获取该目录下的子目录列表。
    path: String,
    /// 目录密码（可选参数）。
    ///
    /// 目录受密码保护时必填。
    password: Option<String>,
    /// 是否忽略用户根目录限制（可选参数）。
    ///
    /// 为 `true` 时按绝对路径获取目录，仅管理员有效。
    force_root: Option<bool>,
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
            force_root: None,
        }
    }
}

impl<'a> super::Fs<'a> {
    /// 获取目录列表。
    ///
    /// 对应 AList `POST /api/fs/dirs`；返回目录下的子目录列表
    /// （`Vec<DirResponse>`，每项含 `name` 与 `modified`），不含文件。
    /// 数据来源：AList OpenAPI 规范的 `/api/fs/dirs` 与
    /// AList 服务端 `handles.FsDirs`（请求结构 `DirReq`，响应结构 `DirResponse`）。
    ///
    /// # Arguments
    ///
    /// * `path` - 目标目录路径。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`Vec<DirResponse>`](crate::schema::fs::DirResponse)。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）
    /// 时，返回 [`crate::Error`]；非管理员请求 `force_root` 时服务端以 403 返回。
    ///
    /// # Version
    ///
    /// - 自 AList `v3.0.0` 起提供本接口。
    /// - 从 AList `v3.6.0` 起，请求路径包含 `../` 或以 `..` 结尾时返回 403；老版本将其静默归一化拼接至用户根目录后继续处理。
    /// - 从 AList `v3.6.0` 起，访问校验纳入元信息隐藏规则，命中隐藏项的路径返回 403（错误消息为 `password is incorrect or you have no permission`）；老版本仅校验元信息密码，错误消息为 `password is incorrect`。
    /// - 从 AList `v3.46.0` 起，目录访问校验改用基于角色的权限合并判定，可见范围、隐藏豁免与免密访问均由用户角色的合并权限决定；老版本按用户自身权限位与元信息密码判定。
    /// - 从 AList `v3.50.0` 起，响应仅包含用户角色可读的子目录（逐项按角色读取权限过滤）；老版本返回目录下的全部子目录。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let dirs = client.fs().dirs("/t")
    ///     .password("secret".to_owned()) // 可选参数链式 setter
    ///     .await?;
    /// for dir in &dirs {
    ///     println!("{}", dir.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn dirs(&self, path: impl Into<String>) -> Request<'a> {
        Request::new(self.client, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 纯 URL/方法断言：POST /api/fs/dirs，请求体含 path/force_root。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "/t")
            .force_root(true)
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(url.contains("/api/fs/dirs"), "URL 应包含路径: {url}");
        let body = built.body().and_then(reqwest::Body::as_bytes).unwrap();
        let json: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(json["path"], "/t");
        assert_eq!(json["force_root"], true);
        assert!(json.get("password").is_none(), "未设置的可选字段应被跳过");
    }

    /// 2) 收发路径断言：mock 服务器返回 OpenAPI `/api/fs/dirs` 示例，断言解码。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_dirs() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/api/fs/dirs")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"code":200,"message":"success","data":[{"name":"a","modified":"2023-07-19T09:48:13.695585868+08:00"}]}"#,
            )
            .create_async()
            .await;
        let client = crate::Client::new(server.url()).unwrap();

        let dirs = client.fs().dirs("/t").send().await.unwrap();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].name, "a");

        mock.assert_async().await;
    }
}
