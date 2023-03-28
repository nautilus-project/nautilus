use nautilus_idl::IdlInstruction;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, ItemFn};

use crate::{
    entry::entry_variant::NautilusEntrypointEnumVariant, entry::parser::parse_function,
    object::NautilusObject,
};

#[derive(Debug)]
pub struct NautilusEntrypointEnum {
    pub variants: Vec<NautilusEntrypointEnumVariant>,
}

impl NautilusEntrypointEnum {
    pub fn new(
        nautilus_objects: Vec<NautilusObject>,
        entrypoint_functions: impl Iterator<Item = ItemFn>,
    ) -> Self {
        let variants = entrypoint_functions
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
                pub fn process_instruction(
                    program_id: &Pubkey,
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
