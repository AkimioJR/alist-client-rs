use super::Client;
use crate::error::{ClientError, Result};

impl Client {
    /// ping the server to check if it's alive.
    /// Returns `true` if the response is "pong".
    pub async fn ping(&self) -> Result<bool> {
        let url = self.api_url("/ping")?;
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let body = resp.text().await?;
        if !status.is_success() {
            return Err(ClientError::HttpStatus { status, body });
        }
        Ok(body.trim() == "pong")
    }
}
