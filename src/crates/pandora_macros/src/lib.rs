extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut field_statements = Vec::new();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = input.data
    {
        for field in fields.named {
            let ident = field.ident.unwrap();
            let statement = quote! {
                println("Failed to decode field: {}", stringify!(#ident));
            };
            field_statements.push(statement);
        }
    }
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn decode(bytes: &mut impl Into<Vec<u8>>) -> Self {
                #(#field_statements)*

                Self::default()

            }
        }
    };
    TokenStream::from(expanded)
}
