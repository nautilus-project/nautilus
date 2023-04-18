pub mod entry_enum;
pub mod entry_variant;
pub mod idl;
pub mod parser;
pub mod required_account;

use nautilus_idl::{
    converters::{py::PythonIdlWrite, ts::TypeScriptIdlWrite},
    idl_metadata::IdlMetadata,
    Idl,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Item, ItemMod};

use self::{
    entry_enum::NautilusEntrypointEnum,
    parser::{is_use_super_star, parse_crate_context, parse_manifest},
};

#[derive(Debug)]
pub struct NautilusEntrypoint {
    pub leftover_content: Vec<Item>,
    pub instruction_enum: TokenStream,
    pub functions: TokenStream,
    pub processor: TokenStream,
}

impl From<ItemMod> for NautilusEntrypoint {
    fn from(value: ItemMod) -> Self {
        let mut declared_functions = vec![];

        let leftover_content: Vec<Item> = value
            .content
            .clone()
            .unwrap()
            .1
            .into_iter()
            .filter_map(|item| match is_use_super_star(&item) {
                true => None,
                false => match item {
                    Item::Fn(input_fn) => {
                        declared_functions.push(input_fn);
                        None
                    }
                    _ => Some(item),
                },
            })
            .collect();

        let (crate_version, crate_name) = parse_manifest();
        let (nautilus_objects, idl_accounts, idl_types) = parse_crate_context();

        let nautilus_enum = &NautilusEntrypointEnum::new(nautilus_objects, declared_functions);
        let (instruction_enum, functions, processor, idl_instructions) = nautilus_enum.into();

        let idl = Idl::new(
            crate_version,
            crate_name,
            idl_instructions,
            idl_accounts,
            idl_types,
            IdlMetadata::new_with_no_id(),
        );
        match idl.write_to_json("./target/idl") {
            Ok(()) => (),
            Err(e) => println!("[ERROR]: Error writing IDL to JSON file: {:#?}", e),
        };
        match idl.write_to_py("./target/idl") {
            Ok(()) => (),
            Err(e) => println!(
                "[ERROR]: Error writing Python bindings to .py file: {:#?}",
                e
            ),
        };
        match idl.write_to_ts("./target/idl") {
            Ok(()) => (),
            Err(e) => println!(
                "[ERROR]: Error writing TypeScript bindings to .ts file: {:#?}",
                e
            ),
        };

        Self {
            leftover_content,
            instruction_enum,
            functions,
            processor,
        }
    }
}

impl Parse for NautilusEntrypoint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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
        let leftover_content = &ast.leftover_content;
        let instruction_enum = &ast.instruction_enum;
        let functions = &ast.functions;
        let processor = &ast.processor;

        quote! {
            #instruction_enum
            #functions
            #processor
            #(#leftover_content)*
        }
        .into()
    }
}
