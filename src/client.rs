//! Async HTTP client for core AList APIs.

use crate::error::{ApiStatusCode, ClientError, InternalErrorKind, Result};
use crate::models::*;
use bytes::Bytes;
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::{Path, PathBuf};

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
            .request_json(Method::POST, "/auth/login", Some(&req))
            .await?;
        self.set_token(resp.token.clone());
        Ok(resp)
    }

    /// Log in with an AList static password hash using `/api/auth/login/hash`.
    pub async fn login_hash_with(&mut self, req: LoginReq) -> Result<LoginResp> {
        let resp: LoginResp = self
            .request_json(Method::POST, "/auth/login/hash", Some(&req))
            .await?;
        self.set_token(resp.token.clone());
        Ok(resp)
    }

    /// Fetch current user info from `/api/me`.
    pub async fn me(&self) -> Result<MeResp> {
        self.request_json::<(), MeResp>(Method::GET, "/me", None)
            .await
    }

    /// Fetch public site settings from `/api/public/settings`.
    pub async fn public_settings(&self) -> Result<PublicSettings> {
        self.request_json::<(), PublicSettings>(Method::GET, "/public/settings", None)
            .await
    }

    /// List a directory with `/api/fs/list`.
    pub async fn fs_list(&self, req: FsListReq) -> Result<FsListResp> {
        self.request_json(Method::POST, "/fs/list", Some(&req))
            .await
    }

    /// Get object metadata with `/api/fs/get`.
    pub async fn fs_get(&self, req: FsGetReq) -> Result<FsGetResp> {
        self.request_json(Method::POST, "/fs/get", Some(&req)).await
    }

    /// List child directories with `/api/fs/dirs`.
    pub async fn fs_dirs(&self, req: DirsReq) -> Result<Vec<DirResp>> {
        self.request_json(Method::POST, "/fs/dirs", Some(&req))
            .await
    }

    /// Search files with `/api/fs/search`.
    pub async fn fs_search(&self, req: SearchReq) -> Result<PageResp<SearchResp>> {
        self.request_json(Method::POST, "/fs/search", Some(&req))
            .await
    }

    /// Create a directory with `/api/fs/mkdir`.
    pub async fn mkdir(&self, path: impl Into<String>) -> Result<()> {
        let req = MkdirReq { path: path.into() };
        self.request_unit(Method::POST, "/fs/mkdir", Some(&req))
            .await
    }

    /// Rename an object with `/api/fs/rename`.
    pub async fn rename(&self, req: RenameReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/rename", Some(&req))
            .await
    }

    /// Move objects with `/api/fs/move`.
    pub async fn move_items(&self, req: MoveCopyReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/move", Some(&req))
            .await
    }

    /// Copy objects with `/api/fs/copy`.
    pub async fn copy_items(&self, req: MoveCopyReq) -> Result<Option<TasksResp>> {
        self.request_json_nullable(Method::POST, "/fs/copy", Some(&req))
            .await
    }

    /// Remove objects with `/api/fs/remove`.
    pub async fn remove(&self, req: RemoveReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/remove", Some(&req))
            .await
    }

    /// Remove empty directories recursively with `/api/fs/remove_empty_directory`.
    pub async fn remove_empty_directory(&self, src_dir: impl Into<String>) -> Result<()> {
        let req = RemoveEmptyDirectoryReq {
            src_dir: src_dir.into(),
        };
        self.request_unit(Method::POST, "/fs/remove_empty_directory", Some(&req))
            .await
    }

    /// Read archive metadata with `/api/fs/archive/meta`.
    pub async fn archive_meta(&self, req: ArchiveMetaReq) -> Result<ArchiveMetaResp> {
        self.request_json(Method::POST, "/fs/archive/meta", Some(&req))
            .await
    }

    /// List an inner archive directory with `/api/fs/archive/list`.
    pub async fn archive_list(&self, req: ArchiveListReq) -> Result<ArchiveListResp> {
        self.request_json(Method::POST, "/fs/archive/list", Some(&req))
            .await
    }

    /// Decompress archive content with `/api/fs/archive/decompress`.
    pub async fn archive_decompress(&self, req: ArchiveDecompressReq) -> Result<()> {
        self.request_unit(Method::POST, "/fs/archive/decompress", Some(&req))
            .await
    }

    /// Upload bytes through `/api/fs/put`.
    pub async fn upload_put(&self, upload: UploadPut) -> Result<Option<UploadResp>> {
        let url = self.api_url("/fs/put")?;
        let mut builder = self.http.put(url).body(upload.body);
        builder = self.apply_auth(builder);
        builder = builder.header(
            "File-Path",
            urlencoding::encode(&upload.file_path).into_owned(),
        );
        builder = builder.header("Password", upload.password);
        builder = builder.header("Overwrite", upload.overwrite.to_string());
        builder = builder.header("As-Task", upload.as_task.to_string());

        if let Some(content_type) = upload.content_type {
            builder = builder.header("Content-Type", content_type);
        }
        if let Some(last_modified) = upload.last_modified_millis {
            builder = builder.header("Last-Modified", last_modified.to_string());
        }
        if let Some(md5) = upload.md5 {
            builder = builder.header("X-File-Md5", md5);
        }
        if let Some(sha1) = upload.sha1 {
            builder = builder.header("X-File-Sha1", sha1);
        }
        if let Some(sha256) = upload.sha256 {
            builder = builder.header("X-File-Sha256", sha256);
        }

        let response = builder.send().await?;
        self.decode_response_nullable(response).await
    }

    /// Upload a local file by reading it asynchronously into memory.
    pub async fn upload_file_put(
        &self,
        path: impl AsRef<Path>,
        file_path: impl Into<String>,
    ) -> Result<Option<UploadResp>> {
        let path: PathBuf = path.as_ref().to_path_buf();
        let body = tokio::fs::read(path).await?;
        self.upload_put(UploadPut::new(file_path, body)).await
    }

    async fn request_unit<B>(&self, method: Method, path: &str, body: Option<&B>) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let _: Option<Value> = self.request_json_nullable(method, path, body).await?;
        Ok(())
    }

    async fn request_json<B, T>(&self, method: Method, path: &str, body: Option<&B>) -> Result<T>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self.send_json(method, path, body).await?;
        self.decode_response(response).await
    }

    async fn request_json_nullable<B, T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<Option<T>>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        let response = self.send_json(method, path, body).await?;
        self.decode_response_nullable(response).await
    }

    async fn send_json<B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<reqwest::Response>
    where
        B: Serialize + ?Sized,
    {
        let url = self.api_url(path)?;
        let mut builder = self.http.request(method, url);
        builder = self.apply_auth(builder);
        if let Some(body) = body {
            builder = builder.json(body);
        }
        Ok(builder.send().await?)
    }

    async fn decode_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self.decode_response_value(response).await?;
        Ok(serde_json::from_value(value)?)
    }

    async fn decode_response_nullable<T>(&self, response: reqwest::Response) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
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

/// Builder-style payload for `/api/fs/put` uploads.
#[derive(Debug, Clone)]
pub struct UploadPut {
    /// Destination path including file name.
    pub file_path: String,
    /// File bytes or any reqwest-compatible body.
    pub body: Bytes,
    /// Meta password. Empty string means no password.
    pub password: String,
    /// Whether existing files can be overwritten.
    pub overwrite: bool,
    /// Whether AList should upload as a background task.
    pub as_task: bool,
    /// Optional content type header.
    pub content_type: Option<String>,
    /// Optional last-modified timestamp in Unix milliseconds.
    pub last_modified_millis: Option<i64>,
    /// Optional MD5 hash.
    pub md5: Option<String>,
    /// Optional SHA-1 hash.
    pub sha1: Option<String>,
    /// Optional SHA-256 hash.
    pub sha256: Option<String>,
}

impl UploadPut {
    /// Create an upload payload with default headers.
    pub fn new(file_path: impl Into<String>, body: impl Into<Bytes>) -> Self {
        Self {
            file_path: file_path.into(),
            body: body.into(),
            password: String::new(),
            overwrite: true,
            as_task: false,
            content_type: None,
            last_modified_millis: None,
            md5: None,
            sha1: None,
            sha256: None,
        }
    }

    /// Set the meta password header.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    /// Set overwrite behavior.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Set whether AList should upload as a background task.
    pub fn as_task(mut self, as_task: bool) -> Self {
        self.as_task = as_task;
        self
    }

    /// Set content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Set last modified timestamp in Unix milliseconds.
    pub fn last_modified_millis(mut self, millis: i64) -> Self {
        self.last_modified_millis = Some(millis);
        self
    }

    /// Set optional hash headers.
    pub fn hashes<M, S, H>(mut self, md5: Option<M>, sha1: Option<S>, sha256: Option<H>) -> Self
    where
        M: Into<String>,
        S: Into<String>,
        H: Into<String>,
    {
        self.md5 = md5.map(Into::into);
        self.sha1 = sha1.map(Into::into);
        self.sha256 = sha256.map(Into::into);
        self
    }
}
