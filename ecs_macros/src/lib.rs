use std::alloc::alloc_zeroed;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemStruct, Fields, FnArg, PatType, PatIdent, Pat, Ident, Type, Token, Index, Result, parse2};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};

struct CommaSeparatedTypes {
    types: Punctuated<Type, Token![,]>,
}

impl Parse for CommaSeparatedTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let types = input.parse_terminated(Type::parse, Token![,])?;
        Ok(Self { types })
    }
}

#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let args: CommaSeparatedTypes = parse2(args.into()).expect("#[entity]'s arguments should be comma separated types");

    let struct_name = &input.ident;

    let fields: Vec<&Type> = match &input.fields {
        Fields::Unnamed(f) => f,
        _ => panic!("Entity should be a tuple-like struct"),
    }.unnamed
        .iter()
        .map(|field| &field.ty)
        .collect();

    let all_components: Vec<&Type> = args.types.iter().collect();

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

    let impl_aware_of_components: Vec<proc_macro2::TokenStream> = all_components
        .iter()
        .map(|&component_ty| {
            // TODO test that components are unique
            let (const_return, mut_return) = match fields.iter().position(|&field_ty| field_ty == component_ty) {
                Some(i) => (
                    quote! { Some(&self.#i) }, 
                    quote! { Some(&mut self.#i) },
                ),
                None => (
                    quote! { None }, 
                    quote! { None },
                )
            };

            quote! {
                impl crate::ecs::AwareOfComponent<#component_ty> for #struct_name {
                    fn try_get_component_raw(&self) -> Option<&#component_ty> { #const_return }
                    fn try_get_component_mut_raw(&mut self) -> Option<&mut #component_ty> { #mut_return }
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

        #(#impl_aware_of_components)*
    }.into()
}
