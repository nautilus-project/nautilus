pub mod impl_nautilus;
pub mod parser;

use nautilus_idl::IdlTypeType;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, Ident, ItemStruct};

use crate::entry::required_account::{name_to_ident, RequiredAccount};

use self::parser::{parse_item_struct, NautilusObjectConfig};

#[derive(Clone, Debug)]
pub struct NautilusObject {
    pub ident: Ident,
    pub entry_config: Option<ObjectEntryConfig>,
    pub object_config: Option<NautilusObjectConfig>,
}

#[derive(Clone, Debug)]
pub struct ObjectEntryConfig {
    pub arg_ident: Ident,
    pub is_create: bool,
    pub is_signer: bool,
}

impl NautilusObject {
    pub fn default(ident: Ident) -> Self {
        Self {
            ident,
            entry_config: None,
            object_config: None,
        }
    }

    pub fn source_nautilus_objects() -> Vec<Self> {
        [
            "Wallet",
            "Token",
            "Mint",
            "Metadata",
            "AssociatedTokenAccount",
        ]
        .into_iter()
        .map(|s| Self::default(name_to_ident(s)))
        .collect()
    }

    pub fn get_required_accounts(&self) -> (Vec<RequiredAccount>, Option<Vec<RequiredAccount>>) {
        match &self.entry_config {
            Some(config) => RequiredAccount::resolve_accounts(
                config.arg_ident.to_string(), 
                RequiredAccount::derive_object_type(&self.ident.to_string()), 
                config.is_create,
                config.is_signer,
            ),
            None => panic!("Error: `get_required_accounts` was invoked before setting the value for `entry_config`!"),
        }
    }

    pub fn to_idl_type(&self) -> IdlTypeType {
        todo!()
    }
}

impl From<ItemStruct> for NautilusObject {
    fn from(value: ItemStruct) -> Self {
        let ident = value.ident.clone();
        let object_config = Some(parse_item_struct(value));
        Self {
            ident,
            entry_config: None,
            object_config,
        }
    }
}

impl ToTokens for NautilusObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&NautilusObject> for TokenStream {
    fn from(ast: &NautilusObject) -> Self {
        let table_ident = &ast.ident;
        // let data_ident = syn::Ident::new(
        //     &(table_ident.to_string() + "NautilusData"),
        //     proc_macro2::Span::call_site(),
        // );
        let object_config = match &ast.object_config {
            Some(object_config) => object_config,
            None => panic!(
                "No object_config was derived for this Nautilus table: {}",
                table_ident.to_string()
            ),
        };
        let data_fields = object_config.data_fields.iter().map(|f| Field {
            attrs: vec![],
            ..f.clone()
        });

        // let data_nautilus_impl = quote!();
        // let table_nautilus_impl = quote!();

        quote! {
            #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
            pub struct #table_ident {
                #(#data_fields,)*
            }
            // #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
            // pub struct #data_ident {
            //     #data_fields
            // }
            // #data_nautilus_impl

            // #[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
            // pub struct #table_ident {
            //     pub program_id: &'a solana_program::pubkey::Pubkey,
            //     pub account_info: solana_program::account_info::AccountInfo<'a>,
            //     pub data: #data_ident,
            // }
            // #table_nautilus_impl
        }
        .into()
    }
}
