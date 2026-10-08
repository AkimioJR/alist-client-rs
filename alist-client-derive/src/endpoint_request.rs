//! [`EndpointRequest`](crate::EndpointRequest) 派生宏实现。
//!
//! 从 `#[endpoint(...)]` 容器属性与字段级 `#[query]` / `#[endpoint(skip)]` 属性出发，
//! 为 AList 端点请求构建器生成：
//!
//! 1. `pub(crate) fn build_request()`：拼接 method + path、`#[query]` 查询串与 JSON 请求体；
//! 2. `pub async fn send()` / `pub async fn send_raw<T>()`：经 `Client::execute` 解码响应；
//! 3. [`core::future::IntoFuture`]：`Output = crate::Result<model>`，手写 `Pin<Box<dyn Future>>`；
//! 4. 仅为 `Option` 字段生成消费式链式 setter；
//! 5. `into_stream = true` 时生成 `#[cfg(feature = "into-stream")]` 门控的 `into_stream()`。

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, GenericArgument, Ident, Lifetime, LitBool, LitStr, Meta,
    PathArguments, Token, Type, parse_macro_input,
};

/// `#[endpoint(...)]` 容器属性的解析结果。
#[derive(Debug, Default)]
struct EndpointAttr {
    method: Option<String>,
    path: Option<LitStr>,
    model: Option<Type>,
    into_stream: bool,
    stream_item: Option<Type>,
}

impl EndpointAttr {
    /// 解析单个 `#[endpoint(...)]` 属性；多个属性按顺序合并，后出现的键覆盖先前的。
    fn parse(attr: &Attribute, parsed: &mut EndpointAttr) -> syn::Result<()> {
        if !attr.path().is_ident("endpoint") {
            return Ok(());
        }
        if !matches!(attr.meta, Meta::List(_)) {
            return Err(syn::Error::new_spanned(
                attr,
                "`#[endpoint]` 需要括号参数，例如 `#[endpoint(method = GET, path = \"/api/fs/list\", model = FsListResponse)]`",
            ));
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("method") {
                let value = meta.value()?;
                let method: Ident = value.parse()?;
                let name = method.to_string();
                if !matches!(
                    name.as_str(),
                    "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
                ) {
                    return Err(syn::Error::new(
                        method.span(),
                        format!(
                            "不支持的 HTTP 方法 `{name}`，仅支持 GET/POST/PUT/DELETE/PATCH/HEAD/OPTIONS"
                        ),
                    ));
                }
                parsed.method = Some(name);
                Ok(())
            } else if meta.path.is_ident("path") {
                let value = meta.value()?;
                parsed.path = Some(value.parse()?);
                Ok(())
            } else if meta.path.is_ident("model") {
                let value = meta.value()?;
                parsed.model = Some(value.parse()?);
                Ok(())
            } else if meta.path.is_ident("into_stream") {
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    parsed.into_stream = value.parse::<LitBool>()?.value;
                } else {
                    parsed.into_stream = true;
                }
                Ok(())
            } else if meta.path.is_ident("stream_item") {
                let value = meta.value()?;
                parsed.stream_item = Some(value.parse()?);
                Ok(())
            } else {
                Err(meta.error(
                    "无法识别的 endpoint 属性，支持：method、path、model、into_stream、stream_item",
                ))
            }
        })
    }
}

/// 单个字段的分类结果。
#[derive(Debug, PartialEq, Eq)]
enum FieldRole {
    /// `#[endpoint(skip)]`：客户端载体等，不参与查询串/请求体/setter。
    Skip,
    /// `#[query]`：序列化为 URL 查询参数。
    Query,
    /// 其余字段：序列化为 JSON 请求体。
    Body,
}

/// 字段级属性的解析结果。
#[derive(Debug, Default)]
struct FieldAttrs {
    skip: bool,
    query: bool,
}

/// 分类后的字段信息。
struct RequestField {
    ident: Ident,
    ty: Type,
    role: FieldRole,
    /// 字段上的文档注释，生成 setter 时会自动携带。
    docs: Vec<Attribute>,
    /// 字段类型是否为 `Option<...>` 形式（决定是否生成 setter 与 `skip_serializing_if`）。
    is_option: bool,
    /// `Option<...>` 的内层类型；非 Option 字段为 `None`。
    option_inner: Option<Type>,
}

/// 派生宏入口（供 lib.rs 调用）。
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_endpoint_request(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// 核心展开逻辑，独立成函数以便单元测试。
fn expand_endpoint_request(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    // 当前约定不支持泛型类型/常量参数（生命周期除外）
    for param in &input.generics.params {
        if let syn::GenericParam::Type(_) | syn::GenericParam::Const(_) = param {
            return Err(syn::Error::new_spanned(
                param,
                "EndpointRequest 暂不支持泛型类型/常量参数，仅允许生命周期参数",
            ));
        }
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let lifetime = lifetime_of(&input.generics);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input,
                    "EndpointRequest 仅支持命名字段结构体",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "EndpointRequest 仅支持结构体",
            ));
        }
    };

    // 解析容器属性（允许多个 `#[endpoint(...)]` 合并，供 cfg_attr 使用）
    let mut attr = EndpointAttr::default();
    for a in &input.attrs {
        EndpointAttr::parse(a, &mut attr)?;
    }
    let method = attr
        .method
        .ok_or_else(|| syn::Error::new_spanned(&input.ident, "缺少 `method = <HTTP 方法>` 属性"))?;
    let path = attr
        .path
        .ok_or_else(|| syn::Error::new_spanned(&input.ident, "缺少 `path = \"...\"` 属性"))?;
    let model = attr
        .model
        .ok_or_else(|| syn::Error::new_spanned(&input.ident, "缺少 `model = <类型>` 属性"))?;
    if attr.stream_item.is_some() && !attr.into_stream {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`stream_item` 仅在 `into_stream = true` 时生效",
        ));
    }

    // 分类字段
    let mut request_fields: Vec<RequestField> = Vec::new();
    for field in fields {
        let Some(field_ident) = field.ident.as_ref() else {
            return Err(syn::Error::new_spanned(field, "仅支持命名字段"));
        };
        let attrs = parse_field_attrs(field)?;
        let role = if attrs.skip {
            FieldRole::Skip
        } else if attrs.query {
            FieldRole::Query
        } else {
            FieldRole::Body
        };
        let docs = field
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc"))
            .cloned()
            .collect();
        let option_inner = option_inner(&field.ty);
        request_fields.push(RequestField {
            ident: field_ident.clone(),
            ty: field.ty.clone(),
            role,
            docs,
            is_option: option_inner.is_some(),
            option_inner,
        });
    }

    // client 字段约定：必须存在，且必须标记 `#[endpoint(skip)]`
    let client_field = request_fields
        .iter()
        .find(|f| f.ident == "client")
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &input.ident,
                "缺少 `client` 字段（类型应为 `&'a crate::Client`）",
            )
        })?;
    if client_field.role != FieldRole::Skip {
        return Err(syn::Error::new_spanned(
            &client_field.ident,
            "`client` 字段必须标记 `#[endpoint(skip)]`",
        ));
    }

    let query_fields: Vec<&RequestField> = request_fields
        .iter()
        .filter(|f| f.role == FieldRole::Query)
        .collect();
    let body_fields: Vec<&RequestField> = request_fields
        .iter()
        .filter(|f| f.role == FieldRole::Body)
        .collect();
    let option_fields: Vec<&RequestField> = request_fields
        .iter()
        .filter(|f| f.role != FieldRole::Skip && f.is_option)
        .collect();

    let query_struct = build_query_struct(&query_fields, &lifetime);
    let body_struct = build_body_struct(&body_fields, &lifetime);
    let setters = build_setters(&option_fields);
    let build_request = build_request_impl(&method, &path, &query_fields, &body_fields);
    let send_impl = build_send(&model);
    let into_stream_impl = if attr.into_stream {
        let stream_item = attr.stream_item.ok_or_else(|| {
            syn::Error::new_spanned(
                &input.ident,
                "`into_stream = true` 必须搭配 `stream_item = <元素类型>` 属性",
            )
        })?;
        if !request_fields.iter().any(|f| f.ident == "page") {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "`into_stream = true` 需要结构体存在 `page: Option<i32>` 字段用于翻页",
            ));
        }
        build_into_stream(&model, &stream_item, &lifetime)
    } else {
        quote! {}
    };
    let into_future_impl = build_into_future(
        ident,
        &impl_generics,
        &ty_generics,
        where_clause,
        &model,
        &lifetime,
    );

    Ok(quote! {
        #query_struct

        #body_struct

        impl #impl_generics #ident #ty_generics #where_clause {
            #(#setters)*

            #build_request

            #send_impl

            #into_stream_impl
        }

        #into_future_impl
    })
}

/// 提取结构体声明的第一个生命周期参数；没有声明时使用 `'static`。
fn lifetime_of(generics: &syn::Generics) -> Lifetime {
    generics
        .lifetimes()
        .next()
        .map(|param| param.lifetime.clone())
        .unwrap_or_else(|| syn::parse_quote!('static))
}

/// 解析字段级 `#[endpoint(skip)]` 与 `#[query]` 属性。
fn parse_field_attrs(field: &syn::Field) -> syn::Result<FieldAttrs> {
    let mut attrs = FieldAttrs::default();
    for attr in &field.attrs {
        if attr.path().is_ident("endpoint") {
            if !matches!(attr.meta, Meta::List(_)) {
                return Err(syn::Error::new_spanned(
                    attr,
                    "字段级 `#[endpoint]` 需要写成 `#[endpoint(skip)]`",
                ));
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    attrs.skip = true;
                    Ok(())
                } else {
                    Err(meta.error("字段级 endpoint 属性仅支持 `skip`"))
                }
            })?;
        } else if attr.path().is_ident("query") {
            if !matches!(attr.meta, Meta::Path(_)) {
                return Err(syn::Error::new_spanned(
                    attr,
                    "`#[query]` 目前不支持参数，直接写 `#[query]`",
                ));
            }
            attrs.query = true;
        }
    }
    if attrs.skip && attrs.query {
        return Err(syn::Error::new_spanned(
            field,
            "字段不能同时标记 `#[endpoint(skip)]` 与 `#[query]`",
        ));
    }
    Ok(attrs)
}

/// 若类型为 `Option<T>` 形式，返回内层类型 `T`。
fn option_inner(ty: &Type) -> Option<Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match args.args.iter().next()? {
        GenericArgument::Type(inner) => Some(inner.clone()),
        _ => None,
    }
}

/// 判断内层类型是否为 `String`；String 字段的 setter 接收 `impl Into<String>` 以便直接传 `&str`。
fn is_string_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    type_path.path.segments.last().is_some_and(|segment| {
        segment.ident == "String" && matches!(&segment.arguments, PathArguments::None)
    })
}

/// 生成文件私有查询参数结构体（仅当存在 `#[query]` 字段时）。
fn build_query_struct(
    query_fields: &[&RequestField],
    lifetime: &Lifetime,
) -> proc_macro2::TokenStream {
    if query_fields.is_empty() {
        return quote! {};
    }
    let serde_fields = query_fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        let skip = if field.is_option {
            quote! { #[serde(skip_serializing_if = "::core::option::Option::is_none")] }
        } else {
            quote! {}
        };
        quote! {
            #skip
            #ident: &#lifetime #ty,
        }
    });
    quote! {
        /// 该请求的 URL 查询参数集合，由 EndpointRequest 派生宏根据 `#[query]` 字段自动生成。
        #[derive(::serde::Serialize)]
        struct QueryParams<#lifetime> {
            #(#serde_fields)*
        }
    }
}

/// 生成文件私有 JSON 请求体结构体（仅当存在请求体字段时）。
fn build_body_struct(
    body_fields: &[&RequestField],
    lifetime: &Lifetime,
) -> proc_macro2::TokenStream {
    if body_fields.is_empty() {
        return quote! {};
    }
    let serde_fields = body_fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        let skip = if field.is_option {
            quote! { #[serde(skip_serializing_if = "::core::option::Option::is_none")] }
        } else {
            quote! {}
        };
        quote! {
            #skip
            #ident: &#lifetime #ty,
        }
    });
    quote! {
        /// 该请求的 JSON 请求体，由 EndpointRequest 派生宏根据请求体字段自动生成。
        #[derive(::serde::Serialize)]
        struct RequestBody<#lifetime> {
            #(#serde_fields)*
        }
    }
}

/// 为所有 `Option` 字段生成消费式链式 setter。
fn build_setters(option_fields: &[&RequestField]) -> Vec<proc_macro2::TokenStream> {
    option_fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let docs = &field.docs;
            let inner = field
                .option_inner
                .as_ref()
                .expect("setter 仅针对 Option 字段生成");
            // String 字段接收 `impl Into<String>` 便于直接传 `&str`；
            // 其余类型直接接收内层类型，避免整型字面量在 `impl Into<T>` 下的推断失败
            let (param_ty, assign) = if is_string_type(inner) {
                (
                    quote!(impl ::core::convert::Into<::std::string::String>),
                    quote!(self.#ident = ::core::option::Option::Some(#ident.into());),
                )
            } else {
                (
                    quote!(#inner),
                    quote!(self.#ident = ::core::option::Option::Some(#ident);),
                )
            };
            quote! {
                #(#docs)*
                #[inline]
                pub fn #ident(mut self, #ident: #param_ty) -> Self {
                    #assign
                    self
                }
            }
        })
        .collect()
}

/// 生成 `build_request()`：method + path + 查询串 + JSON 请求体。
fn build_request_impl(
    method: &str,
    path: &LitStr,
    query_fields: &[&RequestField],
    body_fields: &[&RequestField],
) -> proc_macro2::TokenStream {
    let method_ident = Ident::new(method, proc_macro2::Span::call_site());
    let mut stmts = vec![quote! {
        let builder = self
            .client
            .request(::reqwest::Method::#method_ident, #path);
    }];
    if !query_fields.is_empty() {
        let inits = query_fields.iter().map(|field| {
            let ident = &field.ident;
            quote! { #ident: &self.#ident }
        });
        stmts.push(quote! {
            let builder = builder.query(&QueryParams {
                #(#inits,)*
            });
        });
    }
    if !body_fields.is_empty() {
        let inits = body_fields.iter().map(|field| {
            let ident = &field.ident;
            quote! { #ident: &self.#ident }
        });
        stmts.push(quote! {
            let builder = builder.json(&RequestBody {
                #(#inits,)*
            });
        });
    }
    quote! {
        /// 构建请求；`pub(crate)` 允许端点模块在发送前继续加工（例如上传场景附加原始请求体）。
        ///
        /// 认证（`Authorization`）头由 [`Client::execute`](crate::Client::execute) 在发送时注入，此处不含认证信息。
        #[inline]
        pub(crate) fn build_request(&self) -> ::reqwest::RequestBuilder {
            #(#stmts)*
            builder
        }
    }
}

/// 生成 `send()` 与 `send_raw<T>()`（均借用 `&self`，可供 `into_stream` 循环复用构建器）。
fn build_send(model: &Type) -> proc_macro2::TokenStream {
    quote! {
        /// 发送请求并按端点声明模型解码响应。
        ///
        /// # Errors
        ///
        /// 当网络请求失败、AList 返回非成功状态码，或响应 `data` 无法反序列化为目标模型时，返回 [`crate::Error`]。
        #[inline]
        pub async fn send(&self) -> crate::Result<#model> {
            self.client.execute(self.build_request()).await
        }

        /// 发送请求并按调用方指定的类型 `T` 解码响应（例如 [`::serde_json::Value`]）。
        ///
        /// # Errors
        ///
        /// 语义与 [`Self::send`] 一致，仅目标类型不同。
        #[inline]
        pub async fn send_raw<T: ::serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
            self.client.execute(self.build_request()).await
        }
    }
}

/// 生成 `#[cfg(feature = "into-stream")]` 门控的 `into_stream()`。
fn build_into_stream(
    model: &Type,
    stream_item: &Type,
    lifetime: &Lifetime,
) -> proc_macro2::TokenStream {
    quote! {
        /// 将请求构建器转换为自动翻页流（`into-stream` feature）。
        ///
        /// 从当前 `page`（缺省 `1`）开始循环请求，逐条产出响应 `content` 中的元素；
        /// 当某一页 `content` 为空时结束（因此最后一页可能多发出一次确认请求）。
        #[cfg(feature = "into-stream")]
        pub fn into_stream(
            self,
        ) -> ::futures::stream::BoxStream<#lifetime, crate::Result<#stream_item>> {
            ::std::boxed::Box::pin(::async_stream::try_stream! {
                let mut request = self;
                let mut page = request.page.unwrap_or(1);
                loop {
                    request.page = ::core::option::Option::Some(page);
                    let response: #model = request.send().await?;
                    let content = response.content;
                    if content.is_empty() {
                        break;
                    }
                    for item in content {
                        yield item;
                    }
                    page += 1;
                }
            })
        }
    }
}

/// 生成 `IntoFuture` 实现：手写 `Pin<Box<dyn Future>>`，不引入 futures 依赖。
#[allow(clippy::too_many_arguments)]
fn build_into_future(
    ident: &Ident,
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: Option<&syn::WhereClause>,
    model: &Type,
    lifetime: &Lifetime,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics ::core::future::IntoFuture for #ident #ty_generics #where_clause {
            type Output = crate::Result<#model>;
            type IntoFuture = ::core::pin::Pin<
                ::std::boxed::Box<
                    dyn ::core::future::Future<Output = Self::Output> + ::core::marker::Send + #lifetime,
                >,
            >;

            #[inline]
            fn into_future(self) -> Self::IntoFuture {
                ::std::boxed::Box::pin(async move { self.send().await })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    /// 用 `syn::parse2` 从 token 流构造 `DeriveInput`。
    fn parse_struct(tokens: proc_macro2::TokenStream) -> DeriveInput {
        syn::parse2(tokens).expect("测试结构体应可解析")
    }

    /// 归一化 token 渲染文本（折叠空白），便于断言。
    fn normalized(tokens: impl ToTokens) -> String {
        tokens
            .to_token_stream()
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// 收集展开结果中针对原结构体的 `impl` 块里的方法名。
    fn impl_method_names(expanded: &proc_macro2::TokenStream) -> Vec<String> {
        let file: syn::File = syn::parse2(expanded.clone()).expect("展开结果应可解析为文件");
        let mut names = Vec::new();
        for item in file.items {
            if let syn::Item::Impl(item_impl) = item {
                for item in item_impl.items {
                    if let syn::ImplItem::Fn(method) = item {
                        names.push(method.sig.ident.to_string());
                    }
                }
            }
        }
        names
    }

    /// 收集展开结果中生成的结构体名。
    fn struct_names(expanded: &proc_macro2::TokenStream) -> Vec<String> {
        let file: syn::File = syn::parse2(expanded.clone()).expect("展开结果应可解析为文件");
        file.items
            .into_iter()
            .filter_map(|item| match item {
                syn::Item::Struct(item_struct) => Some(item_struct.ident.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn endpoint_attr_parses_method_path_model() {
        let attr: Attribute = syn::parse_quote! {
            #[endpoint(method = POST, path = "/api/fs/list", model = FsListResponse)]
        };
        let mut parsed = EndpointAttr::default();
        EndpointAttr::parse(&attr, &mut parsed).expect("属性应可解析");
        assert_eq!(parsed.method.as_deref(), Some("POST"));
        assert_eq!(
            parsed.path.as_ref().map(LitStr::value).as_deref(),
            Some("/api/fs/list")
        );
        assert!(parsed.model.is_some());
        assert!(!parsed.into_stream);
        assert!(parsed.stream_item.is_none());
    }

    #[test]
    fn endpoint_attr_parses_into_stream_and_stream_item() {
        let attr: Attribute = syn::parse_quote! {
            #[endpoint(into_stream = true, stream_item = Obj)]
        };
        let mut parsed = EndpointAttr::default();
        EndpointAttr::parse(&attr, &mut parsed).expect("属性应可解析");
        assert!(parsed.into_stream);
        assert!(parsed.stream_item.is_some());
    }

    #[test]
    fn endpoint_attr_bare_into_stream_flag() {
        let attr: Attribute = syn::parse_quote! {
            #[endpoint(into_stream)]
        };
        let mut parsed = EndpointAttr::default();
        EndpointAttr::parse(&attr, &mut parsed).expect("属性应可解析");
        assert!(parsed.into_stream);
    }

    #[test]
    fn endpoint_attr_rejects_unknown_key_and_bad_method() {
        let attr: Attribute = syn::parse_quote! {
            #[endpoint(unknown_key = 1)]
        };
        let mut parsed = EndpointAttr::default();
        assert!(EndpointAttr::parse(&attr, &mut parsed).is_err());

        let attr: Attribute = syn::parse_quote! {
            #[endpoint(method = TRACE_ALL)]
        };
        let mut parsed = EndpointAttr::default();
        assert!(EndpointAttr::parse(&attr, &mut parsed).is_err());
    }

    #[test]
    fn option_inner_detects_option_types() {
        let ty: Type = syn::parse_quote!(Option<String>);
        assert!(option_inner(&ty).is_some());
        let ty: Type = syn::parse_quote!(::core::option::Option<i32>);
        assert!(option_inner(&ty).is_some());
        let ty: Type = syn::parse_quote!(String);
        assert!(option_inner(&ty).is_none());
        let ty: Type = syn::parse_quote!(OptionOf<String>);
        assert!(option_inner(&ty).is_none());
    }

    #[test]
    fn expand_generates_setters_build_request_and_into_future() {
        let input = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/admin/meta/list", model = PageResponse<Meta>)]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
                #[query]
                page: Option<i32>,
                path: String,
            }
        });
        let expanded = expand_endpoint_request(input).expect("展开应成功");
        let code = normalized(&expanded);

        let methods = impl_method_names(&expanded);
        assert!(
            methods.contains(&"page".to_string()),
            "应为 Option 字段生成 setter"
        );
        assert!(
            methods.contains(&"build_request".to_string()),
            "应生成 build_request: {code}"
        );
        assert!(methods.contains(&"send".to_string()), "应生成 send");
        assert!(methods.contains(&"send_raw".to_string()), "应生成 send_raw");
        assert!(
            !methods.contains(&"into_stream".to_string()),
            "未声明 into_stream 时不应生成流方法"
        );

        let structs = struct_names(&expanded);
        assert!(
            structs.contains(&"QueryParams".to_string()),
            "应生成查询参数结构体: {code}"
        );
        assert!(
            structs.contains(&"RequestBody".to_string()),
            "应生成请求体结构体: {code}"
        );

        let code = normalized(&expanded);
        assert!(
            code.contains("skip_serializing_if"),
            "Option 字段应跳过 None: {code}"
        );
        assert!(
            code.contains("IntoFuture for Request"),
            "应生成 IntoFuture: {code}"
        );
    }

    #[test]
    fn expand_string_setter_uses_into() {
        let input = parse_struct(quote! {
            #[endpoint(method = POST, path = "/api/fs/mkdir", model = ())]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
                path: String,
                password: Option<String>,
            }
        });
        let expanded = expand_endpoint_request(input).expect("展开应成功");
        let code = normalized(&expanded);
        assert!(
            code.contains("impl :: core :: convert :: Into < :: std :: string :: String >"),
            "String 字段 setter 应接收 impl Into<String>: {code}"
        );
        // 非 String 字段（i32 等）setter 直接接收内层类型
        assert!(
            !code.contains("impl :: core :: convert :: Into < i32 >"),
            "非 String 字段不应经过 Into 转换: {code}"
        );
    }

    #[test]
    fn expand_into_stream_requires_stream_item_and_page() {
        let missing_item = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/x/list", model = PageResponse<Obj>, into_stream = true)]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
                page: Option<i32>,
            }
        });
        assert!(
            expand_endpoint_request(missing_item).is_err(),
            "缺少 stream_item 时应报错"
        );

        let missing_page = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/x/list", model = PageResponse<Obj>, into_stream = true, stream_item = Obj)]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
            }
        });
        assert!(
            expand_endpoint_request(missing_page).is_err(),
            "缺少 page 字段时应报错"
        );
    }

    #[test]
    fn expand_requires_client_field_with_skip() {
        let no_client = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/x", model = Obj)]
            pub struct Request<'a> {
                path: String,
            }
        });
        assert!(
            expand_endpoint_request(no_client).is_err(),
            "缺少 client 字段应报错"
        );

        let unskipped_client = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/x", model = Obj)]
            pub struct Request<'a> {
                client: &'a crate::Client,
            }
        });
        assert!(
            expand_endpoint_request(unskipped_client).is_err(),
            "client 字段未标记 skip 应报错"
        );
    }

    #[test]
    fn expand_rejects_missing_required_attrs() {
        let no_model = parse_struct(quote! {
            #[endpoint(method = GET, path = "/api/x")]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
            }
        });
        assert!(
            expand_endpoint_request(no_model).is_err(),
            "缺少 model 应报错"
        );

        let empty = parse_struct(quote! {
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
            }
        });
        assert!(
            expand_endpoint_request(empty).is_err(),
            "缺少 #[endpoint] 应报错"
        );
    }

    #[test]
    fn expand_empty_body_generates_no_request_body_struct() {
        let input = parse_struct(quote! {
            #[endpoint(method = POST, path = "/api/auth/2fa/generate", model = Generate2FaResponse)]
            pub struct Request<'a> {
                #[endpoint(skip)]
                client: &'a crate::Client,
            }
        });
        let expanded = expand_endpoint_request(input).expect("展开应成功");
        let structs = struct_names(&expanded);
        assert!(
            !structs.contains(&"RequestBody".to_string()),
            "无请求体字段时不应生成 RequestBody"
        );
        assert!(
            !structs.contains(&"QueryParams".to_string()),
            "无查询字段时不应生成 QueryParams"
        );
        let code = normalized(&expanded);
        assert!(
            !code.contains(". json ("),
            "无请求体字段时不应设置 JSON body: {code}"
        );
        assert!(
            !code.contains(". query ("),
            "无查询字段时不应设置 query: {code}"
        );
    }
}
