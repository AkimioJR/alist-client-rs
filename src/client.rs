//! Async HTTP client core for AList APIs.

pub mod admin;
pub mod auth;
pub mod fs;
pub mod public;

use crate::error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
use crate::models::auth::LoginReq;
use crate::models::common::ApiResponse;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::RwLock;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{Instant, sleep_until};

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
    api_request_rate_limit: Option<RequestRateLimit>,
}

#[derive(Debug)]
struct RequestRateLimit {
    interval: Duration,
    next_request_at: Mutex<Instant>,
}

struct DecodedResponse {
    data: Value,
    body: String,
}

impl RequestRateLimit {
    fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_request_at: Mutex::new(Instant::now()),
        }
    }

    async fn wait(&self) {
        let mut next_request_at = self.next_request_at.lock().await;
        let now = Instant::now();

        if *next_request_at > now {
            let scheduled_at = *next_request_at;
            *next_request_at += self.interval;
            drop(next_request_at);
            sleep_until(scheduled_at).await;
        } else {
            *next_request_at = now + self.interval;
        }
    }
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
            api_request_rate_limit: None,
        })
    }

    /// Return the minimum interval enforced between API requests.
    ///
    /// `None` means requests are sent without client-side rate limiting.
    pub fn api_request_interval(&self) -> Option<Duration> {
        self.api_request_rate_limit
            .as_ref()
            .map(|rate_limit| rate_limit.interval)
    }

    /// Configure the minimum interval between API requests.
    ///
    /// `None` or `Duration::ZERO` disables client-side rate limiting.
    pub fn set_api_request_interval(&mut self, interval: impl Into<Option<Duration>>) {
        self.api_request_rate_limit = interval
            .into()
            .filter(|interval| !interval.is_zero())
            .map(RequestRateLimit::new);
    }

    /// Configure the minimum interval between API requests while building a client.
    ///
    /// `None` or `Duration::ZERO` disables client-side rate limiting.
    pub fn with_api_request_interval(mut self, interval: impl Into<Option<Duration>>) -> Self {
        self.set_api_request_interval(interval);
        self
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

    /// Set the authentication material used to refresh the current token.
    pub fn with_authentication(mut self, authentication: Authentication) -> Self {
        self.set_authentication(authentication);
        self
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
        let request_body = self.serialize_request_body(method.as_str(), path, body)?;

        loop {
            let url = self.api_url(path)?;
            let mut builder = self.http.request(method.clone(), url);
            builder = self.apply_auth(builder);
            if let Some(request_body) = &request_body {
                builder = builder
                    .header(CONTENT_TYPE, "application/json")
                    .body(request_body.clone());
            }
            self.wait_for_rate_limit().await;
            let response = builder.send().await?;

            match self
                .decode_response_value(method.as_str(), path, request_body.as_deref(), response)
                .await
            {
                Ok(decoded) => {
                    return self.decode_response_data(
                        decoded,
                        method.as_str(),
                        path,
                        request_body.as_deref(),
                    );
                }
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
        let request_body = self.serialize_request_body(method.as_str(), path, body)?;
        let url = self.api_url(path)?;
        let mut builder = self.http.request(method.clone(), url);
        builder = self.apply_auth(builder);
        if let Some(request_body) = &request_body {
            builder = builder
                .header(CONTENT_TYPE, "application/json")
                .body(request_body.clone());
        }
        self.wait_for_rate_limit().await;
        let response = builder.send().await?;
        let decoded = self
            .decode_response_value(method.as_str(), path, request_body.as_deref(), response)
            .await?;
        self.decode_response_data(decoded, method.as_str(), path, request_body.as_deref())
    }

    async fn request_with_query<Q: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        let url = self.api_url_with_query(method.as_str(), path, query)?;
        let mut builder = self.http.request(method.clone(), url);
        builder = self.apply_auth(builder);
        self.wait_for_rate_limit().await;
        let response = builder.send().await?;
        let decoded = self
            .decode_response_value(method.as_str(), path, None, response)
            .await?;
        self.decode_response_data(decoded, method.as_str(), path, None)
    }

    async fn request_without_body<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<T> {
        self.request::<(), T>(method, path, None, false).await
    }

    async fn decode_response_nullable<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        request_body: Option<&str>,
        response: reqwest::Response,
    ) -> Result<Option<T>> {
        let decoded = self
            .decode_response_value(method, path, request_body, response)
            .await?;
        if decoded.data.is_null() {
            Ok(None)
        } else {
            self.decode_response_data(decoded, method, path, request_body)
                .map(Some)
        }
    }

    async fn decode_response_value(
        &self,
        method: &str,
        path: &str,
        request_body: Option<&str>,
        response: reqwest::Response,
    ) -> Result<DecodedResponse> {
        let status = response.status();
        let resp_body = response.text().await?;
        if !status.is_success() {
            return Err(ClientError::HttpStatus {
                status,
                body: resp_body,
            });
        }
        let envelope: ApiResponse<Value> = serde_json::from_str(&resp_body).map_err(|source| {
            Self::json_error(
                source,
                method,
                path,
                request_body.map(str::to_string),
                Some(resp_body.clone()),
            )
        })?;
        let code = ApiStatusCode::from_code(envelope.code);
        if !code.is_success() {
            return Err(ClientError::Api {
                code,
                kind: InternalErrorKind::from_message(&envelope.message),
                message: envelope.message,
                data: envelope.data,
            });
        }

        Ok(DecodedResponse {
            data: envelope.data,
            body: resp_body,
        })
    }

    fn decode_response_data<T: DeserializeOwned>(
        &self,
        decoded: DecodedResponse,
        method: &str,
        path: &str,
        request_body: Option<&str>,
    ) -> Result<T> {
        serde_json::from_value(decoded.data).map_err(|source| {
            Self::json_error(
                source,
                method,
                path,
                request_body.map(str::to_string),
                Some(decoded.body),
            )
        })
    }

    fn serialize_request_body<B: Serialize + ?Sized>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> Result<Option<String>> {
        body.map(serde_json::to_string)
            .transpose()
            .map_err(|source| Self::json_error(source, method, path, None, None))
    }

    fn api_url_with_query<Q: Serialize + ?Sized>(
        &self,
        method: &str,
        path: &str,
        query: &Q,
    ) -> Result<Url> {
        let mut url = self.api_url(path)?;
        let value = serde_json::to_value(query)
            .map_err(|source| Self::json_error(source, method, path, None, None))?;
        if let Value::Object(params) = value {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in params {
                // 将简单标量转换成 query 参数，跳过缺省的空值。
                match value {
                    Value::Null => {}
                    Value::String(value) => {
                        pairs.append_pair(&key, &value);
                    }
                    Value::Bool(value) => {
                        pairs.append_pair(&key, if value { "true" } else { "false" });
                    }
                    Value::Number(value) => {
                        pairs.append_pair(&key, &value.to_string());
                    }
                    Value::Array(values) => {
                        for value in values {
                            pairs.append_pair(&key, &value.to_string());
                        }
                    }
                    Value::Object(value) => {
                        pairs.append_pair(&key, &Value::Object(value).to_string());
                    }
                }
            }
        }
        Ok(url)
    }

    fn json_error(
        source: serde_json::Error,
        method: &str,
        path: &str,
        request_body: Option<String>,
        response_body: Option<String>,
    ) -> ClientError {
        ClientError::JsonWithContext {
            source,
            method: method.to_string(),
            path: path.to_string(),
            request_body,
            response_body,
        }
    }

    fn apply_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = self.token() {
            builder.header("Authorization", token)
        } else {
            builder
        }
    }

    async fn wait_for_rate_limit(&self) {
        if let Some(rate_limit) = &self.api_request_rate_limit {
            rate_limit.wait().await;
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
