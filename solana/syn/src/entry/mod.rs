pub mod call_context;
pub mod entry_enum;
pub mod entry_variant;
pub mod parser;
pub mod required_account;

use nautilus_idl::{Idl, IdlMetadata};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Item, ItemMod,
};

use self::{
    entry_enum::NautilusEntrypointEnum,
    parser::{is_use_super_star, parse_crate_context, parse_manifest},
};

#[derive(Debug)]
pub struct NautilusEntrypoint {
    pub mod_content: Vec<Item>,
    pub instruction_enum: TokenStream,
    pub processor: TokenStream,
}

impl From<ItemMod> for NautilusEntrypoint {
    fn from(value: ItemMod) -> Self {
        let mod_content: Vec<Item> = value
            .content
            .unwrap()
            .1
            .into_iter()
            .filter(|item| !is_use_super_star(item))
            .collect();

        let (crate_version, crate_name) = parse_manifest();
        let (nautilus_objects, idl_accounts, idl_types) = parse_crate_context();

        let nautilus_enum = &NautilusEntrypointEnum::new(
            nautilus_objects,
            mod_content.iter().filter_map(|item| {
                if let Item::Fn(item_fn) = item {
                    Some(item_fn.clone())
                } else {
                    None
                }
            }),
        );
        let (instruction_enum, processor, idl_instructions) = nautilus_enum.into();

        Idl::new(
            &crate_version,
            &crate_name,
            idl_instructions,
            idl_accounts,
            idl_types,
            IdlMetadata::new_with_no_id(),
        )
        .write();

        Self {
            mod_content,
            instruction_enum,
            processor,
        }
    }
}

impl Parse for NautilusEntrypoint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ItemMod::parse(input)?.into())
    }
}

impl ToTokens for NautilusEntrypoint {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&NautilusEntrypoint> for TokenStream {
    fn from(ast: &NautilusEntrypoint) -> Self {
        let mod_content = &ast.mod_content;
        let instruction_enum = &ast.instruction_enum;
        let processor = &ast.processor;

        quote! {
            #instruction_enum
            #(#mod_content)*
            #processor
        }
        .into()
    }
}
