//! Client methods for `/api/admin/role/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::IdQuery;
use crate::models::admin::role::{Role, RoleReq};
use crate::models::common::PageResp;
use reqwest::Method;

impl Client {
    /// List roles with `/api/admin/role/list`.
    pub async fn admin_role_list(&self) -> Result<PageResp<Role>> {
        self.request_without_body(Method::GET, "/admin/role/list")
            .await
    }

    /// Get a role with `/api/admin/role/get`.
    pub async fn admin_role_get(&self, id: u64) -> Result<Role> {
        self.request_with_query(Method::GET, "/admin/role/get", &IdQuery { id })
            .await
    }

    /// Create a role with `/api/admin/role/create`.
    pub async fn admin_role_create(&self, req: RoleReq) -> Result<()> {
        self.request(Method::POST, "/admin/role/create", Some(&req), false)
            .await
    }

    /// Update a role with `/api/admin/role/update`.
    pub async fn admin_role_update(&self, req: RoleReq) -> Result<()> {
        self.request(Method::POST, "/admin/role/update", Some(&req), false)
            .await
    }

    /// Delete a role with `/api/admin/role/delete`.
    pub async fn admin_role_delete(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/role/delete", &IdQuery { id })
            .await
    }
}
