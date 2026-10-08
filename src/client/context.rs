//! 请求上下文辅助模型。

use reqwest::RequestBuilder;

/// 请求上下文（用于 JSON 错误的最佳努力定位）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RequestContext {
    pub(super) method: String,
    pub(super) url: String,
}

impl RequestContext {
    /// 从请求构建器提取方法与 URL（流式请求体等无法克隆时返回占位符）。
    pub(super) fn from_builder(builder: &RequestBuilder) -> Self {
        builder
            .try_clone()
            .and_then(|builder| builder.build().ok())
            .map(|request| RequestContext {
                method: request.method().to_string(),
                url: request.url().to_string(),
            })
            .unwrap_or_else(|| RequestContext {
                method: "?".to_string(),
                url: "?".to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};

    use super::*;

    #[test]
    fn request_context_extracts_method_and_url() {
        let client = Client::new();
        let builder = client.request(Method::GET, "https://alist.example/api/fs/list");
        let context = RequestContext::from_builder(&builder);
        assert_eq!(context.method, "GET");
        assert_eq!(context.url, "https://alist.example/api/fs/list");
    }
}
