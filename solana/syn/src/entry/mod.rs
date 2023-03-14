//
//
// ----------------------------------------------------------------
//                 Nautilus entrypoint token generation
// ----------------------------------------------------------------
//
//     proc_macro2::TokenStream -> ItemMod
//         -> NautilusEntrypoint -> * -> proc_macro2::TokenStream
//                                  * New tokens created here
//
//

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusEntrypoint {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub mod_token: syn::token::Mod,
    pub ident: syn::Ident,
    pub content: Option<(syn::token::Brace, Vec<syn::Item>)>,
    pub semi: Option<syn::token::Semi>,
}

impl From<syn::ItemMod> for NautilusEntrypoint {
    fn from(value: syn::ItemMod) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            mod_token: value.mod_token,
            ident: value.ident,
            content: value.content,
            semi: value.semi,
        }
    }
}

impl syn::parse::Parse for NautilusEntrypoint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(syn::ItemMod::parse(input)?.into())
    }
}

impl quote::ToTokens for NautilusEntrypoint {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

impl From<&NautilusEntrypoint> for proc_macro2::TokenStream {
    fn from(_ast: &NautilusEntrypoint) -> Self {
        quote::quote! {

            #[derive(borsh::BorshDeserialize, borsh::BorshSerialize, shank::ShankInstruction)]
            pub enum MyEntrypoint {
                #[account(
                    0,
                    writable,
                    name = "autoinc_account",
                    desc = "The autoincrement account."
                )]
                #[account(1, writable, name = "new_account", desc = "The account to be created.")]
                #[account(
                    2,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(3, writable, signer, name = "fee_payer", desc = "Fee payer")]
                #[account(4, name = "system_program", desc = "The System Program")]
                CreateHero,

                #[account(
                    0,
                    writable,
                    name = "target_account",
                    desc = "The account to be deleted."
                )]
                #[account(
                    1,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
                DeleteHero,

                #[account(
                    0,
                    writable,
                    name = "target_account",
                    desc = "The account to be updated."
                )]
                #[account(
                    1,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
                #[account(3, name = "system_program", desc = "The System Program")]
                UpdateHero,

                #[account(
                    0,
                    writable,
                    name = "autoinc_account",
                    desc = "The autoincrement account."
                )]
                #[account(1, writable, name = "new_account", desc = "The account to be created.")]
                #[account(
                    2,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(3, writable, signer, name = "fee_payer", desc = "Fee payer")]
                #[account(4, name = "system_program", desc = "The System Program")]
                CreateVillain,

                #[account(
                    0,
                    writable,
                    name = "target_account",
                    desc = "The account to be deleted."
                )]
                #[account(
                    1,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
                DeleteVillain,

                #[account(
                    0,
                    writable,
                    name = "target_account",
                    desc = "The account to be updated."
                )]
                #[account(
                    1,
                    writable,
                    signer,
                    name = "authority",
                    desc = "One of the authorities specified for this account."
                )]
                #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
                #[account(3, name = "system_program", desc = "The System Program")]
                UpdateVillain,
            }


            fn process_instruction(
                program_id: &solana_program::pubkey::Pubkey,
                accounts: &[solana_program::account_info::AccountInfo],
                input: &[u8]
            ) -> solana_program::entrypoint::ProgramResult {

                let instruction = MyEntrypoint::try_from_slice(input)?;
                match instruction {
                    MyEntrypoint::CreateHero => {
                        println!("CreateHero");
                        Ok(())
                    }
                    MyEntrypoint::CreateHero => {
                        println!("CreateHero");
                        Ok(())
                    }
                    MyEntrypoint::DeleteHero => {
                        println!("DeleteHero");
                        Ok(())
                    }
                    MyEntrypoint::UpdateHero => {
                        println!("UpdateHero");
                        Ok(())
                    }
                    MyEntrypoint::CreateVillain => {
                        println!("CreateVillain");
                        Ok(())
                    }
                    MyEntrypoint::DeleteVillain => {
                        println!("DeleteVillain");
                        Ok(())
                    }
                    MyEntrypoint::UpdateVillain => {
                        println!("UpdateVillain");
                        Ok(())
                    }
                }
            }

            solana_program::entrypoint!(process_instruction);
        }
        .into()
    }
}
