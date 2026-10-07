//! admin-meta 端点：更新元信息。
//!
//! 对应 `POST /api/admin/meta/update`；请求体为完整 Meta 对象（服务端
//! `UpdateMeta` 直接 `ShouldBind` 到 `model.Meta`，见 `examples/alist/server/handles/meta.go:54`），
//! 响应 `data: null`，以 `()` 作为端点模型。
//! 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/update` 与
//! `examples/alist/server/handles/meta.go`、`examples/alist/internal/db/meta.go`（`db.Save` 全量覆盖）。

use alist_client_derive::EndpointRequest;

use crate::schema::admin::meta::Meta;

/// 更新元信息请求构建器。
///
/// 通过 [`Meta::update`](super::Meta::update) 创建。请求体字段与 [`Meta`] 一一对应；
/// 个别字段可用链式 setter 在提交前覆盖。
/// 直接 `.await` 执行强类型解码，或 [`.send().await`](Request::send) /
/// [`.send_raw::<T>().await`](Request::send_raw) 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(method = POST, path = "/api/admin/meta/update", model = ())]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 待更新元信息的 ID；服务端以其为主键定位记录。
    id: u64,
    /// 规则作用的目录路径（服务端要求非空）。
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
    /// `Request::new` 接收客户端引用与完整 [`Meta`] 对象（`id` 标识待更新记录，
    /// 为必选载荷）；个别字段可用派生 setter 覆盖。
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
    /// 更新元信息。
    ///
    /// 对应 AList `POST /api/admin/meta/update`；请求体即完整 [`Meta`] JSON，
    /// 服务端（`op.UpdateMeta` → `db.Save`）按 `id` **全量覆盖**整条记录——
    /// 因此推荐先 [`get`](super::Meta::get) 取回完整对象、修改需要的字段后整体提交，
    /// 直接以零值对象提交会把未填写的字段清空。成功时响应 `data` 为 `null`。
    /// 数据来源：`docs/api/alistv3.openapi.yaml` 的 `/api/admin/meta/update` 与
    /// `examples/alist/server/handles/meta.go`（实现为 `UpdateMeta`；`hide` 经
    /// `validHide` 做正则校验）、`examples/alist/internal/db/meta.go`（`db.Save`）。
    ///
    /// # Arguments
    ///
    /// * `meta` - 待更新的元信息对象；`id` 必须为已存在条目的 ID（来自
    ///   [`list`](super::Meta::list)/[`get`](super::Meta::get)），`path` 必填。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回 `()`。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或信封 `code` 非 200，
    /// 例如 ID 不存在、`hide` 正则非法）时，返回 [`crate::Error`]。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// // 先取回完整对象，再修改需要变更的字段
    /// let mut meta = client.admin().meta().get(1).await?;
    /// meta.password = "new-passw0rd".to_owned();
    /// client.admin().meta().update(meta).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "该方法仅返回请求构建器，不会自动发送请求"]
    pub fn update(&self, meta: Meta) -> Request<'a> {
        Request::new(self.client, meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// build_request().build() 断言：方法、路径与携带 `id` 的完整 Meta JSON 请求体。
    #[test]
    fn build_request_composes_method_url_and_meta_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(
            &client,
            Meta {
                id: 1,
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
            built.url().as_str().contains("/api/admin/meta/update"),
            "URL 应包含路径: {}",
            built.url()
        );

        // 请求体应为平铺的完整 Meta 对象（对应 openapi update 请求示例 + Go model.Meta 全字段）
        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        let body: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(
            body,
            serde_json::json!({
                "id": 1,
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
                id: 7,
                path: "/b".to_owned(),
                ..Default::default()
            },
        )
        .send()
        .await
        .unwrap();

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].starts_with("POST /api/admin/meta/update"),
            "{}",
            recorded[0]
        );
        assert!(recorded[0].contains("\"id\":7"), "{}", recorded[0]);
        assert!(recorded[0].contains("\"path\":\"/b\""), "{}", recorded[0]);
    }
}
