//! admin-task 管理员任务管理端点句柄。
//!
//! 覆盖 `/api/admin/task` 下的管理员级任务管理端点：
//! 包含 7 种子类别（`upload`, `copy`, `offline_download`, `offline_download_transfer`,
//! `s3_transition`, `decompress`, `decompress_upload`），
//! 每种子类别均支持 12 种控制与查询操作：
//! 任务详情、已完成/未完成列表、删除、取消、重试、清空已完成、清空已成功、重试全部失败任务，
//! 以及批量取消、批量删除与批量重试。
//!
//! 与用户级前缀 `/api/task` 相比，管理员级接口要求管理员凭据，且可查看与操作全部用户的任务。
//! 路由处理与通用任务模块（[`crate::endpoint::task`]）完全一致，底层请求通过
//! [`TaskScope::Admin`] 作用域生成 `/api/admin/task` 路径前缀。
//!
//! 通过 [`Admin::task`](super::Admin::task) 获取句柄，推荐即建即用：
//!
//! ```no_run
//! use alist_client::{Authentication, Client};
//!
//! # async fn example() -> alist_client::Result<()> {
//! let client = Client::new("https://alist.example.com")?
//!     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
//! let undone = client.admin().task().upload().undone().await?;
//! # Ok(())
//! # }
//! ```

pub use crate::endpoint::task::{
    BatchTaskResult, Task, TaskCategory, TaskCategoryHandle, TaskInfo, TaskInfoList, TaskScope,
};

impl<'a> super::Admin<'a> {
    /// 获取管理员级任务管理子句柄。
    ///
    /// 返回用于管理后台任务的 [`Task`] 句柄，固定使用管理员作用域 [`TaskScope::Admin`]，
    /// 实际发起的请求路径前缀为 `/api/admin/task`。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alist_client::{Authentication, Client};
    ///
    /// # async fn example() -> alist_client::Result<()> {
    /// let client = Client::new("https://alist.example.com")?
    ///     .with_authentication(Authentication::token("ADMIN_TOKEN".to_owned()));
    /// let undone = client.admin().task().upload().undone().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn task(&self) -> Task<'a> {
        Task::with_scope(self.client, TaskScope::Admin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证管理员任务句柄的作用域为 [`TaskScope::Admin`]。
    #[test]
    fn admin_task_scope_is_admin() {
        let client = crate::Client::new("https://alist.example").unwrap();
        assert_eq!(client.admin().task().scope(), TaskScope::Admin);
    }

    /// 验证请求构建器的请求方法与 URL 路径。
    #[test]
    fn admin_task_request_building() {
        let client = crate::Client::new("https://alist.example").unwrap();
        let undone_req = client
            .admin()
            .task()
            .upload()
            .undone()
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*undone_req.method(), reqwest::Method::GET);
        assert_eq!(
            undone_req.url().as_str(),
            "https://alist.example/api/admin/task/upload/undone"
        );

        let info_req = client
            .admin()
            .task()
            .copy()
            .info("tid")
            .build_request()
            .build()
            .unwrap();
        assert_eq!(*info_req.method(), reqwest::Method::POST);
        assert_eq!(
            info_req.url().as_str(),
            "https://alist.example/api/admin/task/copy/info?tid=tid"
        );
    }

    /// 验证 `client.admin().task().upload().undone()` 发送 `GET /api/admin/task/upload/undone`。
    #[tokio::test]
    async fn admin_task_upload_undone_sends_get_request() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":[]}"#;
        let mock = server
            .mock("GET", "/api/admin/task/upload/undone")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        let undone = client.admin().task().upload().undone().await.unwrap();
        assert!(undone.is_empty());

        mock.assert_async().await;
    }

    /// 验证 `client.admin().task().copy().info("tid")` 发送 `POST /api/admin/task/copy/info?tid=tid`。
    #[tokio::test]
    async fn admin_task_copy_info_sends_post_request() {
        let mut server = mockito::Server::new_async().await;
        let body = r#"{"code":200,"message":"success","data":{"id":"tid","name":"copy file","state":2,"status":"running","progress":50.0,"error":""}}"#;
        let mock = server
            .mock("POST", "/api/admin/task/copy/info?tid=tid")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create_async()
            .await;
        let client = crate::Client::new(server.url())
            .unwrap()
            .with_authentication(crate::Authentication::Token("admin-token".to_owned()));

        let info = client.admin().task().copy().info("tid").await.unwrap();
        assert_eq!(info.id, "tid");
        assert_eq!(info.name, "copy file");

        mock.assert_async().await;
    }
}
