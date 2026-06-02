//! Async HTTP client core for AList APIs.

mod archive;
mod auth;
mod fs;
mod public;
mod upload;

use crate::error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
use crate::models::{ApiResponse, LoginReq, LoginResp};
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::RwLock;

pub use upload::UploadPut;

/// Stored authentication material used to refresh the current token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    /// Login with username and password when the current token is missing or expired.
    UsernamePassword {
        username: String,
        password: String,
        otp_code: Option<String>,
    },
    /// Re-apply this token when the current token is missing or rejected.
    Token(String),
}

impl Authentication {
    /// Create username/password authentication.
    pub fn username_password(
        username: impl Into<String>,
        password: impl Into<String>,
        otp_code: impl Into<Option<String>>,
    ) -> Self {
        Self::UsernamePassword {
            username: username.into(),
            password: password.into(),
            otp_code: otp_code.into(),
        }
    }

    /// Create token authentication.
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }
}

/// Async AList API client.
#[derive(Debug)]
pub struct Client {
    base_url: Url,
    http: reqwest::Client,
    token: RwLock<Option<String>>,
    authentication: Option<Authentication>,
}

impl Client {
    /// Create a client from an AList site URL.
    ///
    /// The URL may include trailing slashes; request paths are always generated
    /// under `/api` unless an absolute endpoint is explicitly used internally.
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let trimmed = base_url.as_ref().trim_end_matches('/');
        let normalized = format!("{trimmed}/");
        let base_url = Url::parse(&normalized)?;
        Ok(Self {
            base_url,
            http: reqwest::Client::new(),
            token: RwLock::new(None),
            authentication: None,
        })
    }

    /// Create a client with refreshable authentication credentials.
    pub fn with_authentication(
        base_url: impl AsRef<str>,
        authentication: Authentication,
    ) -> Result<Self> {
        let mut client = Self::new(base_url)?;
        client.set_authentication(authentication);
        Ok(client)
    }

    /// Return the current token, if any.
    pub(crate) fn token(&self) -> Option<String> {
        self.token.read().ok().and_then(|token| token.clone())
    }

    /// Return the configured refresh authentication, if any.
    pub fn authentication(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    /// Set authentication material used to refresh the current token.
    pub fn set_authentication(&mut self, authentication: Authentication) {
        if let Authentication::Token(token) = &authentication {
            self.replace_token(Some(token.clone()));
        }
        self.authentication = Some(authentication);
    }

    /// Clear refresh authentication without clearing the current token.
    pub fn clear_authentication(&mut self) {
        self.authentication = None;
    }

    /// Return the base site URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Build an API URL from a path such as `/fs/list`.
    pub fn api_url(&self, path: &str) -> Result<Url> {
        let path = path.trim_start_matches('/');
        Ok(self.base_url.join(&format!("api/{path}"))?)
    }

    async fn request<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        is_retry: bool,
    ) -> Result<T> {
        let url = self.api_url(path)?;
        let mut builder = self.http.request(method.clone(), url);
        builder = self.apply_auth(builder);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        let response = builder.send().await?;
        let status = response.status();
        let resp_body = response.text().await?;
        if !status.is_success() {
            return Err(ClientError::HttpStatus {
                status,
                body: resp_body,
            });
        }
        let envelope: ApiResponse<Value> = serde_json::from_str(&resp_body)?;
        let code = ApiStatusCode::from_code(envelope.code);

        // 未授权或权限不足时，如果尚未重试过且配置了账号密码认证，则尝试刷新令牌后重试一次。
        let should_refresh_token =
            matches!(code, ApiStatusCode::Unauthorized | ApiStatusCode::Forbidden)
                && !is_retry
                && matches!(
                    self.authentication,
                    Some(Authentication::UsernamePassword { .. })
                );

        if should_refresh_token {
            self.refresh_token().await?;
            return self.request(method, path, body, true).await;
        }

        if !code.is_success() {
            return Err(ClientError::Api {
                code,
                kind: InternalErrorKind::from_message(&envelope.message),
                message: envelope.message,
                data: envelope.data,
            });
        }

        let data: T = serde_json::from_str(&resp_body)?;
        Ok(data)
    }

    fn apply_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = self.token() {
            builder.header("Authorization", token)
        } else {
            builder
        }
    }

    async fn refresh_token(&self) -> Result<()> {
        match self.authentication.as_ref() {
            Some(Authentication::UsernamePassword {
                username,
                password,
                otp_code,
            }) => {
                let req = LoginReq {
                    username: username.clone(),
                    password: password.clone(),
                    otp_code: otp_code.clone(),
                };
                let resp = self.login_with(req).await?;
                self.replace_token(Some(resp.token));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn replace_token(&self, token: Option<String>) {
        if let Ok(mut current) = self.token.write() {
            *current = token;
        }
    }
}
