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
                #ident: match <#type_name as Decode>::decode(bytes).await {
                    Ok(value) => *value,
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
            pub async fn decode<T>(bytes: &mut T) -> Result<Self, Box<dyn std::error::Error>>
            where
                T: AsyncRead + AsyncSeek + Unpin,
            {
                Ok(Self {
                    #(#field_statements)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Encode)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_statements = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = input.data
    {
        fields
            .named
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let ty = &field.ty;
                quote! {
                    <#ty as Encode>::encode(&self.#ident, bytes).await?;
                }
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let expanded = quote! {
        impl #name {
            pub async fn encode<T>(&self, bytes: &mut T) -> core::result::Result<(), Error>
            where
                T: AsyncWrite + AsyncSeek + Unpin,
            {
                #(#field_statements)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
