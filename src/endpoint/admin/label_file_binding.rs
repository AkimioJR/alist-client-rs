//! admin-label-file-binding 标签绑定端点句柄。
//!
//! 覆盖 `/api/label_file_binding` 下的文件-标签绑定端点：
//! 创建、批量创建、按文件名查询、按标签查询文件与删除绑定。
//! 路由定义见 `examples/alist/server/router.go` 的 `_labelFileBinding`
//! 与 `admin/label_file_binding` 路由组
//! （openapi 文档未覆盖该分组，以 Go 源码为准）。
//! 通过 [`Admin::label_file_binding`](super::Admin::label_file_binding) 获取句柄，推荐即建即用。

pub mod create;
pub mod create_batch;
pub mod delete;
pub mod get;
pub mod get_file_by_label;

/// admin-label-file-binding 标签绑定句柄。
///
/// 作为标签绑定端点方法的命名空间路由句柄，
/// 通过 [`Admin::label_file_binding`](super::Admin::label_file_binding) 获取。
/// 本身不包含业务状态，无需单独声明变量持有，推荐通过链式调用直接使用。
pub struct LabelFileBinding<'a> {
    // 端点文件实现后（此处读取 client 字段）应移除该 expect。
    #[expect(dead_code)]
    client: &'a crate::Client,
}

impl<'a> LabelFileBinding<'a> {
    /// 创建标签绑定句柄。
    #[inline]
    pub(crate) fn new(client: &'a crate::Client) -> Self {
        Self { client }
    }
}

impl<'a> super::Admin<'a> {
    /// 获取标签绑定管理子句柄。
    #[inline]
    #[must_use]
    pub fn label_file_binding(&self) -> LabelFileBinding<'a> {
        LabelFileBinding::new(self.client)
    }
}
