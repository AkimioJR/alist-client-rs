//! Async HTTP client core for AList APIs.

mod archive;
mod auth;
mod fs;
mod public;
mod upload;

use crate::error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
use crate::models::{ApiResponse, LoginReq};
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
    authentication: RwLock<Option<Authentication>>,
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
            authentication: RwLock::new(None),
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

    /// Create a client with token authentication.
    pub fn with_token(base_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        Self::with_authentication(base_url, Authentication::token(token))
    }

    /// Return the current token, if any.
    pub(crate) fn token(&self) -> Option<String> {
        self.token.read().ok().and_then(|token| token.clone())
    }

    /// Return the configured refresh authentication, if any.
    pub(crate) fn authentication(&self) -> Option<Authentication> {
        self.authentication
            .read()
            .ok()
            .and_then(|authentication| authentication.clone())
    }

    /// Set authentication material used to refresh the current token.
    pub fn set_authentication(&mut self, authentication: Authentication) {
        if let Authentication::Token(token) = &authentication {
            self.replace_token(Some(token.clone()));
        }
        if let Ok(mut current) = self.authentication.write() {
            *current = Some(authentication);
        }
    }

    /// Clear refresh authentication without clearing the current token.
    pub fn clear_authentication(&mut self) {
        if let Ok(mut current) = self.authentication.write() {
            *current = None;
        }
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

    // 发送 API 请求的核心方法，处理认证、错误检查和响应解析
    // - `method`: HTTP 方法（GET、POST 等）
    // - `path`: API 路径（如 `/fs/list`）
    // - `body`: 可选的请求体，必须可序列化为 JSON
    // - `is_retry`: 内部标志，指示这是否是由于认证失败而进行的重试，以避免无限重试循环
    async fn request<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        is_retry: bool,
    ) -> Result<T> {
        let mut is_retry = is_retry;

        loop {
            let url = self.api_url(path)?;
            let mut builder = self.http.request(method.clone(), url);
            builder = self.apply_auth(builder);
            if let Some(body) = body {
                builder = builder.json(body);
            }
            let response = builder.send().await?;

            match self.decode_response_value(response).await {
                Ok(value) => return Ok(serde_json::from_value(value)?),
                Err(err) if !is_retry && self.should_refresh_auth(&err) => {
                    self.refresh_token().await?;
                    is_retry = true;
                }
                Err(err) => return Err(err),
            }
        }
    }

    async fn request_without_refresh<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.api_url(path)?;
        let mut builder = self.http.request(method, url);
        builder = self.apply_auth(builder);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        let response = builder.send().await?;
        let value = self.decode_response_value(response).await?;
        Ok(serde_json::from_value(value)?)
    }

    async fn decode_response_nullable<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<Option<T>> {
        let value = self.decode_response_value(response).await?;
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(value)?))
        }
    }

    async fn decode_response_value(&self, response: reqwest::Response) -> Result<Value> {
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
        if !code.is_success() {
            return Err(ClientError::Api {
                code,
                kind: InternalErrorKind::from_message(&envelope.message),
                message: envelope.message,
                data: envelope.data,
            });
        }

        Ok(envelope.data)
    }

    fn apply_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = self.token() {
            builder.header("Authorization", token)
        } else {
            builder
        }
    }

    async fn refresh_token(&self) -> Result<()> {
        match self.authentication() {
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

    fn should_refresh_auth(&self, err: &ClientError) -> bool {
        if !matches!(
            self.authentication(),
            Some(Authentication::UsernamePassword { .. })
        ) {
            return false;
        }

        match err {
            ClientError::Api {
                code: ApiStatusCode::Unauthorized | ApiStatusCode::Forbidden,
                ..
            } => true,
            ClientError::HttpStatus { status, .. } => {
                *status == reqwest::StatusCode::UNAUTHORIZED
                    || *status == reqwest::StatusCode::FORBIDDEN
            }
            _ => false,
        }
    }

    fn replace_token(&self, token: Option<String>) {
        if let Ok(mut current) = self.token.write() {
            *current = token;
        }
    }
}
