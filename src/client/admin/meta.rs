//! Client methods for `/api/admin/meta/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::meta::{Meta, MetaReq};
use crate::models::admin::{AdminPageQuery, IdQuery};
use crate::models::common::PageResp;
use reqwest::Method;

impl Client {
    /// List metadata rules with `/api/admin/meta/list`.
    pub async fn admin_meta_list(&self, query: AdminPageQuery) -> Result<PageResp<Meta>> {
        self.request_with_query(Method::GET, "/admin/meta/list", &query)
            .await
    }

    /// Get a metadata rule with `/api/admin/meta/get`.
    pub async fn admin_meta_get(&self, id: u64) -> Result<Meta> {
        self.request_with_query(Method::GET, "/admin/meta/get", &IdQuery { id })
            .await
    }

    /// Create a metadata rule with `/api/admin/meta/create`.
    pub async fn admin_meta_create(&self, req: MetaReq) -> Result<()> {
        self.request(Method::POST, "/admin/meta/create", Some(&req), false)
            .await
    }

    /// Update a metadata rule with `/api/admin/meta/update`.
    pub async fn admin_meta_update(&self, req: MetaReq) -> Result<()> {
        self.request(Method::POST, "/admin/meta/update", Some(&req), false)
            .await
    }

    /// Delete a metadata rule with `/api/admin/meta/delete`.
    pub async fn admin_meta_delete(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/meta/delete", &IdQuery { id })
            .await
    }
}
