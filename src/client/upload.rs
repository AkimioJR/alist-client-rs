//! Client methods and payload builder for `/api/fs/put` uploads.

use super::Client;
use crate::error::Result;
use crate::models::UploadResp;
use bytes::Bytes;
use std::path::{Path, PathBuf};

impl Client {
    /// Upload bytes through `/api/fs/put`.
    pub async fn upload_put(&self, upload: UploadPut) -> Result<Option<UploadResp>> {
        let response = self.send_upload_put(&upload).await?;
        match self.decode_response_nullable(response).await {
            Ok(resp) => Ok(resp),
            Err(err) if self.should_refresh_auth(&err) => {
                self.refresh_token().await?;
                let response = self.send_upload_put(&upload).await?;
                self.decode_response_nullable(response).await
            }
            Err(err) => Err(err),
        }
    }

    async fn send_upload_put(&self, upload: &UploadPut) -> Result<reqwest::Response> {
        let url = self.api_url("/fs/put")?;
        let mut builder = self.http.put(url).body(upload.body.clone());
        builder = self.apply_auth(builder);
        builder = builder.header(
            "File-Path",
            urlencoding::encode(&upload.file_path).into_owned(),
        );
        builder = builder.header("Password", upload.password.clone());
        builder = builder.header("Overwrite", upload.overwrite.to_string());
        builder = builder.header("As-Task", upload.as_task.to_string());

        if let Some(content_type) = &upload.content_type {
            builder = builder.header("Content-Type", content_type);
        }
        if let Some(last_modified) = upload.last_modified_millis {
            builder = builder.header("Last-Modified", last_modified.to_string());
        }
        if let Some(md5) = &upload.md5 {
            builder = builder.header("X-File-Md5", md5);
        }
        if let Some(sha1) = &upload.sha1 {
            builder = builder.header("X-File-Sha1", sha1);
        }
        if let Some(sha256) = &upload.sha256 {
            builder = builder.header("X-File-Sha256", sha256);
        }

        self.wait_for_rate_limit().await;
        Ok(builder.send().await?)
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
