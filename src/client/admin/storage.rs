//! Client methods for `/api/admin/storage/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::storage::{Storage, StorageCreateResp, StorageReq};
use crate::models::admin::{AdminPageQuery, IdQuery};
use crate::models::common::PageResp;
use reqwest::Method;

impl Client {
    /// Create a storage with `/api/admin/storage/create`.
    pub async fn admin_storage_create(&self, req: StorageReq) -> Result<StorageCreateResp> {
        self.request(Method::POST, "/admin/storage/create", Some(&req), false)
            .await
    }

    /// Update a storage with `/api/admin/storage/update`.
    pub async fn admin_storage_update(&self, req: StorageReq) -> Result<StorageCreateResp> {
        self.request(Method::POST, "/admin/storage/update", Some(&req), false)
            .await
    }

    /// List storages with `/api/admin/storage/list`.
    pub async fn admin_storage_list(&self, query: AdminPageQuery) -> Result<PageResp<Storage>> {
        self.request_with_query(Method::GET, "/admin/storage/list", &query)
            .await
    }

    /// Enable a storage with `/api/admin/storage/enable`.
    pub async fn admin_storage_enable(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/storage/enable", &IdQuery { id })
            .await
    }

    /// Disable a storage with `/api/admin/storage/disable`.
    pub async fn admin_storage_disable(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/storage/disable", &IdQuery { id })
            .await
    }

    /// Get a storage with `/api/admin/storage/get`.
    pub async fn admin_storage_get(&self, id: u64) -> Result<Storage> {
        self.request_with_query(Method::GET, "/admin/storage/get", &IdQuery { id })
            .await
    }

    /// Delete a storage with `/api/admin/storage/delete`.
    pub async fn admin_storage_delete(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/storage/delete", &IdQuery { id })
            .await
    }

    /// Reload all storages with `/api/admin/storage/load_all`.
    pub async fn admin_storage_load_all(&self) -> Result<()> {
        self.request_without_body(Method::POST, "/admin/storage/load_all")
            .await
    }
}
