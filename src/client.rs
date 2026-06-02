//! Async HTTP client core for AList APIs.

mod archive;
mod auth;
mod fs;
mod public;
mod upload;

use crate::error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
use crate::models::ApiResponse;
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub use upload::UploadPut;

/// Async AList API client.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Url,
    http: reqwest::Client,
    token: Option<String>,
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
            token: None,
        })
    }

    /// Create a client with an existing AList token.
    pub fn with_token(base_url: impl AsRef<str>, token: impl Into<String>) -> Result<Self> {
        let mut client = Self::new(base_url)?;
        client.set_token(token);
        Ok(client)
    }

    /// Return the current token, if any.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Set or replace the token used in the `Authorization` header.
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    /// Clear the current authorization token.
    pub fn clear_token(&mut self) {
        self.token = None;
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
        let response = self.send_json(method, path, body).await?;
        self.decode_response(response).await
    }

    async fn request_json_nullable<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<Option<T>> {
        let response = self.send_json(method, path, body).await?;
        self.decode_response_nullable(response).await
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

    async fn decode_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
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
        if let Some(token) = &self.token {
            builder.header("Authorization", token)
        } else {
            builder
        }
    }
}
