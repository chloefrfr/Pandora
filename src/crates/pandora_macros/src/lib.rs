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
            let type_name = field.ty;

            let statement = quote! {
                #ident: match <#type_name as Decode>::decode(bytes) {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Decode error in field '{}': {:?}", stringify!(#ident), e);
                        return Err(e.into());
                    },
                },
            };
            field_statements.push(statement);
        }
    }

    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn decode<T>(bytes: &mut T) -> Result<Self, Box<dyn std::error::Error>>
            where
                T: std::io::Read,
            {
                Ok(Self {
                    #(#field_statements)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
