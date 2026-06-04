//! Client methods for `/api/auth/*` and `/api/me`.

use super::Client;
use crate::error::Result;
use crate::models::auth::{
    Generate2FaResp, LoginReq, LoginResp, MeResp, RegisterReq, Verify2FaReq,
};
use reqwest::Method;

impl Client {
    /// Log in using `/api/auth/login` with a complete login payload, including optional 2FA code.
    pub(crate) async fn login_with(&self, req: LoginReq) -> Result<LoginResp> {
        self.request_without_refresh(Method::POST, "/auth/login", Some(&req))
            .await
    }

    /// Log in with an AList static password hash using `/api/auth/login/hash`.
    pub async fn login_hash_with(&self, req: LoginReq) -> Result<LoginResp> {
        self.request_without_refresh(Method::POST, "/auth/login/hash", Some(&req))
            .await
    }

    /// Register a new user with `/api/auth/register`.
    pub async fn register(&self, req: RegisterReq) -> Result<()> {
        self.request(Method::POST, "/auth/register", Some(&req), false)
            .await
    }

    /// Generate a two-factor authentication secret with `/api/auth/2fa/generate`.
    pub async fn generate_2fa(&self) -> Result<Generate2FaResp> {
        self.request_without_body(Method::POST, "/auth/2fa/generate")
            .await
    }

    /// Verify and enable two-factor authentication with `/api/auth/2fa/verify`.
    pub async fn verify_2fa(&self, req: Verify2FaReq) -> Result<()> {
        self.request(Method::POST, "/auth/2fa/verify", Some(&req), false)
            .await
    }

    /// Fetch current user info from `/api/me`.
    pub async fn me(&self) -> Result<MeResp> {
        self.request::<(), MeResp>(Method::GET, "/me", None, false)
            .await
    }
}
