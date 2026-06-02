use crate::{Authentication, Client, UploadPut};
use std::sync::{Arc, Mutex};
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
fn authentication_builders_configure_client_token_state() {
    let client = Client::with_token("https://alist.example", "token-1").unwrap();
    assert_eq!(client.token().as_deref(), Some("token-1"));
    assert_eq!(
        client.authentication(),
        Some(Authentication::Token("token-1".to_string()))
    );

    let client = Client::with_authentication(
        "https://alist.example",
        Authentication::username_password("admin", "password", None::<String>),
    )
    .unwrap();
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

#[tokio::test]
async fn username_password_authentication_refreshes_expired_token() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let base_url = spawn_refresh_server(Arc::clone(&requests)).await;
    let client = Client::with_authentication(
        base_url,
        Authentication::username_password("admin", "password", None::<String>),
    )
    .unwrap();

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
