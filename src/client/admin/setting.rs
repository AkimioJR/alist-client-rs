//! Client methods for `/api/admin/setting/*`.

use super::super::Client;
use crate::error::Result;
use crate::models::admin::setting::{
    SetAria2Req, SetQbitReq, Setting, SettingGetQuery, SettingKeyQuery, SettingListQuery,
};
use reqwest::Method;

impl Client {
    /// List settings with `/api/admin/setting/list`.
    pub async fn admin_setting_list(&self, query: SettingListQuery) -> Result<Vec<Setting>> {
        self.request_with_query(Method::GET, "/admin/setting/list", &query)
            .await
    }

    /// Get a setting with `/api/admin/setting/get`.
    pub async fn admin_setting_get(&self, query: SettingGetQuery) -> Result<Setting> {
        self.request_with_query(Method::GET, "/admin/setting/get", &query)
            .await
    }

    /// Save settings with `/api/admin/setting/save`.
    pub async fn admin_setting_save(&self, settings: Vec<Setting>) -> Result<()> {
        self.request(Method::POST, "/admin/setting/save", Some(&settings), false)
            .await
    }

    /// Delete a deprecated setting with `/api/admin/setting/delete`.
    pub async fn admin_setting_delete(&self, key: impl Into<String>) -> Result<()> {
        let query = SettingKeyQuery { key: key.into() };
        self.request_with_query(Method::POST, "/admin/setting/delete", &query)
            .await
    }

    /// Reset the permanent token with `/api/admin/setting/reset_token`.
    pub async fn admin_setting_reset_token(&self) -> Result<String> {
        self.request_without_body(Method::POST, "/admin/setting/reset_token")
            .await
    }

    /// Configure aria2 with `/api/admin/setting/set_aria2`.
    pub async fn admin_setting_set_aria2(&self, req: SetAria2Req) -> Result<String> {
        self.request(Method::POST, "/admin/setting/set_aria2", Some(&req), false)
            .await
    }

    /// Configure qBittorrent with `/api/admin/setting/set_qbit`.
    pub async fn admin_setting_set_qbit(&self, req: SetQbitReq) -> Result<String> {
        self.request(Method::POST, "/admin/setting/set_qbit", Some(&req), false)
            .await
    }
}
