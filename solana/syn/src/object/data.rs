//! Spawns the tokens for the required trait implementations for an annotated struct.
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{token::Colon, Fields, FnArg, Ident, Pat, PatIdent, PatType, Type};

use super::seeds::Seed;

/// Generates tokens to implement `Clone` on a struct.
pub fn impl_clone(ident: &Ident, fields: &Fields) -> TokenStream {
    let clone_constructors = fields.iter().map(|f| {
        let ident = &f.ident.as_ref().unwrap();
        quote! { #ident: ::core::clone::Clone::clone(&self.#ident) }
    });
    quote! {
        impl ::core::clone::Clone for #ident {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    #(#clone_constructors,)*
                }
            }
        }
    }
}

/// Generates tokens to implement `Default` on a struct.
pub fn impl_default(ident: &Ident, fields: &Fields) -> TokenStream {
    let fields_default = fields.iter().map(|f| {
        let i = &f.ident.clone().unwrap();
        quote! { #i: ::core::default::Default::default() }
    });
    quote! {
        impl ::core::default::Default for #ident {
            #[inline]
            fn default() -> Self {
                Self {
                    #(#fields_default,)*
                }
            }
        }
    }
}

/// Generates tokens to implement `BorshDeserialize` and `BorshSerialize` on a struct.
pub fn impl_borsh(ident: &Ident, fields: &Fields) -> TokenStream {
    let borsh_ser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: nautilus::borsh::ser::BorshSerialize }
    });
    let borsh_ser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { nautilus::borsh::BorshSerialize::serialize(&self.#field_name, writer)? }
    });
    let borsh_deser_where = fields.iter().map(|f| {
        let field_ty = f.ty.clone();
        quote::quote! { #field_ty: nautilus::borsh::de::BorshDeserialize }
    });
    let borsh_deser_impl = fields.iter().map(|f| {
        let field_name = f.ident.clone();
        quote::quote! { #field_name: nautilus::borsh::BorshDeserialize::deserialize(buf)? }
    });
    quote::quote! {
        impl nautilus::borsh::ser::BorshSerialize for #ident
        where
            #(#borsh_ser_where,)*
        {
            fn serialize<W: nautilus::borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), nautilus::borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.discriminator(), writer)?; // Serialize the discriminator first
                #(#borsh_ser_impl;)*
                Ok(())
            }
        }
        impl nautilus::borsh::de::BorshDeserialize for #ident
        where
            #(#borsh_deser_where,)*
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, nautilus::borsh::maybestd::io::Error> {
                let _discrim: [u8; 8] = borsh::BorshDeserialize::deserialize(buf)?; // Skip the first 8 bytes for discriminator
                Ok(Self {
                    #(#borsh_deser_impl,)*
                })
            }
        }
    }
}

/// Generates tokens to implement `NautilusRecordData` on a struct.
pub fn impl_nautilus_record_data(
    ident: &Ident,
    fields: &Fields,
    table_name: &String,
    autoincrement: bool,
    primary_key_ident: &Ident,
    primary_key_ty: &Type,
) -> TokenStream {
    let nautilus_create_obj_trait_ident = &Ident::new(
        &("NautilusCreate".to_owned() + &ident.to_string()),
        Span::call_site(),
    );

    let tokens_primary_key_seed = build_tokens_primary_key_seed(primary_key_ident, primary_key_ty);

    let (data_new_fn_args, data_new_call_args) =
        get_new_fn_args_for_record(fields, autoincrement, primary_key_ident);

    let data_new_fn = match autoincrement {
        true => quote! {
            pub fn new<'a>(
                mut nautilus_index: NautilusIndex<'a>,
                fee_payer: impl NautilusSigner<'a>,
                #(#data_new_fn_args,)*
            ) -> Result<Box<Self>, ProgramError> {
                let #primary_key_ident = nautilus_index.add_record(
                    Self::TABLE_NAME,
                    fee_payer,
                )?.try_into().unwrap();
                Ok(Box::new(Self{ #primary_key_ident, #(#data_new_call_args,)* }))
            }
        },
        false => quote! {
            pub fn new<'a>(
                _nautilus_index: NautilusIndex<'a>,
                fee_payer: impl NautilusSigner<'a>,
                #(#data_new_fn_args,)*
            ) -> Result<Box<Self>, ProgramError> {
                Ok(Box::new(Self{ #(#data_new_call_args,)* }))
            }
        },
    };

    quote! {
        impl #ident {
            #data_new_fn
        }

        impl NautilusRecordData for #ident {
            const TABLE_NAME: &'static str = #table_name;

            const AUTO_INCREMENT: bool = #autoincrement;

            fn primary_key(&self) -> Vec<u8> {
                #tokens_primary_key_seed
            }

            fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
                todo!()
            }

            fn count_authorities(&self) -> u8 {
                todo!()
            }
        }

        pub trait #nautilus_create_obj_trait_ident<'a> {
            fn create(&mut self, #(#data_new_fn_args,)*) -> ProgramResult;
            fn create_with_payer(&mut self, #(#data_new_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult;
        }

        impl<'a> #nautilus_create_obj_trait_ident<'a> for Create<'a, Record<'a, #ident>> {
            fn create(&mut self, #(#data_new_fn_args,)*) -> ProgramResult {
                let rent_payer = Signer::new(Wallet {
                    account_info: self.fee_payer.to_owned(),
                    system_program: self.system_program.to_owned(),
                })?;
                self.self_account.data = #ident ::new(
                    self.self_account.index.clone(),
                    rent_payer,
                    #(#data_new_call_args,)*
                )?;
                self.create_record()
            }

            fn create_with_payer(&mut self, #(#data_new_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult {
                self.self_account.data = #ident ::new(
                    self.self_account.index.clone(),
                    payer.clone(),
                    #(#data_new_call_args,)*
                )?;
                self.create_record_with_payer(payer)
            }
        }
    }
}

/// Generates tokens to implement `NautilusAccountData` on a struct.
pub fn impl_nautilus_account_data(
    ident: &Ident,
    fields: &Fields,
    discrminator_str: &String,
    seeds: &Vec<Seed>,
) -> TokenStream {
    let nautilus_inner_trait_ident = &Ident::new(
        &("NautilusInner".to_owned() + &ident.to_string()),
        Span::call_site(),
    );
    let nautilus_create_obj_trait_ident = &Ident::new(
        &("NautilusCreate".to_owned() + &ident.to_string()),
        Span::call_site(),
    );

    let (data_new_fn_args, data_new_call_args) = get_new_fn_args_for_account(fields);

    let data_new_fn = quote! {
        pub fn new<'a>(
            fee_payer: impl NautilusSigner<'a>,
            #(#data_new_fn_args,)*
        ) -> Result<Box<Self>, ProgramError> {
            Ok(Box::new(Self{ #(#data_new_call_args,)* }))
        }
    };

    // Determines how we build the `seeds(..)` and `pda(..)` functions and their respective callers.
    let (seeds_inner, seeds_params_tuple) = build_seeds_components_for_account(seeds);
    let (
        seeds_args,
        seeds_caller,
        pda_args,
        pda_caller,
        pda_caller_outer,
        pda_args_outer,
        create_args,
        create_with_payer_args,
    ) = match &seeds_params_tuple {
        Some(tuple) => (
            quote! { &self, seeds: #tuple },
            quote! { seeds },
            quote! { &self, program_id: &Pubkey, seeds: #tuple },
            quote! { program_id, seeds },
            quote! { seeds },
            quote! { &self, seeds: #tuple },
            quote! { &mut self, #(#data_new_fn_args,)* seeds: #tuple },
            quote! { &mut self, #(#data_new_fn_args,)* seeds: #tuple, payer: impl NautilusSigner<'a> },
        ),
        None => (
            quote! { &self },
            quote!(),
            quote! { &self, program_id: &Pubkey },
            quote! { program_id },
            quote!(),
            quote! { &self },
            quote! { &mut self, #(#data_new_fn_args,)* },
            quote! { &mut self, #(#data_new_fn_args,)* payer: impl NautilusSigner<'a> },
        ),
    };
    // For seeds on inner data `T`.
    let seeds_fn = quote! {
        pub fn seeds(#seeds_args) -> Result<Vec<Vec<u8>>, ProgramError> {
            Ok(#seeds_inner)
        }
    };
    // For PDA on inner data `T`.
    let pda_fn = quote! {
        pub fn pda(#pda_args) -> Result<(Pubkey, u8), ProgramError> {
            let seeds_vec = self.seeds(#seeds_caller)?;
            let seeds: Vec<&[u8]> = seeds_vec.iter().map(AsRef::as_ref).collect();
            Ok(Pubkey::find_program_address(&seeds, program_id))
        }
    };
    // For seeds on `Account<T>`.
    let seeds_fn_outer = quote! {
        fn seeds(#seeds_args) -> Result<Vec<Vec<u8>>, ProgramError> {
            self.data.seeds(#seeds_caller)
        }
    };
    // For PDA on `Account<T>`.
    let pda_fn_outer = quote! {
        fn pda(#pda_args_outer) -> Result<(Pubkey, u8), ProgramError> {
            let program_id = self.program_id;
            self.data.pda(#pda_caller)
        }
    };
    // For seeds on `Create<Account<T>>`.
    let seeds_fn_outer_2 = quote! {
        fn seeds(#seeds_args) -> Result<Vec<Vec<u8>>, ProgramError> {
            self.self_account.seeds(#seeds_caller)
        }
    };
    // For PDA on `Create<Account<T>>`.
    let pda_fn_outer_2 = quote! {
        fn pda(#pda_args_outer) -> Result<(Pubkey, u8), ProgramError> {
            self.self_account.pda(#pda_caller_outer)
        }
    };

    quote! {
        impl #ident {
            #data_new_fn
            #seeds_fn
            #pda_fn
        }

        impl NautilusAccountData for #ident {
            const DISCRIMINATOR_STR: &'static str = #discrminator_str;

            fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
                todo!()
            }

            fn count_authorities(&self) -> u8 {
                todo!()
            }
        }

        pub trait #nautilus_inner_trait_ident<'a> {
            fn seeds(#seeds_args) -> Result<Vec<Vec<u8>>, ProgramError>;
            fn pda(#pda_args_outer) -> Result<(Pubkey, u8), ProgramError>;
        }

        impl<'a> #nautilus_inner_trait_ident<'a> for Account<'a, #ident> {
            #seeds_fn_outer
            #pda_fn_outer
        }

        impl<'a> #nautilus_inner_trait_ident<'a> for Create<'a, Account<'a, #ident>> {
            #seeds_fn_outer_2
            #pda_fn_outer_2
        }

        pub trait #nautilus_create_obj_trait_ident<'a> {
            fn create(#create_args) -> ProgramResult;
            fn create_with_payer(#create_with_payer_args) -> ProgramResult;
        }

        impl<'a> #nautilus_create_obj_trait_ident<'a> for Create<'a, Account<'a, #ident>> {
            fn create(#create_args) -> ProgramResult {
                let payer = Signer::new(Wallet {
                    account_info: self.fee_payer.to_owned(),
                    system_program: self.system_program.to_owned(),
                })?;
                self.self_account.data = #ident ::new(
                    payer.clone(),
                    #(#data_new_call_args,)*
                )?;
                let (pda, bump) = self.pda(#pda_caller_outer)?;
                assert_eq!(
                    &pda,
                    self.key(),
                    "Derived PDA does not match data for account {:#?}",
                    self.key()
                );
                let mut signer_seeds_vec = self.seeds(#seeds_caller)?;
                signer_seeds_vec.push(vec![bump]);
                let signer_seeds: Vec<&[u8]> = signer_seeds_vec.iter().map(AsRef::as_ref).collect();
                cpi::system::create_pda(
                    self.self_account.clone(),
                    self.self_account.program_id,
                    payer,
                    self.self_account.data.clone(),
                    signer_seeds,
                )
            }

            fn create_with_payer(#create_with_payer_args) -> ProgramResult {
                self.self_account.data = #ident ::new(
                    payer.clone(),
                    #(#data_new_call_args,)*
                )?;
                let (pda, bump) = self.pda(#pda_caller_outer)?;
                assert_eq!(
                    &pda,
                    self.key(),
                    "Derived PDA does not match data for account {:#?}",
                    self.key()
                );
                let mut signer_seeds_vec = self.seeds(#seeds_caller)?;
                signer_seeds_vec.push(vec![bump]);
                let signer_seeds: Vec<&[u8]> = signer_seeds_vec.iter().map(AsRef::as_ref).collect();
                cpi::system::create_pda(
                    self.self_account.clone(),
                    self.self_account.program_id,
                    payer,
                    self.self_account.data.clone(),
                    signer_seeds,
                )
            }
        }
    }
}

/// Helper function to generate tokens for writing the function that returns the data type's primary key.
fn build_tokens_primary_key_seed(key: &syn::Ident, ty: &syn::Type) -> TokenStream {
    match quote::quote!(#ty).to_string().as_str() {
        "String" => quote::quote! {
            self.#key.as_bytes().to_vec()
        },
        "u8" => quote::quote! {
            vec![self.#key]
        },
        "u16" | "u32" | "u64" => quote::quote! {
            self.#key.to_le_bytes().to_vec()
        },
        "Pubkey" => quote::quote! {
            self.#key.to_vec()
        },
        _ => panic!(
            "Invalid primary key type! Only `String`, `u8`, `u16, `u32`, `u64`, and `Pubkey` are supported."
        ),
    }
}

/// Helper function that parses the fields of a struct to determine the function signature for
/// a `new(..) -> Self` function to create a record.
fn get_new_fn_args_for_record(
    fields: &Fields,
    autoincrement: bool,
    primary_key_ident: &Ident,
) -> (Vec<FnArg>, Vec<Ident>) {
    let mut data_new_fn_args: Vec<FnArg> = vec![];
    let mut data_new_call_args: Vec<Ident> = vec![];
    fields.iter().for_each(|f| match &f.ident {
        Some(ident) => {
            if !(autoincrement && ident == primary_key_ident) {
                data_new_call_args.push(ident.clone());
                data_new_fn_args.push(FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: ident.clone(),
                        subpat: None,
                    })),
                    colon_token: Colon::default(),
                    ty: Box::new(f.ty.clone()),
                }))
            }
        }
        None => (),
    });
    (data_new_fn_args, data_new_call_args)
}

/// Helper function that parses the fields of a struct to determine the function signature for
/// a `new(..) -> Self` function to create an account.
fn get_new_fn_args_for_account(fields: &Fields) -> (Vec<FnArg>, Vec<Ident>) {
    let mut data_new_fn_args: Vec<FnArg> = vec![];
    let mut data_new_call_args: Vec<Ident> = vec![];
    fields.iter().for_each(|f| match &f.ident {
        Some(ident) => {
            data_new_call_args.push(ident.clone());
            data_new_fn_args.push(FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: ident.clone(),
                    subpat: None,
                })),
                colon_token: Colon::default(),
                ty: Box::new(f.ty.clone()),
            }))
        }
        None => (),
    });
    (data_new_fn_args, data_new_call_args)
}

/// Builds the various arguments for the account's seeds.
///
/// Consider the return type of this function:
/// * 'seeds_inner': Conversions of all seeds into the `Vec<Vec<u8>>`.
/// * 'seeds_params': Input parameters required for any seeds declared that are parameter-like.
/// * 'seeds_params_tuple': Input parameters required represented in a tuple.
fn build_seeds_components_for_account(seeds: &Vec<Seed>) -> (TokenStream, Option<TokenStream>) {
    let mut seeds_inner_items: Vec<TokenStream> = vec![];
    let mut seeds_params_types: Vec<&Type> = vec![];
    let mut i = 0usize;
    seeds.into_iter().for_each(|s| match s {
        Seed::Lit { value } => seeds_inner_items.push(quote! { #value.as_bytes().to_vec() }),
        Seed::Field { ident } => {
            seeds_inner_items.push(quote! { borsh::BorshSerialize::try_to_vec(&self.#ident)? })
        }
        Seed::Param { ident: _, ty } => {
            let index_accessor = syn::Index::from(i);
            seeds_params_types.push(&ty);
            seeds_inner_items
                .push(quote! { borsh::BorshSerialize::try_to_vec(&seeds.#index_accessor)? });
            i += 1;
        }
    });
    // `Vec<u8>` conversions.
    let seeds_inner = quote! {
        vec![#(#seeds_inner_items,)*]
    };
    // Parameters for the `seeds(..)` function as a tuple.
    let seeds_params_tuple = match seeds_params_types.len() > 0 {
        true => Some(quote! { (#(#seeds_params_types,)*) }),
        false => None,
    };
    (seeds_inner, seeds_params_tuple)
}
