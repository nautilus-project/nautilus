use super::error::EnforcePrimaryKeyType;

// TODO: Build seed derivation based on type
//
pub fn build_tokens_primary_key_seed(
    key: &syn::Ident,
    ty: &syn::Type,
) -> Result<proc_macro2::TokenStream, EnforcePrimaryKeyType> {
    match quote::quote!(#ty).to_string().as_str() {
        "String" => Ok(quote::quote! {
            unsafe { self.#key.as_bytes() }
        }),
        "u8" => Ok(quote::quote! {
            unsafe { std::slice::from_raw_parts(&self.#key, 1) }
        }),
        "Pubkey" => Ok(quote::quote! {
            unsafe { self.#key.as_ref() }
        }),
        _ => Err(super::error::EnforcePrimaryKeyType()),
    }
}

pub fn build_count_check_authorities_tokens(
    idents_authorities: &Vec<syn::Ident>,
) -> (u8, proc_macro2::TokenStream) {
    match idents_authorities.len() == 0 {
        true => (0, quote::quote! { Ok(()) }),
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
                idents_authorities.len().try_into().unwrap(),
                quote::quote! {
                    #(#tokens_signer_init_bools;)*
                    for account in accounts {
                        #(#tokens_check_signer;)*
                    }
                    if (#(#tokens_signer_check_bools,)*) == (#(#tokens_signer_ref_bools,)*) {
                        Ok(())
                    } else {
                        Err(nautilus::ProgramError::MissingRequiredSignature)
                    }
                },
            )
        }
    }
}

pub fn nautilus_optionized(
    ident_optionized_struct_name: &syn::Ident,
    tokens_optionized_struct_fields: &Vec<proc_macro2::TokenStream>,
    ident_struct_name: &syn::Ident,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    ident_primary_key: &syn::Ident,
) -> proc_macro2::TokenStream {
    let process_update_tokens: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter(|f| !f.ident.as_ref().unwrap().eq(ident_primary_key))
        .map(|f| {
            let ident_field = &f.ident;
            quote::quote! {
                match update_data.#ident_field {
                    Some(val) => data.#ident_field = val,
                    None => (),
                }
            }
        })
        .collect();

    quote::quote! {
        struct #ident_optionized_struct_name {
            #(#tokens_optionized_struct_fields,)*
        }
        impl nautilus::NautilusOptionized for #ident_optionized_struct_name {
            fn process_nautilus_update_data<T: NautilusAccountData>(data: T, update_data: Self) -> T {
                // #(#process_update_tokens;)*
                data
            }
        }
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
    count_authorities: u8,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl nautilus::NautilusAccountAuth for #ident_struct_name {
            fn check_authorities(&self, accounts: Vec<nautilus::AccountInfo>) -> Result<(), ProgramError> {
                #check_authorities_syntax
            }

            fn count_authorities<'a>() -> u8 {
                #count_authorities
            }
        }
    }
}
