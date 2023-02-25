// TODO: Build seed derivation based on type
//
pub fn build_tokens_primary_key_seed(
    key: &syn::Ident,
    _ty: &syn::Type,
) -> proc_macro2::TokenStream {
    // match ty {
    //     String?? => quote::quote! {
    //         unsafe { self.#key.as_bytes() }
    //     },
    //     U8?? => quote::quote! {
    //         unsafe { std::slice::from_raw_parts(&self.#key, 1) }
    //     },
    //     Pubkey?? => quote::quote! {
    //         unsafe { self.#key.as_ref() }
    //     }
    //     _ => Err("Only u8, string, and Pubkey seeds are allowed")
    // }
    quote::quote! {
        unsafe { std::slice::from_raw_parts(&self.#key, 1) }
    }
}

pub fn build_gather_check_authorities_tokens(
    idents_authorities: &Vec<syn::Ident>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match idents_authorities.len() == 0 {
        true => (quote::quote! { vec![]; }, quote::quote! { Ok(()) }),
        false => {
            let mut tokens_gather_account_infos: Vec<proc_macro2::TokenStream> = vec![];
            let mut tokens_signer_init_bools: Vec<proc_macro2::TokenStream> = vec![];
            let mut tokens_check_signer: Vec<proc_macro2::TokenStream> = vec![];
            let mut tokens_signer_check_bools: Vec<proc_macro2::TokenStream> = vec![];
            let mut tokens_signer_ref_bools: Vec<proc_macro2::TokenStream> = vec![];
            for a in idents_authorities {
                let ident_bool = syn::Ident::new(
                    &(a.to_string() + "_checked"),
                    proc_macro2::Span::call_site(),
                );
                tokens_gather_account_infos
                    .push(quote::quote! { next_account_info(accounts_iter)?.to_owned() });
                tokens_signer_init_bools.push(quote::quote! { let mut #ident_bool = false });
                tokens_check_signer.push(
                    quote::quote! { if account.key.eq(&self.#a) { #ident_bool = account.is_signer } },
                );
                tokens_signer_check_bools.push(quote::quote! { #ident_bool });
                tokens_signer_ref_bools.push(quote::quote! { true });
            }
            (
                quote::quote! {
                    vec![
                        #(#tokens_gather_account_infos,)*
                    ];
                },
                quote::quote! {
                    #(#tokens_signer_init_bools;)*
                    for account in accounts {
                        #(#tokens_check_signer;)*
                    }
                    if (#(#tokens_signer_check_bools,)*) == (#(#tokens_signer_ref_bools,)*) {
                        Ok(())
                    } else {
                        Err(solana_program::program_error::ProgramError::MissingRequiredSignature)
                    }
                },
            )
        }
    }
}

pub fn nautilus_optionized(
    ident_optionized_struct_name: &syn::Ident,
    tokens_optionized_struct_fields: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote::quote! {
        struct #ident_optionized_struct_name {
            #(#tokens_optionized_struct_fields,)*
        }
        impl nautilus::NautilusOptionized for #ident_optionized_struct_name {}
    }
}

pub fn nautilus_account_data_tokens(
    ident_struct_name: &syn::Ident,
    table_name: &String,
    autoincrement_enabled: bool,
    primary_key_seed: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountData for #ident_struct_name {

            const TABLE_NAME: &'static str = #table_name;
            const AUTO_INCREMENT: bool = #autoincrement_enabled;

            fn primary_key<'a>(&self) -> &'a [u8] {
                #primary_key_seed
            }

        }
    }
}

pub fn nautilus_account_auth_tokens(
    ident_struct_name: &syn::Ident,
    check_authorities_syntax: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountAuth for #ident_struct_name {
            fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
                #check_authorities_syntax
            }
        }
    }
}
