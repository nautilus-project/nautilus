pub mod impl_nautilus;
pub mod parser;
pub mod source;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, Ident, ItemStruct, ItemEnum};

use crate::entry::required_account::RequiredAccount;

use self::parser::{parse_item_struct, NautilusObjectConfig};

#[derive(Clone, Debug)]
pub struct NautilusObject {
    pub ident: Ident,
    pub raw_type: NautilusObjectRawType,
    pub entry_config: Option<ObjectEntryConfig>,
    pub object_config: Option<NautilusObjectConfig>,
}

#[derive(Clone, Debug)]
pub enum NautilusObjectRawType {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

#[derive(Clone, Debug)]
pub struct ObjectEntryConfig {
    pub arg_ident: Ident,
    pub is_create: bool,
    pub is_signer: bool,
    pub is_mut: bool,
}

impl NautilusObject {
    pub fn get_required_accounts(&self) -> (Vec<RequiredAccount>, Option<Vec<RequiredAccount>>) {
        match &self.entry_config {
            Some(config) => RequiredAccount::resolve_accounts(
                config.arg_ident.to_string(), 
                RequiredAccount::derive_object_type(&self.ident.to_string()), 
                config.is_create,
                config.is_signer,
                config.is_mut,
            ),
            None => panic!("Error: `get_required_accounts` was invoked before setting the value for `entry_config`!"),
        }
    }
}

impl From<&ItemStruct> for NautilusObject {
    fn from(value: &ItemStruct) -> Self {
        let ident = value.ident.clone();
        let object_config = parse_item_struct(value);
        Self {
            ident,
            raw_type: NautilusObjectRawType::Struct(value.clone()),
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
