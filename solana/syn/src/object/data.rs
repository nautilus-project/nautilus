use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{token::Colon, Fields, FnArg, Ident, Pat, PatIdent, PatType, Type};

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

pub fn impl_nautilus_data(
    ident: &Ident,
    fields: &Fields,
    table_name: &String,
    autoincrement: bool,
    primary_key_ident: &Ident,
    primary_key_ty: &Type,
) -> TokenStream {
    let nautilus_create_obj_ident = &Ident::new(
        &("NautilusCreate".to_owned() + &ident.to_string()),
        Span::call_site(),
    );

    let tokens_primary_key_seed = build_tokens_primary_key_seed(primary_key_ident, primary_key_ty);

    let (data_new_fn_args, data_new_call_args) =
        get_new_fn_args(fields, autoincrement, primary_key_ident);

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

        impl NautilusData for #ident {
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

        pub trait #nautilus_create_obj_ident<'a> {
            fn create(&mut self, #(#data_new_fn_args,)*) -> ProgramResult;
            fn create_with_payer(&mut self, #(#data_new_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult;
        }

        impl<'a> #nautilus_create_obj_ident<'a> for Create<'a, Record<'a, #ident>> {
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

fn get_new_fn_args(
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
