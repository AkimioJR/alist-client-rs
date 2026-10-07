//! 测试辅助（仅 `cfg(test)` 构建）：手搓 tokio TcpListener mock 服务器。
//!
//! 沿用旧 `src/tests.rs` 的方式：每个 accept 读取一次请求原文，
//! 按 `responses` 顺序返回预设响应，可选记录请求原文供断言使用。

use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

/// 一条预设响应。
pub struct MockResponse {
    /// HTTP 状态行，例如 `"HTTP/1.1 200 OK"`。
    pub status_line: &'static str,
    /// 响应体。
    pub body: String,
}

/// 构造 HTTP 200 JSON 响应。
pub fn ok_json(body: impl Into<String>) -> MockResponse {
    MockResponse {
        status_line: "HTTP/1.1 200 OK",
        body: body.into(),
    }
}

/// 启动 mock 服务器。
///
/// 对第 n 个连接返回 `responses[n]`（越界时复用最后一条）；
/// `requests` 提供时按顺序记录每个请求的原文。
/// 返回基址（如 `http://127.0.0.1:8080`）。
pub async fn spawn_mock_server(
    responses: Vec<MockResponse>,
    requests: Option<Arc<Mutex<Vec<String>>>>,
) -> String {
    assert!(!responses.is_empty(), "至少需要一条预设响应");
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let mut index = 0usize;
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let response = responses
                .get(index)
                .unwrap_or_else(|| responses.last().expect("已检查非空"));
            index += 1;

            let mut buffer = vec![0u8; 8192];
            let n = stream.read(&mut buffer).await.unwrap();
            if let Some(requests) = &requests {
                requests
                    .lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&buffer[..n]).to_string());
            }

            let head = format!(
                "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                response.status_line,
                response.body.len()
            );
            stream.write_all(head.as_bytes()).await.unwrap();
            stream.write_all(response.body.as_bytes()).await.unwrap();
            // stream 在迭代末尾 drop，关闭连接，确保客户端不复用连接
        }
    });

    format!("http://{addr}")
}
