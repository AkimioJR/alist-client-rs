//! AList 异步 HTTP 客户端核心。
//!
//! [`Client`] 统一负责：
//!
//! - 站点基址（[`Url`]）与 API 路径拼接；
//! - token 内部状态与 `Authorization` 头注入（发送时注入，而非构建时）；
//! - [`Authentication`] 凭据保存与 401/403 时的自动重新登录并重试一次；
//! - 客户端侧请求限速（[`RequestRateLimit`]）；
//! - JSON 信封解码、HTTP 状态与信封 `code` 检查、`data: null` → `Option<T>`/`()` 解码；
//! - 原始文本响应辅助（供 `/ping` 等非 JSON 端点使用）。
//!
//! 端点模块通过 `pub(crate) request`/`execute` 组合出具体 API 调用，
//! 见 `src/endpoint.rs` 与 `docs/design.md`。
//!
//! 过渡期说明：端点文件尚为实现骨架，`request`/`execute`/`execute_text` 等核心方法
//! 在非测试构建中暂无调用方，因此本模块在非测试构建下豁免 dead_code；
//! 端点实现落地后应移除下方该行，恢复 dead_code 检查。

// 过渡期豁免（见上方模块文档），端点实现后移除。
#![cfg_attr(not(test), allow(dead_code))]

use std::{sync::RwLock, time::Duration};

use reqwest::{Method, RequestBuilder, Url, header::AUTHORIZATION};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

use crate::{
    error::{ApiStatusCode, Error, InternalErrorKind, Result},
    schema::common::Envelope,
};

/// 用于刷新当前 token 的认证凭据。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authentication {
    /// 当前 token 缺失或被拒绝时，使用用户名密码重新登录。
    UsernamePassword {
        /// 用户名。
        username: String,
        /// 密码。
        password: String,
        /// 可选的两步验证码。
        otp_code: Option<String>,
    },
    /// 当前 token 缺失或被拒绝时，重新套用该 token（不会自动登录）。
    Token(String),
}

impl Authentication {
    /// 构造用户名密码认证。
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

    /// 构造 token 认证。
    pub fn token(token: impl Into<String>) -> Self {
        Self::Token(token.into())
    }
}

/// AList 异步 API 客户端。
#[derive(Debug)]
pub struct Client {
    base_url: Url,
    http: reqwest::Client,
    token: RwLock<Option<String>>,
    authentication: RwLock<Option<Authentication>>,
    api_request_rate_limit: Option<RequestRateLimit>,
}

/// 客户端侧请求限速器。
///
/// 通过记录「下一次允许发送的时刻」串行化请求间隔：
/// 多个并发请求会在锁上排队，依次保持 `interval` 间距。
#[derive(Debug)]
struct RequestRateLimit {
    interval: Duration,
    next_request_at: Mutex<Instant>,
}

/// 请求上下文（用于 JSON 错误的最佳努力定位）。
struct RequestContext {
    method: String,
    url: String,
}

/// token 刷新使用的登录请求体（与 `/api/auth/login` 的 JSON 形状一致）。
#[derive(Serialize)]
struct LoginBody {
    username: String,
    password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    otp_code: Option<String>,
}

/// token 刷新使用的登录响应数据（与 `/api/auth/login` 的 JSON 形状一致）。
#[derive(Deserialize)]
struct LoginData {
    token: String,
}

impl RequestRateLimit {
    fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_request_at: Mutex::new(Instant::now()),
        }
    }

    /// 阻塞到当前请求的允许发送时刻，并预订下一个时刻。
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

/// 从请求构建器提取方法与 URL（流式请求体等无法克隆时返回占位符）。
fn request_context(builder: &RequestBuilder) -> RequestContext {
    builder
        .try_clone()
        .and_then(|builder| builder.build().ok())
        .map(|request| RequestContext {
            method: request.method().to_string(),
            url: request.url().to_string(),
        })
        .unwrap_or_else(|| RequestContext {
            method: "?".to_string(),
            url: "?".to_string(),
        })
}

impl Client {
    /// 从 AList 站点地址创建客户端。
    ///
    /// 地址末尾的 `/` 会被归一化；API 路径始终以相对方式拼接到该基址之后
    /// （例如基址 `https://example.com/alist` + 路径 `/api/fs/list` →
    /// `https://example.com/alist/api/fs/list`）。
    ///
    /// # Errors
    ///
    /// 基址无法解析为绝对 URL 时返回 [`Error::Url`]。
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

    /// 返回站点基址。
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// 返回相邻请求之间的最小间隔；`None` 表示未启用客户端侧限速。
    #[must_use]
    pub fn api_request_interval(&self) -> Option<Duration> {
        self.api_request_rate_limit
            .as_ref()
            .map(|rate_limit| rate_limit.interval)
    }

    /// 设置相邻请求之间的最小间隔；`None` 或 `Duration::ZERO` 表示关闭限速。
    pub fn set_api_request_interval(&mut self, interval: impl Into<Option<Duration>>) {
        self.api_request_rate_limit = interval
            .into()
            .filter(|interval| !interval.is_zero())
            .map(RequestRateLimit::new);
    }

    /// 以建造者风格设置相邻请求之间的最小间隔。
    #[must_use]
    pub fn with_api_request_interval(mut self, interval: impl Into<Option<Duration>>) -> Self {
        self.set_api_request_interval(interval);
        self
    }

    /// 设置用于刷新 token 的认证凭据。
    ///
    /// 传入 [`Authentication::Token`] 时会立即将其登记为当前 token。
    pub fn set_authentication(&mut self, authentication: Authentication) {
        if let Authentication::Token(token) = &authentication {
            self.replace_token(Some(token.clone()));
        }
        if let Ok(mut current) = self.authentication.write() {
            *current = Some(authentication);
        }
    }

    /// 清除刷新凭据（不影响已持有的 token）。
    pub fn clear_authentication(&mut self) {
        if let Ok(mut current) = self.authentication.write() {
            *current = None;
        }
    }

    /// 以建造者风格设置用于刷新 token 的认证凭据。
    #[must_use]
    pub fn with_authentication(mut self, authentication: Authentication) -> Self {
        self.set_authentication(authentication);
        self
    }

    /// 返回当前 token（内部状态，供端点与测试使用）。
    pub(crate) fn token(&self) -> Option<String> {
        self.token.read().ok().and_then(|token| token.clone())
    }

    /// 返回当前刷新凭据。
    pub(crate) fn authentication(&self) -> Option<Authentication> {
        self.authentication
            .read()
            .ok()
            .and_then(|authentication| authentication.clone())
    }

    /// 构建指向指定 API 路径的请求构建器。
    ///
    /// `path` 为自站点根起算的完整路径（如 `/api/fs/list`；`/ping` 等非 `/api` 路径同样直接传入）。
    /// 认证头不在此处注入，而是由 [`Client::execute`] 在每次发送前基于当前 token 注入，
    /// 以保证重新登录后的重试请求携带新 token。
    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let relative = path.trim_start_matches('/');
        // 以相对方式拼接以保留基址中的路径前缀；拼接失败时交由 reqwest 在发送时报错
        let url = self
            .base_url
            .join(relative)
            .map_or_else(|_| relative.to_string(), |url| url.to_string());
        self.http.request(method, url)
    }

    /// 发送请求并解码 AList JSON 信封，返回端点模型。
    ///
    /// 处理顺序：限速等待 → 注入认证头 → 发送 → HTTP 状态检查 → 信封 `code` 检查 →
    /// `data` 二次反序列化。当收到 401/403 且配置了
    /// [`Authentication::UsernamePassword`] 时，自动重新登录并重试一次。
    ///
    /// `data: null` 可解码为 `()` 或 `Option<T>`。
    pub(crate) async fn execute<R: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<R> {
        let context = request_context(&builder);
        let mut retry_builder = builder.try_clone();
        let mut pending = Some(builder);
        let mut retried = false;
        let (data, body) = loop {
            let Some(current) = pending.take() else {
                unreachable!("每轮循环都会补充待发请求");
            };
            match self.send_envelope(current, &context, true).await {
                Ok(decoded) => break decoded,
                Err(err) => {
                    let can_retry =
                        !retried && retry_builder.is_some() && self.should_refresh_auth(&err);
                    if !can_retry {
                        return Err(err);
                    }
                    self.refresh_token().await?;
                    retried = true;
                    pending = retry_builder.take();
                }
            }
        };
        serde_json::from_value(data).map_err(|source| Error::Json {
            source,
            method: context.method.clone(),
            url: context.url.clone(),
            response_body: Some(body),
        })
    }

    /// 发送请求并返回原始响应体文本（供 `/ping` 等非 JSON 端点使用）。
    ///
    /// 与 [`Client::execute`] 一致地执行限速、认证注入与 HTTP 状态检查，
    /// 但不做信封解码。
    pub(crate) async fn execute_text(&self, builder: RequestBuilder) -> Result<String> {
        let builder = self.apply_auth(builder);
        self.wait_for_rate_limit().await;
        let response = builder.send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(Error::HttpStatus { status, body });
        }
        Ok(body)
    }

    /// 单次发送并解码信封；返回 `(data, 原始响应体)`。
    ///
    /// `authenticated` 为 `false` 时不注入认证头（用于登录本身）。
    async fn send_envelope(
        &self,
        builder: RequestBuilder,
        context: &RequestContext,
        authenticated: bool,
    ) -> Result<(Value, String)> {
        let builder = if authenticated {
            self.apply_auth(builder)
        } else {
            builder
        };
        self.wait_for_rate_limit().await;
        let response = builder.send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(Error::HttpStatus { status, body });
        }
        let envelope: Envelope<Value> =
            serde_json::from_str(&body).map_err(|source| Error::Json {
                source,
                method: context.method.clone(),
                url: context.url.clone(),
                response_body: Some(body.clone()),
            })?;
        let code = ApiStatusCode::from_code(envelope.code);
        if !code.is_success() {
            return Err(Error::Api {
                code,
                kind: InternalErrorKind::from_message(&envelope.message),
                message: envelope.message,
                data: envelope.data,
            });
        }
        Ok((envelope.data, body))
    }

    /// 使用保存的凭据重新登录并更新 token。
    ///
    /// 实现为不依赖任何 feature 门控端点模块的原始登录（`/api/auth/login`），
    /// 且不注入认证头、不触发再次刷新。
    async fn refresh_token(&self) -> Result<()> {
        let Some(Authentication::UsernamePassword {
            username,
            password,
            otp_code,
        }) = self.authentication()
        else {
            return Ok(());
        };
        let body = LoginBody {
            username,
            password,
            otp_code,
        };
        let builder = self.request(Method::POST, "/api/auth/login").json(&body);
        let context = request_context(&builder);
        let (data, response_body) = self.send_envelope(builder, &context, false).await?;
        let login: LoginData = serde_json::from_value(data).map_err(|source| Error::Json {
            source,
            method: context.method.clone(),
            url: context.url.clone(),
            response_body: Some(response_body),
        })?;
        self.replace_token(Some(login.token));
        Ok(())
    }

    /// 该错误是否应当触发重新登录。
    ///
    /// 仅在保存了 [`Authentication::UsernamePassword`] 时对信封 401/403
    /// 或 HTTP 401/403 生效；[`Authentication::Token`] 永不触发。
    fn should_refresh_auth(&self, err: &Error) -> bool {
        if !matches!(
            self.authentication(),
            Some(Authentication::UsernamePassword { .. })
        ) {
            return false;
        }

        match err {
            Error::Api {
                code: ApiStatusCode::Unauthorized | ApiStatusCode::Forbidden,
                ..
            } => true,
            Error::HttpStatus { status, .. } => {
                *status == reqwest::StatusCode::UNAUTHORIZED
                    || *status == reqwest::StatusCode::FORBIDDEN
            }
            _ => false,
        }
    }

    /// 注入当前 token 作为 `Authorization` 头（无 token 时不注入）。
    fn apply_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        match self.token() {
            Some(token) => builder.header(AUTHORIZATION, token),
            None => builder,
        }
    }

    /// 等待限速窗口。
    async fn wait_for_rate_limit(&self) {
        if let Some(rate_limit) = &self.api_request_rate_limit {
            rate_limit.wait().await;
        }
    }

    /// 替换当前 token。
    fn replace_token(&self, token: Option<String>) {
        if let Ok(mut current) = self.token.write() {
            *current = token;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        time::Instant,
    };

    use serde::Deserialize;

    use super::*;
    use crate::test_support::{MockResponse, ok_json, spawn_mock_server};

    #[derive(Debug, Deserialize, PartialEq)]
    struct MeData {
        username: String,
    }

    #[test]
    fn client_normalizes_base_url_and_joins_api_paths() {
        let client = Client::new("https://alist.example/base/").unwrap();
        assert_eq!(client.base_url().as_str(), "https://alist.example/base/");

        let request = client.request(Method::GET, "/api/fs/list").build().unwrap();
        assert_eq!(
            request.url().as_str(),
            "https://alist.example/base/api/fs/list"
        );
        assert_eq!(*request.method(), Method::GET);
    }

    #[test]
    fn request_joins_root_level_paths_like_ping() {
        let client = Client::new("https://alist.example").unwrap();
        let request = client.request(Method::GET, "/ping").build().unwrap();
        assert_eq!(request.url().as_str(), "https://alist.example/ping");
    }

    #[test]
    fn authentication_builders_configure_client_token_state() {
        let client = Client::new("https://alist.example")
            .unwrap()
            .with_authentication(Authentication::Token("token-1".to_string()));
        assert_eq!(client.token().as_deref(), Some("token-1"));
        assert_eq!(
            client.authentication(),
            Some(Authentication::Token("token-1".to_string()))
        );

        let client = Client::new("https://alist.example")
            .unwrap()
            .with_authentication(Authentication::username_password("admin", "password", None));
        assert_eq!(client.token(), None);
        assert_eq!(
            client.authentication(),
            Some(Authentication::UsernamePassword {
                username: "admin".to_string(),
                password: "password".to_string(),
                otp_code: None,
            })
        );

        let mut client = Client::new("https://alist.example").unwrap();
        client.set_authentication(Authentication::Token("token-2".to_string()));
        client.clear_authentication();
        assert_eq!(client.authentication(), None);
        assert_eq!(client.token().as_deref(), Some("token-2"));
    }

    #[test]
    fn api_request_rate_limit_configuration_is_optional() {
        let mut client = Client::new("https://alist.example").unwrap();
        assert_eq!(client.api_request_interval(), None);

        client.set_api_request_interval(Duration::from_millis(250));
        assert_eq!(
            client.api_request_interval(),
            Some(Duration::from_millis(250))
        );

        client.set_api_request_interval(Duration::ZERO);
        assert_eq!(client.api_request_interval(), None);

        let client = Client::new("https://alist.example")
            .unwrap()
            .with_api_request_interval(Duration::from_secs(1));
        assert_eq!(client.api_request_interval(), Some(Duration::from_secs(1)));
    }

    #[tokio::test]
    async fn api_request_rate_limit_delays_consecutive_requests() {
        let body = r#"{"code":200,"message":"success","data":{"username":"admin"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body), ok_json(body)], None).await;
        let client = Client::new(base_url)
            .unwrap()
            .with_api_request_interval(Duration::from_millis(50));

        let started_at = Instant::now();
        let first: MeData = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        let second: MeData = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        assert_eq!(first.username, "admin");
        assert_eq!(second.username, "admin");
        assert!(started_at.elapsed() >= Duration::from_millis(45));
    }

    #[tokio::test]
    async fn execute_decodes_success_envelope() {
        let body = r#"{"code":200,"message":"success","data":{"username":"admin"}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = Client::new(base_url).unwrap();

        let me: MeData = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        assert_eq!(me.username, "admin");
    }

    #[tokio::test]
    async fn execute_null_data_decodes_to_unit_and_option() {
        let body = r#"{"code":200,"message":"success","data":null}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = Client::new(base_url).unwrap();

        let unit: () = client
            .execute(client.request(Method::POST, "/api/auth/2fa/verify"))
            .await
            .unwrap();
        assert_eq!(unit, ());

        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = Client::new(base_url).unwrap();
        let none: Option<MeData> = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        assert_eq!(none, None);
    }

    #[tokio::test]
    async fn execute_maps_logical_error_code_to_api_error() {
        let body = r#"{"code":403,"message":"password is incorrect","data":null}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = Client::new(base_url).unwrap();

        let err = client
            .execute::<MeData>(client.request(Method::GET, "/api/me"))
            .await
            .unwrap_err();
        match err {
            Error::Api {
                code,
                kind,
                message,
                data,
            } => {
                assert_eq!(code, ApiStatusCode::Forbidden);
                assert_eq!(kind, Some(InternalErrorKind::WrongPassword));
                assert_eq!(message, "password is incorrect");
                assert!(data.is_null());
            }
            other => panic!("expected Api, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn execute_maps_http_error_status() {
        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 500 Internal Server Error",
                body: "boom".to_string(),
            }],
            None,
        )
        .await;
        let client = Client::new(base_url).unwrap();

        let err = client
            .execute::<MeData>(client.request(Method::GET, "/api/me"))
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            Error::HttpStatus { status, body }
                if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR && body == "boom"
        ));
    }

    #[tokio::test]
    async fn json_decode_errors_carry_request_context() {
        let body = r#"{"code":200,"message":"success","data":{"unexpected":1}}"#;
        let base_url = spawn_mock_server(vec![ok_json(body)], None).await;
        let client = Client::new(base_url).unwrap();

        let err = client
            .execute::<MeData>(client.request(Method::GET, "/api/me"))
            .await
            .unwrap_err();
        match err {
            Error::Json {
                method,
                url,
                response_body,
                ..
            } => {
                assert_eq!(method, "GET");
                assert!(url.contains("/api/me"), "url 应包含请求路径: {url}");
                assert_eq!(response_body.as_deref(), Some(body));
            }
            other => panic!("expected Json, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn username_password_authentication_refreshes_expired_token() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let responses = vec![
            ok_json(r#"{"code":401,"message":"token expired","data":null}"#),
            ok_json(r#"{"code":200,"message":"success","data":{"token":"fresh-token"}}"#),
            ok_json(r#"{"code":200,"message":"success","data":{"username":"admin"}}"#),
        ];
        let base_url = spawn_mock_server(responses, Some(Arc::clone(&requests))).await;
        let client = Client::new(base_url)
            .unwrap()
            .with_authentication(Authentication::username_password("admin", "password", None));

        let me: MeData = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        assert_eq!(me.username, "admin");
        assert_eq!(client.token().as_deref(), Some("fresh-token"));

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 3);
        assert!(requests[0].contains("GET /api/me "));
        assert!(requests[1].contains("POST /api/auth/login "));
        assert!(requests[1].contains("\"username\":\"admin\""));
        assert!(requests[1].contains("\"password\":\"password\""));
        assert!(requests[2].contains("GET /api/me "));
        assert!(
            requests[2]
                .to_ascii_lowercase()
                .contains("authorization: fresh-token"),
            "重试请求应携带刷新后的 token: {}",
            requests[2]
        );
    }

    #[tokio::test]
    async fn http_unauthorized_status_also_triggers_refresh() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let responses = vec![
            MockResponse {
                status_line: "HTTP/1.1 401 Unauthorized",
                body: "unauthorized".to_string(),
            },
            ok_json(r#"{"code":200,"message":"success","data":{"token":"fresh-token"}}"#),
            ok_json(r#"{"code":200,"message":"success","data":{"username":"admin"}}"#),
        ];
        let base_url = spawn_mock_server(responses, Some(Arc::clone(&requests))).await;
        let client = Client::new(base_url)
            .unwrap()
            .with_authentication(Authentication::username_password("admin", "password", None));

        let me: MeData = client
            .execute(client.request(Method::GET, "/api/me"))
            .await
            .unwrap();
        assert_eq!(me.username, "admin");

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 3);
        assert!(
            requests[2]
                .to_ascii_lowercase()
                .contains("authorization: fresh-token")
        );
    }

    #[tokio::test]
    async fn token_authentication_does_not_refresh_on_401() {
        let requests = Arc::new(Mutex::new(Vec::new()));
        let responses = vec![ok_json(
            r#"{"code":401,"message":"token expired","data":null}"#,
        )];
        let base_url = spawn_mock_server(responses, Some(Arc::clone(&requests))).await;
        let client = Client::new(base_url)
            .unwrap()
            .with_authentication(Authentication::Token("token-1".to_string()));

        let err = client
            .execute::<MeData>(client.request(Method::GET, "/api/me"))
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            Error::Api {
                code: ApiStatusCode::Unauthorized,
                ..
            }
        ));
        assert_eq!(
            requests.lock().unwrap().len(),
            1,
            "Token 凭据不应触发重新登录"
        );
        assert_eq!(client.token().as_deref(), Some("token-1"));
    }

    #[tokio::test]
    async fn execute_text_returns_plain_body() {
        let base_url = spawn_mock_server(
            vec![MockResponse {
                status_line: "HTTP/1.1 200 OK",
                body: "pong".to_string(),
            }],
            None,
        )
        .await;
        let client = Client::new(base_url).unwrap();

        let text = client
            .execute_text(client.request(Method::GET, "/ping"))
            .await
            .unwrap();
        assert_eq!(text, "pong");
    }
}
