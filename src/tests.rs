use crate::*;
use serde_json::{Map, Value};

fn object_json() -> Value {
    serde_json::json!({
        "id": "obj-id",
        "path": "/movies/demo.mkv",
        "virtual_path": "/demo.mkv",
        "name": "demo.mkv",
        "size": 393090,
        "is_dir": false,
        "modified": "2023-07-19T09:48:13.695585868+08:00",
        "created": "2023-07-19T09:48:13.695585868+08:00",
        "sign": "signed",
        "thumb": "thumb",
        "type": 2,
        "hashinfo": "md5:abc",
        "hash_info": { "md5": "abc" },
        "storage_class": "STANDARD"
    })
}

fn object_map() -> Map<String, Value> {
    object_json().as_object().unwrap().clone()
}

fn object_with(mut extra: Map<String, Value>) -> Value {
    let mut object = object_map();
    object.append(&mut extra);
    Value::Object(object)
}

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

#[test]
fn api_response_deserializes_success_and_error_envelopes() {
    let ok: ApiResponse<LoginResp> = serde_json::from_value(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": { "token": "abc", "device_key": "dev" }
    }))
    .unwrap();
    assert_eq!(ok.data.token, "abc");
    assert_eq!(ok.data.device_key.as_deref(), Some("dev"));

    let err: ApiResponse<Value> = serde_json::from_value(serde_json::json!({
        "code": 403,
        "message": "permission denied",
        "data": null
    }))
    .unwrap();
    assert_eq!(ApiStatusCode::from_code(err.code), ApiStatusCode::Forbidden);
    assert_eq!(
        InternalErrorKind::from_message(&err.message),
        Some(InternalErrorKind::PermissionDenied)
    );
}

#[test]
fn obj_resp_deserializes_current_fs_shape() {
    let obj: ObjResp = serde_json::from_value(object_json()).unwrap();
    assert_eq!(obj.name, "demo.mkv");
    assert_eq!(obj.obj_type, 2);
    assert_eq!(obj.storage_class.as_deref(), Some("STANDARD"));
    assert!(obj.hash_info.is_some());
}

#[test]
fn fs_list_resp_matches_current_api_shape() {
    let item = object_with(Map::from_iter([("label_list".to_string(), Value::Null)]));
    let resp: ApiResponse<FsListResp> = serde_json::from_value(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "content": [item],
            "total": 1,
            "filtered_total": 1,
            "page": 1,
            "per_page": 0,
            "has_more": false,
            "pages_total": 1,
            "readme": "",
            "header": "",
            "write": true,
            "provider": "Local"
        }
    }))
    .unwrap();

    assert_eq!(resp.data.content.len(), 1);
    assert_eq!(resp.data.provider, "Local");
    assert!(resp.data.write);
}

#[test]
fn archive_meta_resp_deserializes_recursive_tree() {
    let child = object_with(Map::from_iter([(
        "children".to_string(),
        Value::Array(Vec::new()),
    )]));
    let root = object_with(Map::from_iter([(
        "children".to_string(),
        Value::Array(vec![child]),
    )]));
    let resp: ApiResponse<ArchiveMetaResp> = serde_json::from_value(serde_json::json!({
        "code": 200,
        "message": "success",
        "data": {
            "comment": "archive comment",
            "encrypted": false,
            "content": [root],
            "sort": { "order_by": "name" },
            "raw_url": "https://example.test/ad/demo.zip",
            "sign": "archive-sign"
        }
    }))
    .unwrap();

    assert_eq!(resp.data.comment, "archive comment");
    assert_eq!(resp.data.content[0].children.len(), 1);
    assert_eq!(resp.data.sign.as_deref(), Some("archive-sign"));
}

#[test]
fn client_trims_base_url_and_builds_api_paths() {
    let client = Client::new("https://alist.example/base/").unwrap();
    assert_eq!(client.base_url().as_str(), "https://alist.example/base/");
    assert_eq!(
        client.api_url("/fs/list").unwrap().as_str(),
        "https://alist.example/base/api/fs/list"
    );
}

#[test]
fn upload_put_builder_sets_expected_defaults_and_hashes() {
    let upload = UploadPut::new("/dst/demo.txt", "hello")
        .password("secret")
        .overwrite(false)
        .as_task(true)
        .content_type("text/plain")
        .last_modified_millis(1_700_000_000_000)
        .hashes(Some("md5"), Some("sha1"), Some("sha256"));

    assert_eq!(upload.file_path, "/dst/demo.txt");
    assert_eq!(upload.body, bytes::Bytes::from_static(b"hello"));
    assert_eq!(upload.password, "secret");
    assert!(!upload.overwrite);
    assert!(upload.as_task);
    assert_eq!(upload.content_type.as_deref(), Some("text/plain"));
    assert_eq!(upload.last_modified_millis, Some(1_700_000_000_000));
    assert_eq!(upload.md5.as_deref(), Some("md5"));
    assert_eq!(upload.sha1.as_deref(), Some("sha1"));
    assert_eq!(upload.sha256.as_deref(), Some("sha256"));
}
