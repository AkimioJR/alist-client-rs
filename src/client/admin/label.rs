//! Client methods for `/api/admin/label/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::label::{
    Label, LabelCreateReq, LabelDeleteReq, LabelIdResp, LabelUpdateReq,
};
use crate::models::admin::{AdminPageQuery, IdQuery};
use crate::models::common::PageResp;
use reqwest::Method;

impl Client {
    /// List labels with `/api/admin/label/list`.
    pub async fn admin_label_list(&self, query: AdminPageQuery) -> Result<PageResp<Label>> {
        self.request_with_query(Method::GET, "/admin/label/list", &query)
            .await
    }

    /// Create a label with `/api/admin/label/create`.
    pub async fn admin_label_create(&self, req: LabelCreateReq) -> Result<LabelIdResp> {
        self.request(Method::POST, "/admin/label/create", Some(&req), false)
            .await
    }

    /// Update a label with `/api/admin/label/update`.
    pub async fn admin_label_update(&self, req: LabelUpdateReq) -> Result<Label> {
        self.request(Method::POST, "/admin/label/update", Some(&req), false)
            .await
    }

    /// Get a label with `/api/admin/label/get`.
    pub async fn admin_label_get(&self, id: u64) -> Result<Label> {
        self.request_with_query(Method::GET, "/admin/label/get", &IdQuery { id })
            .await
    }

    /// Delete a label with `/api/admin/label/delete`.
    pub async fn admin_label_delete(&self, id: impl Into<String>) -> Result<()> {
        let req = LabelDeleteReq { id: id.into() };
        self.request(Method::POST, "/admin/label/delete", Some(&req), false)
            .await
    }
}
