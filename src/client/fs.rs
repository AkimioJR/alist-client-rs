//! Client methods for `/api/fs/*`, excluding archive and upload endpoints.

use super::Client;
use crate::error::Result;
use crate::models::{
    DirResp, DirsReq, FsGetReq, FsGetResp, FsListReq, FsListResp, MkdirReq, MoveCopyReq, PageResp,
    RemoveEmptyDirectoryReq, RemoveReq, RenameReq, SearchReq, SearchResp, TasksResp,
};
use reqwest::Method;

impl Client {
    /// List a directory with `/api/fs/list`.
    pub async fn fs_list(&self, req: FsListReq) -> Result<FsListResp> {
        self.request_json(Method::POST, "/fs/list", Some(&req))
            .await
    }

    /// Get object metadata with `/api/fs/get`.
    pub async fn fs_get(&self, req: FsGetReq) -> Result<FsGetResp> {
        self.request_json(Method::POST, "/fs/get", Some(&req)).await
    }

    /// List child directories with `/api/fs/dirs`.
    pub async fn fs_dirs(&self, req: DirsReq) -> Result<Vec<DirResp>> {
        self.request_json(Method::POST, "/fs/dirs", Some(&req))
            .await
    }

    /// Search files with `/api/fs/search`.
    pub async fn fs_search(&self, req: SearchReq) -> Result<PageResp<SearchResp>> {
        self.request_json(Method::POST, "/fs/search", Some(&req))
            .await
    }

    /// Create a directory with `/api/fs/mkdir`.
    pub async fn mkdir(&self, path: impl Into<String>) -> Result<()> {
        let req = MkdirReq { path: path.into() };
        self.request_unit(Method::POST, "/fs/mkdir", Some(&req))
            .await
    }

    /// Rename an object with `/api/fs/rename`.
    pub async fn rename(&self, req: RenameReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/rename", Some(&req))
            .await
    }

    /// Move objects with `/api/fs/move`.
    pub async fn move_items(&self, req: MoveCopyReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/move", Some(&req))
            .await
    }

    /// Copy objects with `/api/fs/copy`.
    pub async fn copy_items(&self, req: MoveCopyReq) -> Result<Option<TasksResp>> {
        self.request_json_nullable(Method::POST, "/fs/copy", Some(&req))
            .await
    }

    /// Remove objects with `/api/fs/remove`.
    pub async fn remove(&self, req: RemoveReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/remove", Some(&req))
            .await
    }

    /// Remove empty directories recursively with `/api/fs/remove_empty_directory`.
    pub async fn remove_empty_directory(&self, src_dir: impl Into<String>) -> Result<()> {
        let req = RemoveEmptyDirectoryReq {
            src_dir: src_dir.into(),
        };
        self.request_unit(Method::POST, "/fs/remove_empty_directory", Some(&req))
            .await
    }
}
