extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn item(_: TokenStream, tokens: TokenStream) -> TokenStream {
    let original = tokens.clone();
    let input = parse_macro_input!(tokens as DeriveInput);

    expand::readonly(input)
        .unwrap_or_else(|e| {
            let original = proc_macro2::TokenStream::from(original);
            let compile_error = e.to_compile_error();
            quote! {
                #original
                #compile_error
            }
        })
        .into()
}
