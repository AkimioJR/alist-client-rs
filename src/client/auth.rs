//! Client methods for `/api/auth/*` and `/api/me`.

use super::Client;
use crate::error::Result;
use crate::models::{LoginReq, LoginResp, MeResp};
use reqwest::Method;

impl Client {
    /// Log in with a raw password using `/api/auth/login` and store the token.
    pub async fn login(
        &mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<LoginResp> {
        let req = LoginReq {
            username: username.into(),
            password: password.into(),
            otp_code: None,
        };
        self.login_with(req).await
    }

    /// Log in with a complete login payload, including optional 2FA code.
    pub async fn login_with(&mut self, req: LoginReq) -> Result<LoginResp> {
        let resp: LoginResp = self
            .request_json_without_refresh(Method::POST, "/auth/login", Some(&req))
            .await?;
        self.set_token(resp.token.clone());
        Ok(resp)
    }

    /// Log in with an AList static password hash using `/api/auth/login/hash`.
    pub async fn login_hash_with(&mut self, req: LoginReq) -> Result<LoginResp> {
        let resp: LoginResp = self
            .request_json_without_refresh(Method::POST, "/auth/login/hash", Some(&req))
            .await?;
        self.set_token(resp.token.clone());
        Ok(resp)
    }

    /// Fetch current user info from `/api/me`.
    pub async fn me(&self) -> Result<MeResp> {
        self.request_json::<(), MeResp>(Method::GET, "/me", None)
            .await
    }
}
