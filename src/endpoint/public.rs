//! public 公共端点句柄。
//!
//! 覆盖无需认证的站点设置、连通性检测、离线下载工具与压缩包扩展名端点，
//! API 路径见 AList OpenAPI 规范的 `public` 分组与服务端路由定义。
//!
//! 通过 [`Client::public`](crate::Client::public) 获取句柄，推荐即建即用。

pub mod archive_extensions;
pub mod offline_download_tools;
pub mod ping;
pub mod settings;

/// public 公共句柄。
///
/// 作为公共端点方法的命名空间路由句柄，通过 [`Client::public`](crate::Client::public) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Public<'a> {
    client: &'a crate::Client,
}

impl crate::Client {
    /// 获取 public 公共句柄。
    ///
    /// 返回用于访问公共端点的 [`Public`] 句柄，推荐直接链式调用端点方法。
    #[inline]
    #[must_use]
    pub fn public(&self) -> Public<'_> {
        Public { client: self }
    }
}
