use proc_macro::TokenStream;

mod implement;

use implement::secured::secured_impl;

#[proc_macro_attribute]
pub fn secured(attributes: TokenStream, input: TokenStream) -> TokenStream {
    secured_impl(attributes, input)
}
