// extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::ToTokens;
// use syn::parse_macro_input;

pub mod crud;

// #[proc_macro_derive(Nautilus, attributes(nautilus))]
// pub fn derive_anchor_deserialize(item: TokenStream) -> TokenStream {
//     parse_macro_input!(item as anchor_syn::AccountsStruct)
//         .to_token_stream()
//         .into()
// }