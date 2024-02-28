extern crate proc_macro;
use proc_macro::TokenStream;

mod mutate;


#[proc_macro_derive(Mutate)]
pub fn derive_mutate(input: TokenStream) -> TokenStream {
    mutate::derive_proc_macro_impl(input)
}