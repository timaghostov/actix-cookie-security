use proc_macro::TokenStream as Stream;
use proc_macro2::Group;
use proc_macro2::TokenStream;

use quote::ToTokens;

use syn::parse_macro_input;
use syn::parse_quote;
use syn::token::Brace;
use syn::Block;
use syn::Ident;
use syn::ImplItemFn;
use syn::ReturnType;
use syn::Stmt;

mod parser;

#[derive(Debug)]
struct Attibutes {
    session: Ident,
    roles: Group,
    unauthorized_function: Option<Ident>,
    forbidden_function: Option<Ident>,
}

pub fn secured_impl(attributes_stream: Stream, function_stream: Stream) -> Stream {
    let Attibutes {
        session,
        roles,
        unauthorized_function,
        forbidden_function,
    } = parse_macro_input!(attributes_stream);

    let ImplItemFn {
        attrs,
        vis,
        defaultness,
        mut sig,
        block,
    } = parse_macro_input!(function_stream);

    let unauthorized_code = build_unauthorized_block(unauthorized_function);

    let forbidden_code = build_forbidden_block(forbidden_function);

    let braces = block.brace_token;

    let function_body =
        build_new_function_body(session, roles, unauthorized_code, forbidden_code, block);

    sig.output = build_output_type();

    ImplItemFn {
        attrs,
        vis,
        defaultness,
        sig,
        block: wrap_in_braces(function_body, braces),
    }
    .into_token_stream()
    .into()
}

fn build_output_type() -> ReturnType {
    syn::parse_str(" -> actix_cookie_security::HttpResult").unwrap()
}

fn build_unauthorized_block(unauthorized_function: Option<Ident>) -> TokenStream {
    match unauthorized_function {
        Some(function) => quote::quote! {
            actix_cookie_security::WrapperHttpResult::from(#function().await).0
        },
        None => quote::quote! {
            actix_cookie_security::WrapperHttpResult::from(HttpResponse::Unauthorized().finish()).0
        },
    }
}

fn build_forbidden_block(forbidden_function: Option<Ident>) -> TokenStream {
    match forbidden_function {
        Some(function) => quote::quote! {
            actix_cookie_security::WrapperHttpResult::from(#function().await).0
        },
        None => quote::quote! {
            actix_cookie_security::WrapperHttpResult::from(HttpResponse::Forbidden().finish()).0
        },
    }
}

fn build_new_function_body(
    session: Ident,
    roles: Group,
    unauthorized_code: TokenStream,
    forbidden_code: TokenStream,
    original_code: Block,
) -> Vec<Stmt> {
    parse_quote! {

        let is_authorized = #session.is_authorized();
        let has_access = #session.has_access(&#roles);

        let mut result = if is_authorized && has_access {
            actix_cookie_security::HttpResult::from(#original_code)
        } else if !is_authorized {
            #unauthorized_code
        } else {
            #forbidden_code
        };

        if let Ok(mut response) = result.as_mut() {
            response
                .add_cookie(&#session.cookie())
                .map_err(actix_cookie_security::ApplicationError::from)?;
        }

        result
    }
}

fn wrap_in_braces(function_body: Vec<Stmt>, braces: Brace) -> Block {
    Block {
        brace_token: braces,
        stmts: function_body,
    }
}
