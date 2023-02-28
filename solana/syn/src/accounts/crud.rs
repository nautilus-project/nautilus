pub fn nautilus_account_create_tokens(
    struct_name: &syn::Ident,
    gather_authorities_syntax: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountCreate for #struct_name {
            fn parse_nautilus_create_args<'a>(
                program_id: &'a nautilus::Pubkey,
                accounts: &'a [AccountInfo<'a>],
                create_instruction_args: Self,
            ) -> Result<nautilus::NautilusCreateArgs<'a, Self>, nautilus::ProgramError> {

                let accounts_iter = &mut accounts.iter();
                let autoinc_account = match Self::AUTO_INCREMENT {
                    true => Some(next_account_info(accounts_iter)?.to_owned()),
                    false => None,
                };
                let new_account = next_account_info(accounts_iter)?.to_owned();
                let authorities = #gather_authorities_syntax
                let fee_payer = next_account_info(accounts_iter)?.to_owned();
                let system_program = next_account_info(accounts_iter)?.to_owned();

                Ok(nautilus::NautilusCreateArgs {
                    program_id,
                    autoinc_account,
                    new_account,
                    authorities,
                    fee_payer,
                    system_program,
                    data: create_instruction_args,
                })
            }
        }
    }
}

pub fn nautilus_account_delete_tokens(
    struct_name: &syn::Ident,
    gather_authorities_syntax: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountDelete for #struct_name {
            fn parse_nautilus_delete_args<'a>(
                program_id: &'a nautilus::Pubkey,
                accounts: &'a [AccountInfo<'a>],
            ) -> Result<nautilus::NautilusDeleteArgs<'a>, nautilus::ProgramError> {

                let accounts_iter = &mut accounts.iter();
                let target_account = next_account_info(accounts_iter)?.to_owned();
                let authorities = #gather_authorities_syntax
                let fee_payer = next_account_info(accounts_iter)?.to_owned();

                Ok(nautilus::NautilusDeleteArgs {
                    program_id,
                    target_account,
                    authorities,
                    fee_payer,
                })
            }
        }
    }
}

pub fn nautilus_account_update_tokens(
    struct_name: &syn::Ident,
    struct_name_optionized: &syn::Ident,
    gather_authorities_syntax: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountUpdate for #struct_name {

            fn parse_nautilus_update_args<'a, T: nautilus::NautilusOptionized>(
                program_id: &'a nautilus::Pubkey,
                accounts: &'a [AccountInfo<'a>],
                update_data: T,
            ) -> Result<nautilus::NautilusUpdateArgs<'a, T>, nautilus::ProgramError> {

                let accounts_iter = &mut accounts.iter();
                let target_account = next_account_info(accounts_iter)?.to_owned();
                let authorities = #gather_authorities_syntax
                let fee_payer = next_account_info(accounts_iter)?.to_owned();
                let system_program = next_account_info(accounts_iter)?.to_owned();

                Ok(nautilus::NautilusUpdateArgs {
                    program_id,
                    target_account,
                    authorities,
                    fee_payer,
                    system_program,
                    update_data,
                })
            }

            fn process_nautilus_update_data<#struct_name_optionized>(
                &mut self,
                update_data: #struct_name_optionized,
            ) {

                todo!()
            }
        }
    }
}
