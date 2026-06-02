//! Client methods for `/api/public/*`.

use super::Client;
use crate::error::Result;
use crate::models::PublicSettings;
use reqwest::Method;

impl Client {
    /// Fetch public site settings from `/api/public/settings`.
    pub async fn public_settings(&self) -> Result<PublicSettings> {
        self.request_json::<(), PublicSettings>(Method::GET, "/public/settings", None)
            .await
    }
}
