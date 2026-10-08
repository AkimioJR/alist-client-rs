//! admin-label-file-binding 端点：创建标签绑定。
//!
//! 对应 `POST /api/admin/label_file_binding/create`；成功时响应 `data` 为
//! `{"msg":"添加成功！"}`，以
//! [`CreateResponse`] 作为端点模型。
//! 该分组未收录进 openapi 文档；路由见 `examples/alist/server/router.go:211`，
//! 处理逻辑见 `examples/alist/server/handles/label_file_binding.go`
//! （实现为 `handles.CreateLabelFileBinDing`，请求体为
//! `examples/alist/internal/op/label_file_binding.go:13` 的 `op.CreateLabelFileBinDingReq`）。

use alist_client_derive::EndpointRequest;
use chrono::{DateTime, Utc};

use crate::schema::admin::label_file_binding::CreateResponse;

/// 创建标签绑定请求构建器。
///
/// 通过 [`LabelFileBinding::create`](super::LabelFileBinding::create) 创建。
/// 可选参数使用链式 setter，直接 `.await` 执行强类型解码，
/// 或 [`.send().await`](Request::send) / [`.send_raw::<T>().await`](Request::send_raw)
/// 自定义解码类型。
#[derive(EndpointRequest)]
#[endpoint(
    method = POST,
    path = "/api/admin/label_file_binding/create",
    model = CreateResponse
)]
#[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
pub struct Request<'a> {
    #[endpoint(skip)]
    client: &'a crate::Client,
    /// 目标文件名（必选）。
    ///
    /// 服务端以文件名定位绑定：创建前会先删除该文件的全部既有绑定，
    /// 再按标签列表重建（见 `op.CreateLabelFileBinDing`）。
    name: String,
    /// 文件 ID（可选；对应 Go `model.ObjFile.Id`）。
    id: Option<String>,
    /// 文件完整路径（可选）。
    path: Option<String>,
    /// 文件大小（字节，可选）。
    size: Option<i64>,
    /// 是否为目录（可选）。
    ///
    /// 服务端拒绝为目录创建绑定：返回 400「Unable to bind folder」。
    is_dir: Option<bool>,
    /// 文件修改时间（可选）。
    modified: Option<DateTime<Utc>>,
    /// 文件创建时间（可选）。
    created: Option<DateTime<Utc>>,
    /// 签名字符串（可选；启用签名保护的非空）。
    sign: Option<String>,
    /// 缩略图链接（可选）。
    thumb: Option<String>,
    /// 文件类型枚举值（可选；0 未指定、1 目录、2 视频、3 音频、4 文本、5 图片，
    /// 见 `examples/alist/internal/conf/const.go:165`）。
    r#type: Option<i32>,
    /// 哈希信息字符串（可选；JSON 键为 `hashinfo`，对应 Go `HashInfoStr`）。
    hashinfo: Option<String>,
    /// 逗号分隔的标签 ID 列表（可选，如 `"1,2,3"`）。
    ///
    /// Go 侧请求体同时存在数组形态的 `labelIdList` 键（`LabelIDs []uint64`），
    /// 两种形态服务端解析行为一致，本构建器统一使用字符串形态。
    /// 不设置或为空时，服务端仅清除该文件的既有绑定、不新建（即「清空标签」语义）。
    label_ids: Option<String>,
}

impl<'a> Request<'a> {
    /// `Request::new` 只接收**必选**参数；可选参数一律走派生 setter。
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub(crate) fn new(client: &'a crate::Client, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
            id: None,
            path: None,
            size: None,
            is_dir: None,
            modified: None,
            created: None,
            sign: None,
            thumb: None,
            r#type: None,
            hashinfo: None,
            label_ids: None,
        }
    }
}

impl<'a> super::LabelFileBinding<'a> {
    /// 创建标签绑定。
    ///
    /// 对应 AList `POST /api/admin/label_file_binding/create`；成功时响应 `data`
    /// 为 `{"msg":"添加成功！"}`。
    /// 数据来源：`examples/alist/server/router.go:211`（路由注册）与
    /// `examples/alist/server/handles/label_file_binding.go:58`
    /// （实现为 `handles.CreateLabelFileBinDing`）。
    ///
    /// # Arguments
    ///
    /// * `name` - 目标文件名（服务端以文件名定位绑定，创建前会先清除该文件的既有标签绑定）。
    ///
    /// # Returns
    ///
    /// 返回 [`Request`] 请求构建器；可直接 `.await`，成功时返回
    /// [`CreateResponse`]。
    ///
    /// # Errors
    ///
    /// 当网络请求失败或 AList 返回非成功状态码（HTTP 非 2xx 或响应 `code` 非 200）时，
    /// 返回 [`crate::Error`]。例如通过 `is_dir(true)` 为目录创建绑定时，
    /// 服务端返回 400「Unable to bind folder」。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("TOKEN".to_owned()));
    /// let resp = client
    ///     .admin()
    ///     .label_file_binding()
    ///     .create("movie.mp4")
    ///     .label_ids("1,2") // 可选：绑定的标签 ID，逗号分隔
    ///     .size(1024)       // 可选：文件大小等元信息
    ///     .await?;
    /// println!("{}", resp.msg);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use = "请求构建器不会自动发送请求，请调用 `.await` 或 `.send().await`"]
    pub fn create(&self, name: impl Into<String>) -> Request<'a> {
        Request::new(self.client, name)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::test_support::{ok_json, spawn_mock_server};

    /// 请求形状断言：方法/URL/查询串与 JSON 请求体键名。
    #[test]
    fn build_request_composes_method_url_and_body() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let built = Request::new(&client, "movie.mp4")
            .label_ids("1,2")
            .size(1024)
            .r#type(2)
            .hashinfo("sha1:abcdef")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*built.method(), reqwest::Method::POST);
        let url = built.url().as_str();
        assert!(
            url.contains("/api/admin/label_file_binding/create"),
            "URL 应包含路径: {url}"
        );
        assert!(
            !url.contains('?'),
            "本端点无查询参数，URL 不应携带查询串: {url}"
        );

        let body = built.body().unwrap().as_bytes().unwrap();
        let body = std::str::from_utf8(body).unwrap();
        assert!(body.contains("\"name\":\"movie.mp4\""), "{body}");
        assert!(body.contains("\"label_ids\":\"1,2\""), "{body}");
        assert!(body.contains("\"size\":1024"), "{body}");
        assert!(body.contains("\"type\":2"), "{body}"); // r#type → JSON 键 "type"
        assert!(body.contains("\"hashinfo\":\"sha1:abcdef\""), "{body}");
        // 未设置的 Option 字段应被跳过
        assert!(!body.contains("is_dir"), "{body}");
        assert!(!body.contains("modified"), "{body}");
    }

    /// 收发路径断言：mock 服务器 + 记录请求原文，并解码响应内 `data`。
    #[tokio::test]
    async fn send_posts_expected_request_and_decodes_msg() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let base_url = spawn_mock_server(
            vec![ok_json(
                r#"{"code":200,"message":"success","data":{"msg":"添加成功！"}}"#,
            )],
            Some(Arc::clone(&requests)),
        )
        .await;
        let client = crate::Client::new(base_url)
            .unwrap()
            .with_authentication(crate::Authentication::Token("token-1".to_owned()));

        let resp = Request::new(&client, "movie.mp4")
            .label_ids("1,2")
            .send()
            .await
            .unwrap();
        assert_eq!(resp.msg, "添加成功！");

        let recorded = requests.lock().unwrap();
        assert!(
            recorded[0].contains("POST /api/admin/label_file_binding/create "),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"name\":\"movie.mp4\""),
            "{}",
            recorded[0]
        );
        assert!(
            recorded[0].contains("\"label_ids\":\"1,2\""),
            "{}",
            recorded[0]
        );
    }
}
