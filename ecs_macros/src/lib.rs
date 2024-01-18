use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemStruct, Fields, FnArg, PatType, PatIdent, Pat};
use syn::punctuated::Punctuated;

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    let Fields::Unnamed(struct_fields) = &input.fields else {
        panic!("Entity should be a tuple-like struct")
    };

    let new_args_vec = struct_fields.unnamed.iter()
        .enumerate()
        .map(|(i, field)| FnArg::Typed(PatType {
            attrs: vec![],
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: vec![],
                by_ref: None, 
                mutability: None,
                ident: format_ident!("arg{}", i),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(field.ty.clone()),
        }))
        .collect::<Vec<FnArg>>();

    let new_typed_params: Punctuated<FnArg, syn::token::Comma> = Punctuated::from_iter(new_args_vec);
    let new_params: Punctuated<syn::Ident, syn::token::Comma> = Punctuated::from_iter(
        (0..new_typed_params.len()).map(|i| format_ident!("arg{}", i))
    );

    quote! { 
        #input
    
        impl #struct_name {
            pub fn new(#new_typed_params) -> Self { Self(#new_params) }
        }
    }.into()
}
