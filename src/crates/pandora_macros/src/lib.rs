extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);

    let mut field_assignments = Vec::new();

    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields),
        ..
    }) = parsed_input.data
    {
        for field in fields.named {
            let field_name = field.ident.unwrap();
            let field_type = &field.ty;

            let assignment = quote! {
                #field_name: #field_type::decode(bytes)?,
            };
            field_assignments.push(assignment);
        }
    } else {
        return quote! {
            compile_error!("Decode can only be derived for structs with named fields.");
        }
        .into();
    }

    let struct_name = parsed_input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn decode<R>(bytes: &mut R) -> Result<Self, Box<dyn std::error::Error>>
            where
                R: Read,
            {
                Ok(Self {
                    #(#field_assignments)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
