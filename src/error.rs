//! AList 客户端错误类型。
//!
//! AList 的 JSON 响应（[`Response`](crate::schema::common::Response)）承载业务状态码，HTTP 状态码仅反映传输层结果，
//! 因此错误分类同时覆盖两个层面：[`Error::HttpStatus`]（HTTP 非 2xx）与
//! [`Error::Api`]（HTTP 200 但响应 `code` 非 200）。

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// AList JSON 响应中的逻辑状态码。
///
/// 参考实现见 `examples/alist/server/common/resp.go` 与 `internal/errs`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiStatusCode {
    /// 成功响应（响应 `code` 为 `200` 或 `0`）。
    Ok,
    /// 归档密码错误或归档元信息尚未就绪（`202`）。
    Accepted,
    /// 请求参数或校验错误（`400`）。
    BadRequest,
    /// 需要认证或 token 无效（`401`）。
    Unauthorized,
    /// 登录流程中需要/校验失败的两步验证码（`402`）。
    TwoFactor,
    /// 权限不足（`403`）。
    Forbidden,
    /// 资源不存在（`404`）。
    NotFound,
    /// 方法或操作不被允许（`405`）。
    MethodNotAllowed,
    /// 登录请求被限流（`429`）。
    TooManyRequests,
    /// 服务端内部错误（`500`）。
    InternalServerError,
    /// 当前客户端版本未知的其他状态码。
    Unknown(i32),
}

impl ApiStatusCode {
    /// 将 AList 响应中的原始状态码转换为类型化状态。
    pub fn from_code(code: i32) -> Self {
        match code {
            0 | 200 => Self::Ok,
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

    /// 返回 AList 使用的数字状态码。
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

    /// 该状态码是否表示成功的 API 响应。
    pub fn is_success(self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<i32> for ApiStatusCode {
    fn from(value: i32) -> Self {
        Self::from_code(value)
    }
}

/// `alist/internal/errs` 中常量错误信息的稳定命名。
///
/// AList 响应不携带符号化错误 ID，因此只能按 `internal/errs` 的常量文本做子串匹配分类。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InternalErrorKind {
    /// `not implement`。
    NotImplement,
    /// `not support`。
    NotSupport,
    /// `access using relative path is not allowed`。
    RelativePath,
    /// `can't move files between two storages, try to copy`。
    MoveBetweenTwoStorages,
    /// `upload not supported`。
    UploadNotSupported,
    /// `meta not found`。
    MetaNotFound,
    /// `storage not found`。
    StorageNotFound,
    /// `upload/download stream incomplete, possible network issue`。
    StreamIncomplete,
    /// `StreamPeekFail`。
    StreamPeekFail,
    /// `unknown archive format`。
    UnknownArchiveFormat,
    /// `wrong archive password`。
    WrongArchivePassword,
    /// `driver extraction not supported`。
    DriverExtractNotSupported,
    /// `object not found`。
    ObjectNotFound,
    /// `not a folder`。
    NotFolder,
    /// `not a file`。
    NotFile,
    /// `username is empty`。
    EmptyUsername,
    /// `password is empty`。
    EmptyPassword,
    /// `password is incorrect`。
    WrongPassword,
    /// `cannot delete admin or guest`。
    DeleteAdminOrGuest,
    /// `search not available`。
    SearchNotAvailable,
    /// `build index is running, please try later`。
    BuildIndexIsRunning,
    /// `permission denied`。
    PermissionDenied,
    /// `invalid file name`。
    InvalidName,
    /// `empty token`。
    EmptyToken,
    /// `link is dir`。
    LinkIsDir,
    /// `cannot modify admin role`。
    ErrChangeDefaultRole,
    /// `too many active devices`。
    TooManyDevices,
    /// `session inactive`。
    SessionInactive,
}

impl InternalErrorKind {
    /// 按常量错误文本对 AList 的错误消息做尽力分类。
    ///
    /// 匹配规则为大小写不敏感的子串匹配，命中多个时取映射表中最先出现的条目。
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

/// crate 统一的 [`Result`](std::result::Result) 别名。
pub type Result<T> = std::result::Result<T, Error>;

/// 本 crate 产生的全部错误。
#[derive(Debug, Error)]
pub enum Error {
    /// URL 构建或解析失败。
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    /// reqwest 传输层错误。
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// JSON 序列化/反序列化失败，附带最佳努力的请求上下文。
    #[error("json error at {method} {url}: {source}; response_body={response_body:?}")]
    Json {
        /// 底层 JSON 解析错误。
        source: serde_json::Error,
        /// HTTP 方法（无法还原时为 `"?"`）。
        method: String,
        /// 请求 URL（无法还原时为 `"?"`）。
        url: String,
        /// 可用时为原始响应体。
        response_body: Option<String>,
    },
    /// I/O 错误，通常来自上传请求体构建。
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// 响应解码前的非 2xx HTTP 状态。
    #[error("http status {status}: {body}")]
    HttpStatus {
        /// HTTP 状态码。
        status: reqwest::StatusCode,
        /// 响应体文本。
        body: String,
    },
    /// HTTP 200 但 AList 响应 `code` 非 200。
    #[error("alist api error {code:?}: {message}")]
    Api {
        /// 类型化的 AList 状态码。
        code: ApiStatusCode,
        /// 服务端消息。
        message: String,
        /// 按 `alist/internal/errs` 常量文本做的尽力分类。
        kind: Option<InternalErrorKind>,
        /// 错误响应中的原始 `data`。
        data: Value,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_status_code_maps_known_and_unknown_codes() {
        assert_eq!(ApiStatusCode::from_code(0), ApiStatusCode::Ok);
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
        assert!(ApiStatusCode::Ok.is_success());
        assert!(!ApiStatusCode::Forbidden.is_success());
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
        assert_eq!(InternalErrorKind::from_message("其他未知错误"), None);
    }
}
