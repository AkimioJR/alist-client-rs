//! alist-client 内部派生宏。
//!
//! 仅服务于 [`alist-client`](https://crates.io/crates/alist-client) 主 crate，
//! 不建议单独使用；宏的属性面与生成规则见主 crate 的 `docs/design.md`。

use proc_macro::TokenStream;

mod endpoint_request;

/// 为 AList 端点请求构建器派生 `build_request`、`send`/`send_raw`、
/// [`core::future::IntoFuture`]、可选字段 setter 与（按需）`into_stream`。
///
/// 容器属性 `#[endpoint(...)]` 支持键：
///
/// - `method = <GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS>`（必填）
/// - `path = "/api/fs/list"`（必填，站点根路径起算的完整 API 路径）
/// - `model = <响应数据类型>`（必填，`send`/`IntoFuture` 的解码目标）
/// - `into_stream = true`（可选，生成自动翻页流方法）
/// - `stream_item = <流元素类型>`（可选，仅在 `into_stream` 时生效）
///
/// 字段属性：
///
/// - `#[endpoint(skip)]`：排除该字段（`client` 字段必须标记）；
/// - `#[query]`：字段参与 URL 查询串，其余字段参与 JSON 请求体。
#[proc_macro_derive(EndpointRequest, attributes(endpoint, query))]
pub fn derive_endpoint_request(input: TokenStream) -> TokenStream {
    endpoint_request::expand(input)
}
