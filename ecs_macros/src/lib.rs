use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
