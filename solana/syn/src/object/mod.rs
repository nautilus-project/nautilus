pub mod data;
pub mod parser;
pub mod source;

use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::{Field, Ident, ItemStruct, ItemEnum};

use crate::entry::required_account::RequiredAccount;

use self::{parser::{parse_item_struct, NautilusObjectConfig}, data::impl_nautilus_data};

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
                RequiredAccount::derive_object_type(&self.ident.to_string(), config.is_mut), 
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
        let ident = &ast.ident;
        let data_ident = Ident::new(&(ident.to_string() + "NautilusData"), Span::call_site());
        let object_config = match &ast.object_config {
            Some(object_config) => object_config,
            None => panic!(
                "No object_config was derived for this Nautilus table: {}",
                ident.to_string()
            ),
        };
        let data_fields = object_config.data_fields.iter().map(|f| Field {
            attrs: vec![],
            ..f.clone()
        });

        let nautilus_data_impl = impl_nautilus_data(
            &ident, 
            &data_ident, 
            data_fields.clone().collect(),
            object_config.autoincrement_enabled,
            &object_config.primary_key_ident, 
            &object_config.primary_key_ty,
        );

        quote! {
            #[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
            pub struct #data_ident {
                #(#data_fields,)*
            }

            pub type #ident<'a> = Table<'a, #data_ident>;
            
            #nautilus_data_impl
        }
        .into()
    }
}
