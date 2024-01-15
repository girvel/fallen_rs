extern crate proc_macro;
use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::quote;

#[proc_macro]
pub fn entity(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, generics, ..
    } = syn::parse(input).unwrap();

    println!("ident = {:?}\ndata = {:?}\ngenerics = {:?}", ident, data, generics);

    quote!{
        
    }.into()
}
