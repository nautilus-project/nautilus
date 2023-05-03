//! Nautilus' macros used to power its abstraction.
use nautilus_syn::{entry::NautilusEntrypoint, object::NautilusObject};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemStruct};

extern crate proc_macro;

/// The procedural macro to build the entirety of a Nautilus program.
///
/// This macro alone can build a valid Nautilus program from the annotated module.
///
/// Parses the annotated module into a `syn::ItemMod` and converts that to a `nautilus_syn::NautilusEntrypoint`
/// to build the program's entrypoint, processor, and IDL.
#[proc_macro_attribute]
pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(input as NautilusEntrypoint)
        .to_token_stream()
        .into()
}

/// The derive macro to implement the required traits to allow for the annotated struct to serve
/// as the data type for a Nautilus record - allowing it to be used as `T` inside of `Record<'_, T>`.
#[proc_macro_derive(Table, attributes(default_instructions, primary_key, authority))]
pub fn nautilus_table(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    NautilusObject::from_item_struct(
        item_struct,
        nautilus_syn::object::NautilusObjectType::Record,
    )
    .to_token_stream()
    .into()
}

/// The derive macro to implement the required traits to allow for the annotated struct to serve
/// as the data type for a Nautilus account - allowing it to be used as `T` inside of `Account<'_, T>`.
#[proc_macro_derive(Directory, attributes(seeds, authority))]
pub fn nautilus_account(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    NautilusObject::from_item_struct(
        item_struct,
        nautilus_syn::object::NautilusObjectType::Account,
    )
    .to_token_stream()
    .into()
}
