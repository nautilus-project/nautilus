extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use nautilus_syn::{ NautilusAccountStruct, NautilusEntrypointEnum };
use syn::parse_macro_input;

#[proc_macro_derive(NautilusEntrypoint)]
pub fn derive_nautilus_entrypoint(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusEntrypointEnum)
        .to_token_stream()
        .into()
}

#[proc_macro_derive(NautilusAccount, attributes(primary_key, authority))]
pub fn derive_nautilus_account(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusAccountStruct)
        .to_token_stream()
        .into()
}
