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
                    Ok(value) => Box::into_inner(value),
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
    let mut field_statements = Vec::new();
    let mut has_packet_id = false;

    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = input.data
    {
        for field in fields.named {
            let ident = field.ident.unwrap();
            let type_name = field.ty;

            if ident == "packet_id" {
                has_packet_id = true;
            }

            let statement = quote! {
                <#type_name as Encode>::encode(&self.#ident, &mut bytes).await?;
            };
            field_statements.push(statement);
        }
    }

    if !has_packet_id {
        return TokenStream::from(quote! {
            compile_error!("Struct must have a packet_id field");
        });
    }

    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub async fn encode(&self) -> core::result::Result<Vec<u8>, std::io::Error>
            {
                let mut bytes = Vec::new();

                #(#field_statements)*

                let packet_data = bytes.clone();
                let length = packet_data.len() as i32;
                let var_int = packet_manager::types::varint_types::VarInt::new(length);

                let mut encoded_data = Vec::new();
                var_int.encode(&mut encoded_data).await?;

                encoded_data.extend_from_slice(&packet_data);

                Ok(encoded_data)
            }
        }
    };

    TokenStream::from(expanded)
}
