use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Field, Fields, Meta, parse_macro_input};
use syn::spanned::Spanned;

#[proc_macro_derive(Resource, attributes(resource_key))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let map_name = format_ident!("{}Map", struct_name.clone());

    let Data::Struct(ref struc) = input.data else {
        return syn::Error::new(input.span(), "Can only implement resource on struct")
            .to_compile_error()
            .into();
    };

    let Fields::Named(ref fields) = struc.fields else {
        return syn::Error::new(struc.fields.span(), "Can only implement resource on struct with named fields")
            .to_compile_error()
            .into();
    };

    let key_fields: Vec<&Field> = fields.named.iter()
        .filter(|&f| {
            f.attrs.iter()
                .any(|a| {
                    if let Meta::Path(ref p) = a.meta {
                        p.is_ident("resource_key")
                    } else {
                        false
                    }
                })
        })
        .collect();

    let list_type = {
        let mut list_type = quote! { #struct_name };
        for &f in key_fields.iter().rev() {
            let ty = &f.ty;
            list_type = quote! { std::collections::HashMap<#ty, #list_type> };
        }
        list_type
    };

    let get_method_args = key_fields.iter()
        .map(|&f| {
            let name = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            quote! { #name: &#ty }
        });

    let key_names = key_fields.iter()
        .map(|&f| f.ident.to_token_stream());

    let doc = format!("Map type containing all [`{}`] in (nested) map format. Can be deserialized into.", struct_name);

    let json_name = format!("{}.json", struct_name);

    let expanded = quote! {
        #[derive(serde::Deserialize)]
        #[serde(transparent)]
        #[doc = #doc]
        pub struct #map_name(#list_type);

        impl #map_name {
            pub fn get(&self, #(#get_method_args),*) -> Option<&#struct_name> {
                self.0#(.get(#key_names))?*
            }
        }

        impl crate::resource::ResourceMap for #map_name {
            fn get_json_name() -> &'static str {
                #json_name
            }
        }
    };

    TokenStream::from(expanded)
}