use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemStruct, Fields, FnArg, PatType, PatIdent, Pat, Ident, Type, Token, Index};
use syn::punctuated::Punctuated;

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let struct_name = &input.ident;

    let fields: Vec<&Type> = match &input.fields {
        Fields::Unnamed(f) => f,
        _ => panic!("Entity should be a tuple-like struct"),
    }.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect();

    let new_args_base: Vec<(Ident, &Type)> = fields
        .iter()
        .enumerate()
        .map(|(i, &ty)| (format_ident!("arg{}", i), ty))
        .collect();

    let new_typed_params: Punctuated<FnArg, Token![,]> = Punctuated::from_iter(
        new_args_base
            .iter()
            .map(|(ident, &ref ty)| FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None, 
                    mutability: None,
                    ident: ident.clone(),
                    subpat: None,
                })),
                colon_token: Default::default(),
                ty: Box::new(ty.clone()),
            }))
    );

    let new_params: Punctuated<Ident, Token![,]> = Punctuated::from_iter(
        new_args_base.iter().map(|(ident, _)| ident.clone())
    );

    let impl_has_components: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .enumerate()
        .map(|(i, &ty)| {
            let i = Index::from(i);
            quote! {
                impl crate::ecs::HasComponent<#ty> for #struct_name {
                    fn get_component_raw(&self) -> &#ty { &self.#i }
                    fn get_component_mut_raw(&mut self) -> &mut #ty { &mut self.#i }
                }
            }
        })
        .collect();

    quote! { 
        #input
    
        impl #struct_name {
            pub fn new(#new_typed_params) -> Self { Self(#new_params) }
        }

        #(#impl_has_components)*
    }.into()
}
