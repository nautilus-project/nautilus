#[derive(Debug)]
pub struct NautilusEntrypointEnum {
    pub variants: Vec<super::entry_variant::NautilusEntrypointEnumVariant>,
}

impl NautilusEntrypointEnum {
    pub fn new(
        mod_ident: syn::Ident,
        nautilus_object_names: Vec<String>,
        entrypoint_functions: impl Iterator<Item = syn::ItemFn>,
    ) -> Self {
        let variants = entrypoint_functions
            .enumerate()
            .map(|(i, f)| {
                super::entry_variant::NautilusEntrypointEnumVariant::new(
                    mod_ident.clone(),
                    i.try_into().unwrap(),
                    f,
                    nautilus_object_names.clone(),
                )
            })
            .collect();
        Self { variants }
    }

    pub fn enum_name() -> syn::Ident {
        syn::Ident::new("NautilusEntrypoint", proc_macro2::Span::call_site())
    }
}

impl From<&NautilusEntrypointEnum>
    for (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        Vec<nautilus_idl::IdlInstruction>,
    )
{
    fn from(value: &NautilusEntrypointEnum) -> Self {
        let enum_name = NautilusEntrypointEnum::enum_name();
        let (variants, match_arms, idl_instructions) = value.variants.iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut variants, mut match_arms, mut idl_instructions), v| {
                let (a, b, c): (
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                    nautilus_idl::IdlInstruction,
                ) = v.into();
                variants.push(a);
                match_arms.push(b);
                idl_instructions.push(c);
                (variants, match_arms, idl_instructions)
            },
        );
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
            idl_instructions,
        )
    }
}
