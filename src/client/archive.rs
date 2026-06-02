//! Client methods for `/api/fs/archive/*`.

use super::Client;
use crate::error::Result;
use crate::models::{
    ArchiveDecompressReq, ArchiveListReq, ArchiveListResp, ArchiveMetaReq, ArchiveMetaResp,
};
use reqwest::Method;

impl Client {
    /// Read archive metadata with `/api/fs/archive/meta`.
    pub async fn archive_meta(&self, req: ArchiveMetaReq) -> Result<ArchiveMetaResp> {
        self.request_json(Method::POST, "/fs/archive/meta", Some(&req))
            .await
    }

    /// List an inner archive directory with `/api/fs/archive/list`.
    pub async fn archive_list(&self, req: ArchiveListReq) -> Result<ArchiveListResp> {
        self.request_json(Method::POST, "/fs/archive/list", Some(&req))
            .await
    }

    /// Decompress archive content with `/api/fs/archive/decompress`.
    pub async fn archive_decompress(&self, req: ArchiveDecompressReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/archive/decompress", Some(&req))
            .await
    }
}
