//! Client methods for `/api/admin/driver/*`.

use reqwest::Method;

use super::super::Client;
use crate::{
    error::Result,
    models::admin::{
        DriverQuery,
        driver::{DriverInfoResp, DriverListResp, DriverNamesResp},
    },
};

impl Client {
    /// List driver templates with `/api/admin/driver/list`.
    pub async fn admin_driver_list(&self) -> Result<DriverListResp> {
        self.request_without_body(Method::GET, "/admin/driver/list")
            .await
    }

    /// List driver names with `/api/admin/driver/names`.
    pub async fn admin_driver_names(&self) -> Result<DriverNamesResp> {
        self.request_without_body(Method::GET, "/admin/driver/names")
            .await
    }

    /// Get one driver template with `/api/admin/driver/info`.
    pub async fn admin_driver_info(&self, driver: impl Into<String>) -> Result<DriverInfoResp> {
        let query = DriverQuery {
            driver: driver.into(),
        };
        self.request_with_query(Method::GET, "/admin/driver/info", &query)
            .await
    }
}
