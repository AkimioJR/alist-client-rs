//! Client methods for `/api/fs/*`, excluding archive and upload endpoints.

pub mod archive;
pub mod upload;

use super::Client;
use crate::error::Result;
use crate::models::common::PageResp;
use crate::models::fs::{
    BatchRenameReq, DirResp, DirsReq, FsGetReq, FsGetResp, FsListReq, FsListResp, MkdirReq,
    MoveCopyReq, RegexRenameReq, RemoveEmptyDirectoryReq, RemoveReq, RenameReq, SearchReq,
    SearchResp, TasksResp,
};
use reqwest::Method;

impl Client {
    /// List a directory with `/api/fs/list`.
    pub async fn fs_list(&self, req: FsListReq) -> Result<FsListResp> {
        self.request(Method::POST, "/fs/list", Some(&req), false)
            .await
    }

    /// Get object metadata with `/api/fs/get`.
    pub async fn fs_get(&self, req: FsGetReq) -> Result<FsGetResp> {
        self.request(Method::POST, "/fs/get", Some(&req), false)
            .await
    }

    /// List child directories with `/api/fs/dirs`.
    pub async fn fs_dirs(&self, req: DirsReq) -> Result<Vec<DirResp>> {
        self.request(Method::POST, "/fs/dirs", Some(&req), false)
            .await
    }

    /// Search files with `/api/fs/search`.
    pub async fn fs_search(&self, req: SearchReq) -> Result<PageResp<SearchResp>> {
        self.request(Method::POST, "/fs/search", Some(&req), false)
            .await
    }

    /// Create a directory with `/api/fs/mkdir`.
    pub async fn mkdir(&self, path: impl Into<String>) -> Result<()> {
        let req = MkdirReq { path: path.into() };
        self.request(Method::POST, "/fs/mkdir", Some(&req), false)
            .await
    }

    /// Rename an object with `/api/fs/rename`.
    pub async fn rename(&self, req: RenameReq) -> Result<()> {
        self.request(Method::POST, "/fs/rename", Some(&req), false)
            .await
    }

    /// Rename multiple objects with `/api/fs/batch_rename`.
    pub async fn batch_rename(&self, req: BatchRenameReq) -> Result<()> {
        self.request(Method::POST, "/fs/batch_rename", Some(&req), false)
            .await
    }

    /// Rename objects by regular expression with `/api/fs/regex_rename`.
    pub async fn regex_rename(&self, req: RegexRenameReq) -> Result<()> {
        self.request(Method::POST, "/fs/regex_rename", Some(&req), false)
            .await
    }

    /// Move objects with `/api/fs/move`.
    pub async fn move_items(&self, req: MoveCopyReq) -> Result<()> {
        self.request(Method::POST, "/fs/move", Some(&req), false)
            .await
    }

    /// Copy objects with `/api/fs/copy`.
    pub async fn copy_items(&self, req: MoveCopyReq) -> Result<Option<TasksResp>> {
        self.request(Method::POST, "/fs/copy", Some(&req), false)
            .await
    }

    /// Remove objects with `/api/fs/remove`.
    pub async fn remove(&self, req: RemoveReq) -> Result<()> {
        self.request(Method::POST, "/fs/remove", Some(&req), false)
            .await
    }

    /// Remove empty directories recursively with `/api/fs/remove_empty_directory`.
    pub async fn remove_empty_directory(&self, src_dir: impl Into<String>) -> Result<()> {
        let req = RemoveEmptyDirectoryReq {
            src_dir: src_dir.into(),
        };
        self.request(
            Method::POST,
            "/fs/remove_empty_directory",
            Some(&req),
            false,
        )
        .await
    }
}
