use crate::client::fs::upload::{UploadForm, UploadPut};
use crate::models::fs::{FsListReq, MoveCopyReq};
use crate::{Authentication, Client, ClientError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

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

#[test]
fn upload_form_builder_sets_expected_defaults() {
    let upload = UploadForm::new("/dst/demo.txt", "demo.txt", "hello")
        .as_task(true)
        .content_type("text/plain");

    assert_eq!(upload.file_path, "/dst/demo.txt");
    assert_eq!(upload.file_name, "demo.txt");
    assert_eq!(upload.body, bytes::Bytes::from_static(b"hello"));
    assert!(upload.as_task);
    assert_eq!(upload.content_type.as_deref(), Some("text/plain"));
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
    let base_url = spawn_me_server(2).await;
    let client = Client::new(base_url)
        .unwrap()
        .with_api_request_interval(Duration::from_millis(50));

    let started_at = Instant::now();
    client.me().await.unwrap();
    client.me().await.unwrap();

    assert!(started_at.elapsed() >= Duration::from_millis(45));
}

#[tokio::test]
async fn json_parse_errors_include_request_and_response_context() {
    let response_body =
        r#"{"code":200,"message":"success","data":{"content":2,"total":0}}"#.to_string();
    let base_url = spawn_static_response_server(response_body.clone()).await;
    let client = Client::new(base_url).unwrap();

    let err = client.fs_list(FsListReq::all("/broken")).await.unwrap_err();

    match err {
        ClientError::JsonWithContext {
            method,
            path,
            request_body,
            response_body: actual_response_body,
            ..
        } => {
            assert_eq!(method, "POST");
            assert_eq!(path, "/fs/list");
            assert!(
                request_body
                    .expect("request body should be captured")
                    .contains("\"path\":\"/broken\"")
            );
            assert_eq!(
                actual_response_body.as_deref(),
                Some(response_body.as_str())
            );
        }
        other => panic!("expected JsonWithContext, got {other:?}"),
    }
}

#[tokio::test]
async fn username_password_authentication_refreshes_expired_token() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let base_url = spawn_refresh_server(Arc::clone(&requests)).await;
    let client = Client::new(base_url)
        .unwrap()
        .with_authentication(Authentication::username_password("admin", "password", None));

    let me = client.me().await.unwrap();

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
            .contains("authorization: fresh-token")
    );
}

#[tokio::test]
async fn query_requests_append_parameters_and_auth_header() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let body = r#"{"code":200,"message":"success","data":{"id":1,"path":"/a","password":"c","p_sub":false,"write":false,"w_sub":false,"hide":"","h_sub":false,"readme":"","r_sub":false}}"#;
    let base_url = spawn_recording_response_server(Arc::clone(&requests), body).await;
    let client = Client::new(base_url)
        .unwrap()
        .with_authentication(Authentication::Token("token-1".to_string()));

    let meta = client.admin_meta_get(1).await.unwrap();

    assert_eq!(meta.id, 1);
    let requests = requests.lock().unwrap();
    assert!(requests[0].contains("GET /api/admin/meta/get?id=1 "));
    assert!(
        requests[0]
            .to_ascii_lowercase()
            .contains("authorization: token-1")
    );
}

#[tokio::test]
async fn empty_body_requests_send_no_json_payload() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let body = r#"{"code":200,"message":"success","data":{"qr":"data:image/png;base64,a","secret":"secret"}}"#;
    let base_url = spawn_recording_response_server(Arc::clone(&requests), body).await;
    let client = Client::new(base_url)
        .unwrap()
        .with_authentication(Authentication::Token("token-1".to_string()));

    let generated = client.generate_2fa().await.unwrap();

    assert_eq!(generated.secret, "secret");
    let requests = requests.lock().unwrap();
    assert!(requests[0].contains("POST /api/auth/2fa/generate "));
    assert!(!requests[0].contains("Content-Type: application/json"));
    assert!(!requests[0].contains("\r\n\r\nnull"));
}

#[tokio::test]
async fn nullable_api_responses_decode_to_none() {
    let body = r#"{"code":200,"message":"success","data":null}"#.to_string();
    let base_url = spawn_static_response_server(body).await;
    let client = Client::new(base_url).unwrap();

    let resp = client
        .copy_items(MoveCopyReq {
            src_dir: "/src".to_string(),
            dst_dir: "/dst".to_string(),
            names: vec!["a.txt".to_string()],
            overwrite: false,
        })
        .await
        .unwrap();

    assert_eq!(resp, None);
}

async fn spawn_refresh_server(requests: Arc<Mutex<Vec<String>>>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        for index in 0..3 {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = vec![0; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]).to_string();
            requests.lock().unwrap().push(request);

            let body = match index {
                0 => r#"{"code":401,"message":"token expired","data":null}"#,
                1 => r#"{"code":200,"message":"success","data":{"token":"fresh-token"}}"#,
                _ => {
                    r#"{"code":200,"message":"success","data":{"id":2,"username":"admin","password":"","base_path":"/","role":[2],"disabled":false,"permission":65535,"sso_id":"","otp":false,"role_names":["admin"],"permissions":[{"path":"/","permission":65535}]}}"#
                }
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
    });

    format!("http://{addr}")
}

async fn spawn_me_server(request_count: usize) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        for _ in 0..request_count {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = vec![0; 4096];
            stream.read(&mut buffer).await.unwrap();

            let body = r#"{"code":200,"message":"success","data":{"id":2,"username":"admin","password":"","base_path":"/","role":[2],"disabled":false,"permission":65535,"sso_id":"","otp":false,"role_names":["admin"],"permissions":[{"path":"/","permission":65535}]}}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
    });

    format!("http://{addr}")
}

async fn spawn_static_response_server(body: String) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0; 4096];
        stream.read(&mut buffer).await.unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });

    format!("http://{addr}")
}

async fn spawn_recording_response_server(
    requests: Arc<Mutex<Vec<String>>>,
    body: &'static str,
) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0; 4096];
        let n = stream.read(&mut buffer).await.unwrap();
        requests
            .lock()
            .unwrap()
            .push(String::from_utf8_lossy(&buffer[..n]).to_string());

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).await.unwrap();
    });

    format!("http://{addr}")
}
