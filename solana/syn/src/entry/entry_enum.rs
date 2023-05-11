//! A `syn`-powered enum that dissolves to the required components to create the
//! program's entrypoint, processor, and IDL.
use nautilus_idl::idl_instruction::IdlInstruction;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemFn};

use crate::{
    entry::entry_variant::NautilusEntrypointEnumVariant, entry::parser::parse_function,
    object::NautilusObject,
};

/// The struct used to house all of the "variants" of type
/// `NautilusEntrypointEnumVariant` which dissolve to the required components
/// for building out the generated program.
///
/// The key functionality actually occurs in the trait implementations for this
/// struct - including the self implementations such as `new(..)`.
#[derive(Debug)]
pub struct NautilusEntrypointEnum {
    pub variants: Vec<NautilusEntrypointEnumVariant>,
}

impl NautilusEntrypointEnum {
    /// Creates a new `NautilusEntrypointEnum`.
    ///
    /// This action will simply convert the user's declared functions into
    /// `NautilusEntrypointEnumVariant` instances, which dissolve to
    /// the required components for building out the generated program.
    pub fn new(nautilus_objects: Vec<NautilusObject>, declared_functions: Vec<ItemFn>) -> Self {
        let variants = declared_functions
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let (variant_ident, variant_args, call_ident, call_context) =
                    parse_function(&nautilus_objects, f);
                NautilusEntrypointEnumVariant::new(
                    i.try_into().unwrap(),
                    variant_ident,
                    variant_args,
                    call_ident,
                    call_context,
                )
            })
            .collect();
        Self { variants }
    }

    pub fn enum_ident() -> Ident {
        Ident::new("NautilusEntrypoint", Span::call_site())
    }
}

impl From<&NautilusEntrypointEnum> for (TokenStream, TokenStream, Vec<IdlInstruction>) {
    /// Maps each `NautilusEntrypointEnumVariant` into the proper components and
    /// dissolves itself into the required components for building out the
    /// generated program.
    ///
    /// Consider the `fold` operation on the `variants` field, which returns
    /// (`variants`, `match_arms`, `idl_instructions`):
    /// * `variants`: The variants of the instruction enum for the program.
    /// * `match_arms`: The match arms of the processor which will process
    ///   whichever instruction enum variant (and its arguments) is provided to
    ///   the program.
    /// * `idl_instructions`: The IDL instructions derived from the declared
    ///   functions - to be used in generating the IDL.
    ///
    /// Consider the return type of the function itself - defined at the trait
    /// level: (`TokenStream`, `TokenStream`, `Vec<IdlInstruction>`):
    /// * `TokenStream` (first): The instruction enum.
    /// * `TokenStream` (second): The processor.
    /// * `Vec<IdlInstruction>`: The list of IDL instructions.
    fn from(value: &NautilusEntrypointEnum) -> Self {
        let enum_name = NautilusEntrypointEnum::enum_ident();
        let (variants, match_arms, idl_instructions) = value.variants.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut variants, mut match_arms, mut idl_instructions), v| {
                let (a, b, c): (TokenStream, TokenStream, IdlInstruction) = v.into();
                variants.push(a);
                match_arms.push(b);
                idl_instructions.push(c);
                (variants, match_arms, idl_instructions)
            },
        );
        (
            quote! {
                #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
                pub enum #enum_name {
                    #(#variants)*
                }
            },
            quote! {
                pub fn process_instruction<'a>(
                    program_id: &'static Pubkey,
                    accounts: &[AccountInfo],
                    input: &[u8],
                ) -> ProgramResult {
                    let instruction = #enum_name::try_from_slice(input)?;

                    match instruction {
                        #(#match_arms)*
                    }
                }

                entrypoint!(process_instruction);
            },
            idl_instructions,
        )
    }
}
