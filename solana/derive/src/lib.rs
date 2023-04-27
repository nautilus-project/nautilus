use nautilus_syn::{entry::NautilusEntrypoint, object::NautilusObject};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusEntrypoint)
        .to_token_stream()
        .into()
}

#[proc_macro_derive(Table, attributes(default_instructions, primary_key, authority))]
pub fn nautilus_table(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusObject)
        .to_token_stream()
        .into()
}

#[proc_macro_derive(Object, attributes(seeds, authority))]
pub fn nautilus_object(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusObject)
        .to_token_stream()
        .into()
}
