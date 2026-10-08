//! 客户端侧请求限速实现与测试。

use std::time::Duration;

use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

/// 客户端侧请求限速器。
///
/// 通过记录「下一次允许发送的时刻」串行化请求间隔：
/// 多个并发请求会在锁上排队，依次保持 `interval` 间距。
#[derive(Debug)]
pub(super) struct RequestRateLimit {
    interval: Duration,
    next_request_at: Mutex<Instant>,
}

impl RequestRateLimit {
    pub(super) fn new(interval: Duration) -> Self {
        Self {
            interval,
            next_request_at: Mutex::new(Instant::now()),
        }
    }

    #[must_use]
    pub(super) fn interval(&self) -> Duration {
        self.interval
    }

    /// 阻塞到当前请求的允许发送时刻，并预订下一个时刻。
    pub(super) async fn wait(&self) {
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use reqwest::Method;
    use serde::Deserialize;

    use super::*;
    use crate::{
        Client,
        test_support::{ok_json, spawn_mock_server},
    };

    #[derive(Debug, Deserialize, PartialEq)]
    struct MeData {
        username: String,
    }

    #[tokio::test]
    async fn rate_limit_directly_delays_consecutive_waits() {
        let limiter = RequestRateLimit::new(Duration::from_millis(50));
        assert_eq!(limiter.interval(), Duration::from_millis(50));

        let start = Instant::now();
        limiter.wait().await;
        limiter.wait().await;
        assert!(start.elapsed() >= Duration::from_millis(45));
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
}
