//! Error types for AList API responses and client-side failures.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// AList logical status codes carried in the JSON response envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiStatusCode {
    /// Successful response.
    Ok,
    /// Archive password is wrong or archive metadata is not ready.
    Accepted,
    /// Bad request or validation error.
    BadRequest,
    /// Authentication required or token invalid.
    Unauthorized,
    /// 2FA code required/invalid in login flow.
    TwoFactor,
    /// Permission denied.
    Forbidden,
    /// Resource not found.
    NotFound,
    /// Method or operation not allowed.
    MethodNotAllowed,
    /// Login throttled.
    TooManyRequests,
    /// Server-side error.
    InternalServerError,
    /// Any server code not known to this client version.
    Unknown(i32),
}

impl ApiStatusCode {
    /// Convert a raw AList response code into a typed status.
    pub fn from_code(code: i32) -> Self {
        match code {
            200 => Self::Ok,
            202 => Self::Accepted,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::TwoFactor,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            429 => Self::TooManyRequests,
            500 => Self::InternalServerError,
            other => Self::Unknown(other),
        }
    }

    /// Return the numeric code used by AList.
    pub fn as_i32(self) -> i32 {
        match self {
            Self::Ok => 200,
            Self::Accepted => 202,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::TwoFactor => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::TooManyRequests => 429,
            Self::InternalServerError => 500,
            Self::Unknown(code) => code,
        }
    }

    /// Whether this code represents a successful API response.
    pub fn is_success(self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<i32> for ApiStatusCode {
    fn from(value: i32) -> Self {
        Self::from_code(value)
    }
}

/// Stable names for constant errors in `alist/internal/errs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalErrorKind {
    /// `not implement`.
    NotImplement,
    /// `not support`.
    NotSupport,
    /// `access using relative path is not allowed`.
    RelativePath,
    /// `can't move files between two storages, try to copy`.
    MoveBetweenTwoStorages,
    /// `upload not supported`.
    UploadNotSupported,
    /// `meta not found`.
    MetaNotFound,
    /// `storage not found`.
    StorageNotFound,
    /// `upload/download stream incomplete, possible network issue`.
    StreamIncomplete,
    /// `StreamPeekFail`.
    StreamPeekFail,
    /// `unknown archive format`.
    UnknownArchiveFormat,
    /// `wrong archive password`.
    WrongArchivePassword,
    /// `driver extraction not supported`.
    DriverExtractNotSupported,
    /// `object not found`.
    ObjectNotFound,
    /// `not a folder`.
    NotFolder,
    /// `not a file`.
    NotFile,
    /// `username is empty`.
    EmptyUsername,
    /// `password is empty`.
    EmptyPassword,
    /// `password is incorrect`.
    WrongPassword,
    /// `cannot delete admin or guest`.
    DeleteAdminOrGuest,
    /// `search not available`.
    SearchNotAvailable,
    /// `build index is running, please try later`.
    BuildIndexIsRunning,
    /// `permission denied`.
    PermissionDenied,
    /// `invalid file name`.
    InvalidName,
    /// `empty token`.
    EmptyToken,
    /// `link is dir`.
    LinkIsDir,
    /// `cannot modify admin role`.
    ErrChangeDefaultRole,
    /// `too many active devices`.
    TooManyDevices,
    /// `session inactive`.
    SessionInactive,
}

impl InternalErrorKind {
    /// Classify an AList error message by matching the constant error text.
    ///
    /// AList responses do not include a symbolic error id, so this intentionally
    /// matches exact substrings from `alist/internal/errs`.
    pub fn from_message(message: &str) -> Option<Self> {
        let normalized = message.to_ascii_lowercase();
        const MAPPINGS: &[(&str, InternalErrorKind)] = &[
            (
                "upload not supported",
                InternalErrorKind::UploadNotSupported,
            ),
            (
                "driver extraction not supported",
                InternalErrorKind::DriverExtractNotSupported,
            ),
            ("not implement", InternalErrorKind::NotImplement),
            ("not support", InternalErrorKind::NotSupport),
            (
                "access using relative path is not allowed",
                InternalErrorKind::RelativePath,
            ),
            (
                "can't move files between two storages, try to copy",
                InternalErrorKind::MoveBetweenTwoStorages,
            ),
            ("meta not found", InternalErrorKind::MetaNotFound),
            ("storage not found", InternalErrorKind::StorageNotFound),
            (
                "upload/download stream incomplete, possible network issue",
                InternalErrorKind::StreamIncomplete,
            ),
            ("streampeekfail", InternalErrorKind::StreamPeekFail),
            (
                "unknown archive format",
                InternalErrorKind::UnknownArchiveFormat,
            ),
            (
                "wrong archive password",
                InternalErrorKind::WrongArchivePassword,
            ),
            ("object not found", InternalErrorKind::ObjectNotFound),
            ("not a folder", InternalErrorKind::NotFolder),
            ("not a file", InternalErrorKind::NotFile),
            ("username is empty", InternalErrorKind::EmptyUsername),
            ("password is empty", InternalErrorKind::EmptyPassword),
            ("password is incorrect", InternalErrorKind::WrongPassword),
            (
                "cannot delete admin or guest",
                InternalErrorKind::DeleteAdminOrGuest,
            ),
            (
                "search not available",
                InternalErrorKind::SearchNotAvailable,
            ),
            (
                "build index is running, please try later",
                InternalErrorKind::BuildIndexIsRunning,
            ),
            ("permission denied", InternalErrorKind::PermissionDenied),
            ("invalid file name", InternalErrorKind::InvalidName),
            ("empty token", InternalErrorKind::EmptyToken),
            ("link is dir", InternalErrorKind::LinkIsDir),
            (
                "cannot modify admin role",
                InternalErrorKind::ErrChangeDefaultRole,
            ),
            ("too many active devices", InternalErrorKind::TooManyDevices),
            ("session inactive", InternalErrorKind::SessionInactive),
        ];

        MAPPINGS
            .iter()
            .find_map(|(needle, kind)| normalized.contains(needle).then_some(*kind))
    }
}

/// Result alias used by the client.
pub type Result<T> = std::result::Result<T, ClientError>;

/// All errors produced by this crate.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Failed to build or parse a URL.
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    /// HTTP transport error from reqwest.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// JSON serialization or deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// I/O error, typically from upload body construction.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Non-success HTTP status before AList JSON decoding.
    #[error("http status {status}: {body}")]
    HttpStatus {
        /// HTTP status code.
        status: reqwest::StatusCode,
        /// Response body text.
        body: String,
    },
    /// AList JSON envelope had a non-200 logical code.
    #[error("alist api error {code:?}: {message}")]
    Api {
        /// Typed AList status code.
        code: ApiStatusCode,
        /// Server message.
        message: String,
        /// Best-effort classification from `alist/internal/errs`.
        kind: Option<InternalErrorKind>,
        /// Raw `data` from the error envelope.
        data: Value,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_status_code_maps_known_and_unknown_codes() {
        assert_eq!(ApiStatusCode::from_code(200), ApiStatusCode::Ok);
        assert_eq!(ApiStatusCode::from_code(202), ApiStatusCode::Accepted);
        assert_eq!(ApiStatusCode::from_code(400), ApiStatusCode::BadRequest);
        assert_eq!(ApiStatusCode::from_code(401), ApiStatusCode::Unauthorized);
        assert_eq!(ApiStatusCode::from_code(402), ApiStatusCode::TwoFactor);
        assert_eq!(ApiStatusCode::from_code(403), ApiStatusCode::Forbidden);
        assert_eq!(ApiStatusCode::from_code(404), ApiStatusCode::NotFound);
        assert_eq!(
            ApiStatusCode::from_code(405),
            ApiStatusCode::MethodNotAllowed
        );
        assert_eq!(
            ApiStatusCode::from_code(429),
            ApiStatusCode::TooManyRequests
        );
        assert_eq!(
            ApiStatusCode::from_code(500),
            ApiStatusCode::InternalServerError
        );
        assert_eq!(ApiStatusCode::from_code(599), ApiStatusCode::Unknown(599));
        assert_eq!(ApiStatusCode::Forbidden.as_i32(), 403);
    }

    #[test]
    fn internal_error_kind_covers_alist_internal_errs_messages() {
        let cases = [
            ("not implement", InternalErrorKind::NotImplement),
            ("not support", InternalErrorKind::NotSupport),
            (
                "access using relative path is not allowed",
                InternalErrorKind::RelativePath,
            ),
            (
                "can't move files between two storages, try to copy",
                InternalErrorKind::MoveBetweenTwoStorages,
            ),
            (
                "upload not supported",
                InternalErrorKind::UploadNotSupported,
            ),
            ("meta not found", InternalErrorKind::MetaNotFound),
            ("storage not found", InternalErrorKind::StorageNotFound),
            (
                "upload/download stream incomplete, possible network issue",
                InternalErrorKind::StreamIncomplete,
            ),
            ("StreamPeekFail", InternalErrorKind::StreamPeekFail),
            (
                "unknown archive format",
                InternalErrorKind::UnknownArchiveFormat,
            ),
            (
                "wrong archive password",
                InternalErrorKind::WrongArchivePassword,
            ),
            (
                "driver extraction not supported",
                InternalErrorKind::DriverExtractNotSupported,
            ),
            ("object not found", InternalErrorKind::ObjectNotFound),
            ("not a folder", InternalErrorKind::NotFolder),
            ("not a file", InternalErrorKind::NotFile),
            ("username is empty", InternalErrorKind::EmptyUsername),
            ("password is empty", InternalErrorKind::EmptyPassword),
            ("password is incorrect", InternalErrorKind::WrongPassword),
            (
                "cannot delete admin or guest",
                InternalErrorKind::DeleteAdminOrGuest,
            ),
            (
                "search not available",
                InternalErrorKind::SearchNotAvailable,
            ),
            (
                "build index is running, please try later",
                InternalErrorKind::BuildIndexIsRunning,
            ),
            ("permission denied", InternalErrorKind::PermissionDenied),
            ("invalid file name", InternalErrorKind::InvalidName),
            ("empty token", InternalErrorKind::EmptyToken),
            ("link is dir", InternalErrorKind::LinkIsDir),
            (
                "cannot modify admin role",
                InternalErrorKind::ErrChangeDefaultRole,
            ),
            ("too many active devices", InternalErrorKind::TooManyDevices),
            ("session inactive", InternalErrorKind::SessionInactive),
        ];

        for (message, expected) in cases {
            assert_eq!(InternalErrorKind::from_message(message), Some(expected));
        }
    }
}
