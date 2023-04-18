use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{token::Colon, Fields, FnArg, Ident, Pat, PatIdent, PatType, Type};

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
        true => {
            let tokens_lookup_primary_key = quote! {let #primary_key_ident = {
                let primary_key = match self.get_next_count(#ident ::TABLE_NAME) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let index_pk = entry.get_mut();
                        *index_pk += 1;
                        *index_pk as #primary_key_ty
                    },
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(1);
                        1
                    },
                };
                self.self_account.index.update(index_data)?;
                primary_key
            };};
            quote! {
                pub fn new(#(#data_new_fn_args,)*) -> ProgramResult {
                    #tokens_lookup_primary_key
                    self.create_record(#ident ::new(#(#data_new_call_args,)*))
                }
            }
        }
        false => quote! {
            pub fn new(#(#data_new_fn_args,)*) -> ProgramResult {
                self.create_record(#ident ::new(#(#data_new_call_args,)*))
            }
        },
    };

    let impl_clone = impl_clone(ident, fields);

    quote! {
        impl #ident {
            #data_new_fn
        }

        #impl_clone

        impl NautilusData for #ident {
            const TABLE_NAME: &'static str = #table_name;

            const AUTO_INCREMENT: bool = #autoincrement;

            fn primary_key<'a>(&self) -> &'a [u8] {
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
                self.create_record(self, #ident ::new(#(#data_new_call_args,)*))
            }

            fn create_with_payer(&mut self, #(#data_new_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult {
                self.create_record(self, #ident ::new(#(#data_new_call_args,)*))
            }
        }
    }
}

fn build_tokens_primary_key_seed(key: &syn::Ident, ty: &syn::Type) -> TokenStream {
    match quote::quote!(#ty).to_string().as_str() {
        "String" => quote::quote! {
            unsafe { self.#key.as_bytes() }
        },
        "u8" => quote::quote! {
            unsafe { std::slice::from_raw_parts(&self.#key, 1) }
        },
        "Pubkey" => quote::quote! {
            unsafe { self.#key.as_ref() }
        },
        _ => panic!(
            "Invalid primary key type! Only `String`, `u8`, and `Pubkey` are supported right now."
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

fn impl_clone(ident: &Ident, fields: &Fields) -> TokenStream {
    let clone_constructors = fields.iter().map(|f| {
        let ident = &f.ident.as_ref().unwrap();
        quote! { #ident: self.#ident.clone() }
    });
    quote! {
        impl Clone for #ident {
            fn clone(&self) -> Self {
                Self {
                    #(#clone_constructors,)*
                }
            }
        }
    }
}
