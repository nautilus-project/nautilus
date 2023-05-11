//! Builds the entrypoint, processor, and IDL for a Nautilus program.
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
use syn::{parse::Parse, Item, ItemFn, ItemMod};

use crate::object::parser::NautilusObjectConfig;

use self::{
    entry_enum::NautilusEntrypointEnum,
    parser::{is_use_super_star, parse_crate_context, parse_manifest},
};

/// The struct containing the parsed contents required to convert the user's
/// annotated module into the proper program configurations.
/// * `leftover_content`: Any declarations in the module that aren't named
///   functions.
/// * `instruction_enum`: The built-out program instruction enum derived from
///   the functions and their arguments.
/// * `all_functions`: The user's declared functions as-is, plus any default
///   instructions.
/// * `processor`: The program's processor, built into a function
///   `process_instruction`.
#[derive(Debug)]
pub struct NautilusEntrypoint {
    pub leftover_content: Vec<Item>,
    pub instruction_enum: TokenStream,
    pub all_functions: Vec<ItemFn>,
    pub processor: TokenStream,
}

impl From<ItemMod> for NautilusEntrypoint {
    /// Converts the user's annotated module into the `NautilusEntrypoint`
    /// struct.
    ///
    /// All of the work to build out the entrypoint, processor, and IDL for a
    /// Nautilus program is done here. During this conversion, the module
    /// (`ItemMod`) is broken down into components, and the functions declared
    /// by the user are extracted and used to build out various child
    /// structs such as `NautilusEntrypointEnum` and
    /// `NautilusEntrypointEnumVariant`.
    ///
    /// Using information extracted from the user's manifest (`Cargo.toml`) and
    /// the entirety of their crate, this function builds the
    /// `NautilusEntrypointEnum`, which basically dissolves to the required
    /// components.
    ///
    /// For more specific information see the documentation for
    /// `NautilusEntrypointEnum` and `NautilusEntrypointEnumVariant`.
    fn from(value: ItemMod) -> Self {
        let mut all_functions = vec![];

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
                        all_functions.push(input_fn);
                        None
                    }
                    _ => Some(item),
                },
            })
            .collect();

        let (crate_version, crate_name) = parse_manifest();
        let (nautilus_objects, idl_accounts, idl_types) = parse_crate_context();

        // Append any default instructions to the list of functions
        nautilus_objects.iter().for_each(|o| {
            if let Some(obj_config) = &o.object_config {
                if let NautilusObjectConfig::RecordConfig {
                    ref default_instructions,
                    ..
                } = obj_config
                {
                    all_functions
                        .append(&mut default_instructions.iter().map(ItemFn::from).collect());
                }
            }
        });

        let nautilus_enum = &NautilusEntrypointEnum::new(nautilus_objects, all_functions.clone());
        let (instruction_enum, processor, idl_instructions) = nautilus_enum.into();

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
            all_functions,
            processor,
        }
    }
}

impl Parse for NautilusEntrypoint {
    /// Parses the user's defined module into a `syn::ItemMod`, which is an
    /// already pre-fabricated function, and calls Into<NautilusEntrypoint>
    /// to fire the `from(value: ItemMod)` in the trait implementation `impl
    /// From<ItemMod> for NautilusEntrypoint`, which does all the magic.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ItemMod::parse(input)?.into())
    }
}

impl ToTokens for NautilusEntrypoint {
    /// Extends the existing compiler tokens by the tokens generated by the
    /// `NautilusEntrypoint`.
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&NautilusEntrypoint> for TokenStream {
    /// Converts each component of the `NautilusEntrypoint` into the proper
    /// tokens for the compiler.
    fn from(ast: &NautilusEntrypoint) -> Self {
        let leftover_content = &ast.leftover_content;
        let instruction_enum = &ast.instruction_enum;
        let all_functions = &ast.all_functions;
        let processor = &ast.processor;

        quote! {
            #instruction_enum
            #processor
            #(#all_functions)*
            #(#leftover_content)*
        }
        .into()
    }
}
