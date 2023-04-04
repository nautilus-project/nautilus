use nautilus_idl::idl_instruction::IdlInstruction;
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
    pub fn new(nautilus_objects: Vec<NautilusObject>, declared_functions: Vec<ItemFn>) -> Self {
        let variants = declared_functions
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let (variant_ident, variant_args, call_ident, call_context, modified_fn) =
                    parse_function(&nautilus_objects, f);
                NautilusEntrypointEnumVariant::new(
                    i.try_into().unwrap(),
                    variant_ident,
                    variant_args,
                    call_ident,
                    call_context,
                    modified_fn,
                )
            })
            .collect();
        Self { variants }
    }

    pub fn enum_ident() -> Ident {
        Ident::new("NautilusEntrypoint", Span::call_site())
    }
}

impl From<&NautilusEntrypointEnum>
    for (TokenStream, TokenStream, TokenStream, Vec<IdlInstruction>)
{
    fn from(value: &NautilusEntrypointEnum) -> Self {
        let enum_name = NautilusEntrypointEnum::enum_ident();
        let (variants, match_arms, modified_fns, idl_instructions) = value.variants.iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(mut variants, mut match_arms, mut modified_fns, mut idl_instructions), v| {
                let (a, b, c, d): (TokenStream, TokenStream, TokenStream, IdlInstruction) =
                    v.into();
                variants.push(a);
                match_arms.push(b);
                modified_fns.push(c);
                idl_instructions.push(d);
                (variants, match_arms, modified_fns, idl_instructions)
            },
        );
        (
            quote! {
                #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
                pub enum #enum_name {
                    #(#variants)*
                }
            },
            quote! { #(#modified_fns)* },
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
