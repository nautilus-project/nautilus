use super::required_account;

#[derive(Debug)]
pub struct NautilusEntrypointEnum {
    pub variants: Vec<super::entry_variant::NautilusEntrypointEnumVariant>,
}

/// Creates the entrypoint enum by iterating over each parsed user-defined function and turning it
///     into a NautilusEntrypointEnumVariant, which will have all of the data required to
///     generate tokens and write an IDL fragment.
impl NautilusEntrypointEnum {
    pub fn new<'a>(
        nautilus_structs: Vec<(String, Vec<super::required_account::RequiredAccount>)>,
        entrypoint_functions: impl Iterator<Item = syn::ItemFn>,
    ) -> Self {
        let variants = entrypoint_functions
            .enumerate()
            .map(|(i, f)| {
                super::entry_variant::NautilusEntrypointEnumVariant::new(
                    i.try_into().unwrap(),
                    f,
                    nautilus_structs.clone(),
                )
            })
            .collect();
        Self { variants }
    }
}

/// Allows us to convert the entrypoint enum into the following pieces:
///     * instruction_enum: the enum itself representing the instructions as variants
///     * processor: the `process_instruction` fn with the match statement inside
///
impl From<&NautilusEntrypointEnum> for (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    fn from(value: &NautilusEntrypointEnum) -> Self {
        let enum_name = syn::Ident::new("NautilusEntrypoint", proc_macro2::Span::call_site());
        let (variants, match_arms): (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) =
            value
                .variants
                .iter()
                .map(|v| {
                    let name = &v.name;
                    let required_accounts = &v.required_accounts;
                    println!("REQUIRED ACCOUNTS: {}", required_accounts.len());
                    required_accounts.into_iter().for_each(|r| {
                        println!(
                            "{}, isMutable: {}, isSigner: {}, desc: {}",
                            r.name, r.is_mut, r.is_signer, r.desc
                        );
                    });
                    let (arg_names, arg_types): (Vec<syn::Ident>, Vec<syn::Type>) =
                        v.args.clone().into_iter().unzip();
                    (
                        quote::quote! { #name(#(#arg_types,)*), },
                        quote::quote! { #enum_name::#name(#(#arg_names,)*) => Ok(()), },
                    )
                })
                .unzip();
        (
            quote::quote! {
                #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
                pub enum #enum_name {
                    #(#variants)*
                }
            },
            quote::quote! {
                pub fn process_instruction(
                    program_id: &solana_program::pubkey::Pubkey,
                    accounts: &[solana_program::account_info::AccountInfo],
                    input: &[u8],
                ) -> solana_program::entrypoint::ProgramResult {
                    let instruction = #enum_name::try_from_slice(input)?;

                    match instruction {
                        #(#match_arms)*
                    }
                }

                solana_program::entrypoint!(process_instruction);
            },
        )
    }
}

// /// This function will convert Vec<Variant> -> Vec<IdlInstruction> using the nautilus_idl crate.
// ///
// /// It will also introduce a `write_to_idl_fragment()` fn for writing instructions to IDL.
// ///
// impl From<&NautilusEntrypointEnum> for nautilus_idl::IdlInstruction {
//     fn from(ast: &NautilusEntrypointEnum) {
//         todo!()
//     }
// }
