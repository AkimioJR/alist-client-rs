//! Client methods for `/api/admin/task/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::TaskIdQuery;
use crate::models::admin::task::UploadTaskInfo;
use reqwest::Method;

impl Client {
    /// Get upload task info with `/api/admin/task/upload/info`.
    pub async fn admin_upload_task_info(
        &self,
        tid: impl Into<String>,
    ) -> Result<Vec<UploadTaskInfo>> {
        let query = TaskIdQuery { tid: tid.into() };
        self.request_with_query(Method::POST, "/admin/task/upload/info", &query)
            .await
    }

    /// List completed upload tasks with `/api/admin/task/upload/done`.
    pub async fn admin_upload_task_done(&self) -> Result<Vec<UploadTaskInfo>> {
        self.request_without_body(Method::GET, "/admin/task/upload/done")
            .await
    }

    /// List unfinished upload tasks with `/api/admin/task/upload/undone`.
    pub async fn admin_upload_task_undone(&self) -> Result<Vec<UploadTaskInfo>> {
        self.request_without_body(Method::GET, "/admin/task/upload/undone")
            .await
    }

    /// Delete an upload task with `/api/admin/task/upload/delete`.
    pub async fn admin_upload_task_delete(&self, tid: impl Into<String>) -> Result<()> {
        let query = TaskIdQuery { tid: tid.into() };
        self.request_with_query(Method::POST, "/admin/task/upload/delete", &query)
            .await
    }

    /// Cancel an upload task with `/api/admin/task/upload/cancel`.
    pub async fn admin_upload_task_cancel(&self, tid: impl Into<String>) -> Result<()> {
        let query = TaskIdQuery { tid: tid.into() };
        self.request_with_query(Method::POST, "/admin/task/upload/cancel", &query)
            .await
    }

    /// Retry an upload task with `/api/admin/task/upload/retry`.
    pub async fn admin_upload_task_retry(&self, tid: impl Into<String>) -> Result<()> {
        let query = TaskIdQuery { tid: tid.into() };
        self.request_with_query(Method::POST, "/admin/task/upload/retry", &query)
            .await
    }

    /// Clear completed upload tasks with `/api/admin/task/upload/clear_done`.
    pub async fn admin_upload_task_clear_done(&self) -> Result<()> {
        self.request_without_body(Method::POST, "/admin/task/upload/clear_done")
            .await
    }

    /// Clear succeeded upload tasks with `/api/admin/task/upload/clear_succeeded`.
    pub async fn admin_upload_task_clear_succeeded(&self) -> Result<()> {
        self.request_without_body(Method::POST, "/admin/task/upload/clear_succeeded")
            .await
    }
}
