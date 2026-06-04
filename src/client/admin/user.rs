//! Client methods for `/api/admin/user/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::user::{AdminUser, AdminUserReq};
use crate::models::admin::{IdQuery, UsernameQuery};
use crate::models::common::PageResp;
use reqwest::Method;

impl Client {
    /// List users with `/api/admin/user/list`.
    pub async fn admin_user_list(&self) -> Result<PageResp<AdminUser>> {
        self.request_without_body(Method::GET, "/admin/user/list")
            .await
    }

    /// Get a user with `/api/admin/user/get`.
    pub async fn admin_user_get(&self, id: u64) -> Result<AdminUser> {
        self.request_with_query(Method::GET, "/admin/user/get", &IdQuery { id })
            .await
    }

    /// Create a user with `/api/admin/user/create`.
    pub async fn admin_user_create(&self, req: AdminUserReq) -> Result<()> {
        self.request(Method::POST, "/admin/user/create", Some(&req), false)
            .await
    }

    /// Update a user with `/api/admin/user/update`.
    pub async fn admin_user_update(&self, req: AdminUserReq) -> Result<()> {
        self.request(Method::POST, "/admin/user/update", Some(&req), false)
            .await
    }

    /// Cancel a user's two-factor authentication with `/api/admin/user/cancel_2fa`.
    pub async fn admin_user_cancel_2fa(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/user/cancel_2fa", &IdQuery { id })
            .await
    }

    /// Delete a user with `/api/admin/user/delete`.
    pub async fn admin_user_delete(&self, id: u64) -> Result<()> {
        self.request_with_query(Method::POST, "/admin/user/delete", &IdQuery { id })
            .await
    }

    /// Delete a user's cache with `/api/admin/user/del_cache`.
    pub async fn admin_user_delete_cache(&self, username: impl Into<String>) -> Result<()> {
        let query = UsernameQuery {
            username: username.into(),
        };
        self.request_with_query(Method::POST, "/admin/user/del_cache", &query)
            .await
    }
}
