pub mod borsh_impl;
pub mod create;
pub mod data;
pub mod parser;

use nautilus_idl::IdlTypeType;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Fields, Ident, ItemStruct, Type,
};

use crate::entry::required_account::{name_to_ident, RequiredAccount};

use self::parser::parse_field_attributes;

#[derive(Clone, Debug)]
pub struct NautilusObject {
    pub ident: Ident,
    pub arg_ident: Option<Ident>,
    pub config: Option<NautilusConfig>,
}

#[derive(Clone, Debug)]
pub struct NautilusConfig {
    pub autoincrement_enabled: bool,
    pub table_name: String,
    pub fields: Fields,
    pub ident_optionized_struct_name: Ident,
    pub ident_primary_key: Ident,
    pub idents_authorities: Vec<Ident>,
    pub optionized_struct_fields: Vec<(Ident, TokenStream, TokenStream)>,
    pub ty_primary_key: Type,
}

impl NautilusObject {
    pub fn default(ident: Ident) -> Self {
        Self {
            ident,
            arg_ident: None,
            config: None,
        }
    }

    pub fn source_nautilus_objects() -> Vec<Self> {
        let source = [
            "Wallet",
            "Token",
            "Mint",
            "Metadata",
            "AssociatedTokenAccount",
        ];
        source
            .into_iter()
            .map(|s| Self::default(name_to_ident(s)))
            .collect()
    }

    pub fn get_required_accounts(&self) -> Vec<RequiredAccount> {
        match &self.arg_ident {
            Some(arg) => RequiredAccount::resolve_for_read(arg.to_string(), RequiredAccount::derive_object_type(&self.ident.to_string())),
            None => panic!("Error: `get_required_accounts` was invoked before setting the value for `arg_ident`!"),
        }
    }

    pub fn to_idl_type(&self) -> IdlTypeType {
        todo!()
    }
}

impl From<ItemStruct> for NautilusObject {
    fn from(value: ItemStruct) -> Self {
        let mut primary_key_ident_opt: Option<(Ident, Type)> = None;
        let mut autoincrement_enabled: bool = true;
        let mut idents_authorities: Vec<Ident> = vec![];

        let ident = value.ident.clone();
        let ident_string = ident.to_string();
        let ident_optionized_struct_name =
            Ident::new(&(ident_string.clone() + "Optionized"), Span::call_site());

        let table_name = ident_string.clone().to_lowercase();

        let mut optionized_struct_fields: Vec<(Ident, TokenStream, TokenStream)> = vec![];

        let fields = value.fields;

        for f in fields.iter() {
            let parsed_attributes = parse_field_attributes(&f);
            if !parsed_attributes.autoincrement_enabled {
                autoincrement_enabled = parsed_attributes.autoincrement_enabled;
            }
            if parsed_attributes.is_primary_key {
                primary_key_ident_opt = Some((f.ident.clone().unwrap(), f.ty.clone()));
            }
            if parsed_attributes.is_authority {
                idents_authorities.push(f.ident.clone().unwrap());
            }

            let field_name = &f.ident;
            let field_ty = &f.ty;
            optionized_struct_fields.push(match parsed_attributes.is_primary_key {
                true => (
                    field_name.clone().unwrap(),
                    quote! { #field_ty },
                    quote! { #field_name: #field_ty },
                ),
                false => (
                    field_name.clone().unwrap(),
                    quote! { std::option::Option<#field_ty> },
                    quote! { #field_name: std::option::Option<#field_ty> },
                ),
            });
        }

        let (ident_primary_key, ty_primary_key) = match primary_key_ident_opt {
            Some((ident, ty)) => (ident, ty),
            None => todo!("Throw an error on None value"),
        };

        let config = Some(NautilusConfig {
            autoincrement_enabled,
            table_name,
            fields,
            ident_optionized_struct_name,
            ident_primary_key,
            idents_authorities,
            optionized_struct_fields,
            ty_primary_key,
        });

        Self {
            ident,
            arg_ident: None,
            config,
        }
    }
}

impl Parse for NautilusObject {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ItemStruct::parse(input)?.into())
    }
}

impl ToTokens for NautilusObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

impl From<&NautilusObject> for TokenStream {
    fn from(_ast: &NautilusObject) -> Self {
        todo!()
    }
}
