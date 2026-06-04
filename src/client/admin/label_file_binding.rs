//! Client methods for `/api/admin/label_file_binding/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::label::Label;
use crate::models::admin::label_file_binding::{
    FileNameQuery, LabelFileBindingBatchReq, LabelFileBindingBatchResp, LabelFileBindingCreateReq,
    LabelFileBindingCreateResp, LabelFileBindingDeleteReq, LabelIdQuery, LabeledFile,
};
use reqwest::Method;

impl Client {
    /// Bind labels to a file with `/api/admin/label_file_binding/create`.
    pub async fn admin_label_file_binding_create(
        &self,
        req: LabelFileBindingCreateReq,
    ) -> Result<LabelFileBindingCreateResp> {
        self.request(
            Method::POST,
            "/admin/label_file_binding/create",
            Some(&req),
            false,
        )
        .await
    }

    /// Get labels by file name with `/api/admin/label_file_binding/get`.
    pub async fn admin_label_file_binding_get(
        &self,
        file_name: impl Into<String>,
    ) -> Result<Vec<Label>> {
        let query = FileNameQuery {
            file_name: file_name.into(),
        };
        self.request_with_query(Method::GET, "/admin/label_file_binding/get", &query)
            .await
    }

    /// Delete a file label binding with `/api/admin/label_file_binding/delete`.
    pub async fn admin_label_file_binding_delete(
        &self,
        req: LabelFileBindingDeleteReq,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/admin/label_file_binding/delete",
            Some(&req),
            false,
        )
        .await
    }

    /// Get files by label with `/api/admin/label_file_binding/get_file_by_label`.
    pub async fn admin_label_file_binding_get_file_by_label(
        &self,
        label_id: impl Into<String>,
    ) -> Result<Vec<LabeledFile>> {
        let query = LabelIdQuery {
            label_id: label_id.into(),
        };
        self.request_with_query(
            Method::GET,
            "/admin/label_file_binding/get_file_by_label",
            &query,
        )
        .await
    }

    /// Batch bind labels to files with `/api/admin/label_file_binding/create_batch`.
    pub async fn admin_label_file_binding_create_batch(
        &self,
        req: LabelFileBindingBatchReq,
    ) -> Result<LabelFileBindingBatchResp> {
        self.request(
            Method::POST,
            "/admin/label_file_binding/create_batch",
            Some(&req),
            false,
        )
        .await
    }
}
