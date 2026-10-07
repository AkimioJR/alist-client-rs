//! Client methods for `/api/public/*`.

use reqwest::Method;

use super::Client;
use crate::{error::Result, models::public::PublicSettings};

impl Client {
    /// Fetch public site settings from `/api/public/settings`.
    pub async fn public_settings(&self) -> Result<PublicSettings> {
        self.request::<(), PublicSettings>(Method::GET, "/public/settings", None, false)
            .await
    }
}
