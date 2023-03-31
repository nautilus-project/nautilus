use nautilus_syn::{entry::NautilusEntrypoint, object::NautilusObject};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Item};

extern crate proc_macro;

// #[proc_macro_attribute]
// pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
//     parse_macro_input!(input as NautilusEntrypoint)
//         .to_token_stream()
//         .into()
// }

// #[proc_macro_derive(Nautilus, attributes(default_instructions, primary_key, authority))]
// pub fn nautilus_object(input: TokenStream) -> TokenStream {
//     parse_macro_input!(input as NautilusObject)
//         .to_token_stream()
//         .into()
// }

#[proc_macro_attribute]
pub fn nautilus(_: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_item = parse_macro_input!(input as Item);

    match parsed_item {
        Item::Struct(item_struct) => {
            let object: NautilusObject = item_struct.into();
            object.to_token_stream().into()
        }
        Item::Mod(item_mod) => {
            let entry: NautilusEntrypoint = item_mod.into();
            entry.to_token_stream().into()
        }
        _ => panic!("#[nautilus] can only be used with structs and modules."),
    }
}
