extern crate proc_macro;

use polars::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ToDataFrame)]
pub fn to_dataframe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Unsupported struct type"),
        },
        _ => panic!("Unsupported data type"),
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let vec_creations = fields.iter().map(|f| {
                                         let ident = &f.ident;
                                         quote! { let mut #ident = Vec::with_capacity(len); }
                                     });
    let pushes = fields.iter().map(|f| {
                                  let ident = &f.ident;
                                  quote! { #ident.push(e.#ident.clone()); }
                              });

    let gen = quote! {
        impl #name {
            pub fn to_dataframe(input: Vec<#name>) -> polars::prelude::DataFrame {
                let len = input.len();
                #(#vec_creations)*
                for e in input {
                    #(#pushes)*
                }
                df! {
                    #(#field_names => #field_names,)*
                }
            }
        }
    };
    gen.into()
}
