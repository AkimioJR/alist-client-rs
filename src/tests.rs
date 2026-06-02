use crate::{Client, UploadPut};

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
