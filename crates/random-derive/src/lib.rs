extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;


#[proc_macro_derive(RandField)]
pub fn derive_rand_field(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let Data::Enum(enum_data) = data else { panic!("Cannot derive RandField for non-enums") };
    let variants = enum_data.variants.iter().map(|x| &x.ident);
    let length = enum_data.variants.len();
    let indices = 0..length;

    let match_arms = quote! { #(#indices => #ident::#variants), * };

    quote! {
        impl rand::distributions::Distribution<#ident> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> #ident {
                use rand::Rng;
                match rng.gen_range(0..#length) {
                    #match_arms,
                    _ => unreachable!()
                }
            }
        }

        impl #ident {
            pub fn rand_field(rng: &mut rand::rngs::ThreadRng) -> #ident {
                rand::random()
            }
        }
    }.into()
}
