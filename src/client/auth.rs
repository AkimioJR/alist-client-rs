//! Client methods for `/api/auth/*` and `/api/me`.

use super::Client;
use crate::error::Result;
use crate::models::{LoginReq, LoginResp, MeResp};
use reqwest::Method;

impl Client {
    /// Log in using `/api/auth/login` with a complete login payload, including optional 2FA code.
    pub(crate) async fn login_with(&self, req: LoginReq) -> Result<LoginResp> {
        let resp: LoginResp = self
            .request_json_without_refresh(Method::POST, "/auth/login", Some(&req))
            .await?;
        Ok(resp)
    }

    /// Log in with an AList static password hash using `/api/auth/login/hash`.
    pub async fn login_hash_with(&self, req: LoginReq) -> Result<LoginResp> {
        let resp: LoginResp = self
            .request_json_without_refresh(Method::POST, "/auth/login/hash", Some(&req))
            .await?;
        Ok(resp)
    }

    /// Fetch current user info from `/api/me`.
    pub async fn me(&self) -> Result<MeResp> {
        self.request_json::<(), MeResp>(Method::GET, "/me", None)
            .await
    }
}
