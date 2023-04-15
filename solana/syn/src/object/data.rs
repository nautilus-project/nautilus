use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{token::Colon, Field, FnArg, Ident, Pat, PatIdent, PatType, Type};

pub fn impl_nautilus_data(
    ident: &Ident,
    data_ident: &Ident,
    data_fields: Vec<Field>,
    autoincrement: bool,
    primary_key_ident: &Ident,
    primary_key_ty: &Type,
) -> TokenStream {
    let table_name = ident.to_string();
    let tokens_primary_key_seed = build_tokens_primary_key_seed(primary_key_ident, primary_key_ty);
    let nautilus_create_obj_ident = &Ident::new(
        &("NautilusCreate".to_owned() + &ident.to_string()),
        Span::call_site(),
    );
    let mut data_fn_args: Vec<FnArg> = vec![];
    let mut data_fn_arg_idents: Vec<Ident> = vec![];
    data_fields.into_iter().for_each(|f| match f.ident {
        Some(_) => {
            let ident = f.ident.unwrap();
            if !(autoincrement && &ident == primary_key_ident) {
                data_fn_arg_idents.push(ident.clone());
                data_fn_args.push(FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident,
                        subpat: None,
                    })),
                    colon_token: Colon::default(),
                    ty: Box::new(f.ty.clone()),
                }))
            }
        }
        None => (),
    });

    let (tokens_create, tokens_create_with_payer) = match autoincrement {
        true => {
            let tokens_lookup_primary_key = quote! {let #primary_key_ident = {
                let primary_key = match self.get_next_count(#data_ident::TABLE_NAME) {
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
            (
                quote! {
                    fn create(&mut self, #(#data_fn_args,)*) -> ProgramResult {
                        #tokens_lookup_primary_key
                        self.create_record(#data_ident { #primary_key_ident, #(#data_fn_arg_idents,)* })
                    }
                },
                quote! {
                    fn create_with_payer(&mut self, #(#data_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult {
                        #tokens_lookup_primary_key
                        self.create_record_with_payer(#data_ident { #primary_key_ident, #(#data_fn_arg_idents,)* }, payer)
                    }
                },
            )
        }
        false => (
            quote! {
                fn create(&mut self, #(#data_fn_args,)*) -> ProgramResult {
                    self.create_record(#data_ident { #(#data_fn_arg_idents,)* })
                }
            },
            quote! {
                fn create_with_payer(&mut self, #(#data_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult {
                    self.create_record_with_payer(#data_ident { #(#data_fn_arg_idents,)* }, payer)
                }
            },
        ),
    };

    quote! {
        impl NautilusData for #data_ident {
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
            fn create(&mut self, #(#data_fn_args,)*) -> ProgramResult;
            fn create_with_payer(&mut self, #(#data_fn_args,)* payer: impl NautilusSigner<'a>) -> ProgramResult;
        }

        impl<'a> #nautilus_create_obj_ident<'a> for Create<'a, #ident<'a>> {
            #tokens_create
            #tokens_create_with_payer
        }
    }
}

pub fn build_tokens_primary_key_seed(key: &syn::Ident, ty: &syn::Type) -> TokenStream {
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
