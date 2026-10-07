//! admin-driver 驱动端点句柄。
//!
//! 覆盖 `/api/admin/driver` 下的存储驱动信息端点：
//! 驱动列表（含元信息）、驱动名称列表与单个驱动详情。
//! 通过 [`Admin::driver`](super::Admin::driver) 获取句柄，推荐即建即用。

pub mod info;
pub mod list;
pub mod names;

/// admin-driver 驱动句柄。
///
/// 作为驱动端点方法的命名空间路由句柄，通过 [`Admin::driver`](super::Admin::driver) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct Driver<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> Driver<'a> {
    /// 创建驱动句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取驱动信息子句柄。
    #[inline]
    #[must_use]
    pub fn driver(&self) -> Driver<'a> {
        Driver::new(self.client)
    }
}
