extern crate proc_macro;

use proc_macro::TokenStream;
use quote::ToTokens;
use nautilus_syn::NautilusAccount;
use syn::parse_macro_input;

#[proc_macro_derive(NautilusEntrypoint)]
pub fn derive_nautilus_entrypoint(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(Nautilus, attributes(nautilus))]
pub fn derive_nautilus_account(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusAccount)
        .to_token_stream()
        .into()
}
