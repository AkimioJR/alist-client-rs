//! admin-meta 端点：创建元信息。
//!
//! 对应 `POST /api/admin/meta/create`；请求体为完整 Meta 对象（服务端
//! `CreateMeta` 直接 `ShouldBind` 到 `model.Meta`，见 `examples/alist/server/handles/meta.go:36`），
//! 响应 `data: null`，以 `()` 作为端点模型。
//! 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/create` 与
//! `examples/alist/server/handles/meta.go`、`examples/alist/internal/model/meta.go`。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::meta::Meta;

/// 创建元信息请求构建器。
///
/// 通过 [`Meta::create`](super::Meta::create) 创建。请求体字段与 [`Meta`] 一一对应；
/// 个别字段可用链式 setter 在提交前覆盖。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/meta/create", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 元信息 ID；创建时应保持 `0`，由服务端自增分配。
    id: u64,
    /// 规则作用的目录路径（服务端要求非空，且同一路径唯一）。
    path: String,
    /// 目录密码；空字符串表示不设密码。
    password: Option<String>,
    /// 密码是否应用于子目录。
    p_sub: Option<bool>,
    /// 是否允许访客写入该目录。
    write: Option<bool>,
    /// 写权限是否应用于子目录。
    w_sub: Option<bool>,
    /// 隐藏条目的匹配规则（正则表达式，多条以 `\n` 分隔；服务端逐条校验合法性）。
    hide: Option<String>,
    /// 隐藏规则是否应用于子目录。
    h_sub: Option<bool>,
    /// 目录说明内容。
    readme: Option<String>,
    /// 说明是否应用于子目录。
    r_sub: Option<bool>,
    /// 自定义响应头（每行一条 `Header: Value`）。
    header: Option<String>,
    /// 自定义响应头是否应用于子目录。
    header_sub: Option<bool>,
}

impl<'a> Request<'a> {
    /// `Request::new` 接收客户端引用与完整 [`Meta`] 对象（即该端点的必选载荷）；
    /// 个别字段可用派生 setter 覆盖。
    #[inline]
    #[must_use = "仅构造请求构建器，不会自动发送请求"]
    pub(crate) fn new(client: &'a crate::Client, meta: Meta) -> Self {
        Self {
            client,
            id: meta.id,
            path: meta.path,
            password: Some(meta.password),
            p_sub: Some(meta.p_sub),
            write: Some(meta.write),
            w_sub: Some(meta.w_sub),
            hide: Some(meta.hide),
            h_sub: Some(meta.h_sub),
            readme: Some(meta.readme),
            r_sub: Some(meta.r_sub),
            header: Some(meta.header),
            header_sub: Some(meta.header_sub),
        }
    }
}

impl<'a> super::Meta<'a> {
    /// 创建元信息。
    ///
    /// 对应 AList `POST /api/admin/meta/create`；请求体即完整 [`Meta`] JSON
    /// （`id` 应保持 `0`，由服务端分配；`path` 必填且同一路径仅一条规则），
    /// 成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/create` 与
    /// `examples/alist/server/handles/meta.go`（实现为 `CreateMeta`；`hide` 经
    /// `validHide` 做正则校验，非法时返回 `code` 非 200 的错误响应）。
    ///
    /// # Arguments
    ///
    /// * `meta` - 待创建的元信息对象；`path` 必填，`id` 保持 `0`，其余字段按需填写
    ///   （可用 [`Meta::default`] 起步后逐字段覆盖）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200，
    /// 例如 `path` 为空、`hide` 正则非法或路径重复）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::schema::admin::meta::Meta;
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let meta = Meta {
    ///     path: "/secret".to_owned(),
    ///     password: "passw0rd".to_owned(),
    ///     p_sub: true,
    ///     ..Default::default()
    /// };
    /// client.admin().meta().create(meta).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn create(&self, meta: Meta) -> Request<'a> {
        Request::new(self.client, meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build_request().build() 断言：方法、路径与完整 Meta JSON 请求体。
    #[test]
    fn build_request_composes_method_url_and_meta_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            Meta {
                path: "/a".to_owned(),
                password: "c".to_owned(),
                ..Default::default()
            },
        )
        .build_request()
        .build()
        .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        assert!(
            built.url().as_str().contains("/api/admin/meta/create"),
            "URL 应包含路径: {}",
            built.url()
        );

        // 请求体应为平铺的完整 Meta 对象（对应 openapi create 请求示例 + Go model.Meta 全字段）
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        let body: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(
            body,
            serde_json::json!({
                "id": 0,
                "path": "/a",
                "password": "c",
                "p_sub": false,
                "write": false,
                "w_sub": false,
                "hide": "",
                "h_sub": false,
                "readme": "",
                "r_sub": false,
                "header": "",
                "header_sub": false
            })
        );
    }

    /// setter 覆盖断言：派生 setter 可在提交前改写字段。
    #[test]
    fn setters_override_meta_fields() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            Meta {
                path: "/a".to_owned(),
                ..Default::default()
            },
        )
        .password("pw".to_owned())
        .write(true)
        .hide("secret")
        .build_request()
        .build()
        .unwrap();
        let body = built.body().unwrap().as_bytes().unwrap();
        let body: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(body["password"], "pw");
        assert_eq!(body["write"], true);
        assert_eq!(body["hide"], "secret");
        assert_eq!(body["path"], "/a");
    }

    /// 收发路径断言：POST 平铺 Meta 请求体，`data: null` 解码为 `()`。
    #[tokio::test]
    async fn send_posts_meta_body_and_decodes_null_data() {
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
            Meta {
                path: "/a".to_owned(),
                ..Default::default()
            },
        )
        .send()
        .await
        .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/meta/create"),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"path\":\"/a\""), "{}", recorded[0]);
        assert!(recorded[0].contains("\"id\":0"), "{}", recorded[0]);
    }
}
