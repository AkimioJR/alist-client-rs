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
use std::sync::{Arc, RwLock};

pub use upload::UploadPut;

/// Stored authentication material used to refresh the current token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    /// Login with username and password when the current token is missing or expired.
    UsernamePassword { username: String, password: String },
    /// Re-apply this token when the current token is missing or rejected.
    Token(String),
}

impl Authentication {
    /// Create username/password authentication.
    pub fn username_password(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::UsernamePassword {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create token authentication.
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }
}

/// Async AList API client.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    http: reqwest::Client,
    token: Arc<RwLock<Option<String>>>,
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
            token: Arc::new(RwLock::new(None)),
            authentication: None,
        })
    }

    /// Create a client with an existing AList token.
    pub fn with_token(base_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        let mut client = Self::new(base_url)?;
        client.set_token(token.clone());
        client.set_authentication(Authentication::Token(token));
        Ok(client)
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
    pub fn token(&self) -> Option<String> {
        self.token.read().ok().and_then(|token| token.clone())
    }

    /// Set or replace the token used in the `Authorization` header.
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.replace_token(Some(token.into()));
    }

    /// Clear the current authorization token.
    pub fn clear_token(&mut self) {
        self.replace_token(None);
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

    async fn request_unit<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        let _: Option<Value> = self.request_json_nullable(method, path, body).await?;
        Ok(())
    }

    async fn request_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let value = self.request_value(method, path, body, true).await?;
        Ok(serde_json::from_value(value)?)
    }

    async fn request_json_nullable<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<Option<T>> {
        let value = self.request_value(method, path, body, true).await?;
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_value(value)?))
        }
    }

    async fn request_json_without_refresh<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let response = self.send_json(method, path, body).await?;
        self.decode_response(response).await
    }

    async fn request_value<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        retry_auth: bool,
    ) -> Result<Value> {
        let response = self.send_json(method.clone(), path, body).await?;
        match self.decode_response_value(response).await {
            Ok(value) => Ok(value),
            Err(err) if retry_auth && self.should_refresh_auth(&err) => {
                self.refresh_token().await?;
                let response = self.send_json(method, path, body).await?;
                self.decode_response_value(response).await
            }
            Err(err) => Err(err),
        }
    }

    async fn send_json<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<reqwest::Response> {
        let url = self.api_url(path)?;
        let mut builder = self.http.request(method, url);
        builder = self.apply_auth(builder);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        Ok(builder.send().await?)
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

    async fn decode_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let value = self.decode_response_value(response).await?;
        Ok(serde_json::from_value(value)?)
    }

    async fn decode_response_value(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(ClientError::HttpStatus { status, body: text });
        }

        let envelope: ApiResponse<Value> = serde_json::from_str(&text)?;
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
        match self.authentication.as_ref() {
            Some(Authentication::UsernamePassword { username, password }) => {
                let req = LoginReq {
                    username: username.clone(),
                    password: password.clone(),
                    otp_code: None,
                };
                let response = self
                    .send_json(Method::POST, "/auth/login", Some(&req))
                    .await?;
                let resp: LoginResp = self.decode_response(response).await?;
                self.replace_token(Some(resp.token));
                Ok(())
            }
            Some(Authentication::Token(token)) => {
                self.replace_token(Some(token.clone()));
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn should_refresh_auth(&self, err: &ClientError) -> bool {
        if self.authentication.is_none() {
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
