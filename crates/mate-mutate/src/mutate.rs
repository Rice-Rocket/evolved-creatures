use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive_proc_macro_impl(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let description_str = match data {
        Data::Struct(val) => {
            match val.fields {
                Fields::Named(fields) => {
                    let named_fields = fields.named.iter().map(|x| &x.ident);
                    format!(
                        "a struct with named fields: {}",
                        quote! { #(#named_fields), * }
                    )
                },
                Fields::Unnamed(fields) => {
                    let unnamed_fields = fields.unnamed.iter().map(|x| &x.ident);
                    format!(
                        "a struct with unnamed fields: {}",
                        quote! { #(#unnamed_fields), * }
                    )
                },
                Fields::Unit => {
                    format!("a unit struct")
                }
            }
        },
        Data::Enum(val) => {
            let variant_idents = val.variants.iter().map(|x| &x.ident);
            format!(
                "an enum with variants: {}",
                quote! { #(#variant_idents), * }
            )
        },
        Data::Union(val) => {
            let named_fields = val.fields.named.iter().map(|x| &x.ident);
            format!(
                "a struct with named fields: {}",
                quote! { #(#named_fields), * }
            )
        }
    };

    quote! {
        impl #generics #ident #generics {
            fn mutate(&self) -> String {
                let mut string = String::from(stringify!(#ident));
                string.push_str(" is ");
                string.push_str(#description_str);
                string
            }
        }
    }.into()
}