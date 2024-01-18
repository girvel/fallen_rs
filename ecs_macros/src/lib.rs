extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn deconstruct(input: TokenStream) -> TokenStream {
    let result = format!("{:?}", input);
    quote! { #result }.into()
}
