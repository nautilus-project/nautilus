//! Parses a user's defined struct.
use proc_macro2::TokenStream;
use syn::{Fields, Ident, ItemStruct, Type};

use crate::object::seeds::SeedParser;

use super::{
    default_instructions::{DefaultInstruction, DefaultInstructionParser},
    seeds::Seed,
    NautilusObjectType,
};

/// Object configurations for either a `Record<T>` or `Account<T>`.
#[derive(Clone, Debug)]
pub enum NautilusObjectConfig {
    /// Object configurations for a `Record<T>`.
    RecordConfig {
        table_name: String,
        data_fields: Fields,
        autoincrement_enabled: bool,
        primary_key_ident: Ident,
        primary_key_ty: Type,
        authorities: Vec<Ident>,
        default_instructions: Vec<DefaultInstruction>,
    },
    /// Object configurations for an `Account<T>`.
    AccountConfig {
        discrminator_str: String,
        data_fields: Fields,
        authorities: Vec<Ident>,
        seeds: Vec<Seed>,
    },
}

pub struct NautilusAccountFieldAttributes {
    pub is_primary_key: bool,
    pub autoincrement_enabled: bool,
    pub is_authority: bool,
}

/// Parse out a `syn::ItemStruct` according to whichever type of Nautilus object is
/// attempting to be created from the macro.
pub fn parse_item_struct(
    item_struct: &ItemStruct,
    nautilus_ty: NautilusObjectType,
) -> Option<NautilusObjectConfig> {
    let ident_string = item_struct.ident.to_string();
    let discrminator_str = ident_string.clone().to_lowercase();
    let data_fields = item_struct.fields.clone();

    match nautilus_ty {
        NautilusObjectType::Record => {
            let default_instructions =
                parse_top_level_attributes_for_record(&ident_string, &item_struct.attrs);

            let mut primary_key_ident_opt: Option<(Ident, Type)> = None;
            let mut autoincrement_enabled: bool = true;
            let mut authorities: Vec<Ident> = vec![];
            let mut _optionized_struct_fields: Vec<(Ident, TokenStream, TokenStream)> = vec![];

            for f in data_fields.iter() {
                let parsed_attributes = parse_field_attributes(&f);
                if !parsed_attributes.autoincrement_enabled {
                    autoincrement_enabled = parsed_attributes.autoincrement_enabled;
                }
                if parsed_attributes.is_primary_key {
                    primary_key_ident_opt = Some((f.ident.clone().unwrap(), f.ty.clone()));
                }
                if parsed_attributes.is_authority {
                    authorities.push(f.ident.clone().unwrap());
                }
            }

            let (primary_key_ident, primary_key_ty) = match primary_key_ident_opt {
                Some((ident, ty)) => (ident, ty),
                None => return None,
            };

            Some(NautilusObjectConfig::RecordConfig {
                table_name: discrminator_str,
                data_fields,
                autoincrement_enabled,
                primary_key_ident,
                primary_key_ty,
                authorities,
                default_instructions,
            })
        }
        NautilusObjectType::Account => {
            let seeds = parse_top_level_attributes_for_account(&item_struct.attrs);

            let mut authorities: Vec<Ident> = vec![];
            let mut _optionized_struct_fields: Vec<(Ident, TokenStream, TokenStream)> = vec![];

            for f in data_fields.iter() {
                let parsed_attributes = parse_field_attributes(&f);
                if parsed_attributes.is_authority {
                    authorities.push(f.ident.clone().unwrap());
                }
            }

            Some(NautilusObjectConfig::AccountConfig {
                discrminator_str,
                data_fields,
                authorities,
                seeds,
            })
        }
    }
}

/// Parses the field attributes of the struct, such as `#[authority]` and `#[primary_key(..)]`.
pub fn parse_field_attributes(field: &syn::Field) -> NautilusAccountFieldAttributes {
    let mut is_primary_key = false;
    let mut autoincrement_enabled = true;
    let mut is_authority = false;
    for attr in field.attrs.iter() {
        if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("primary_key") {
                is_primary_key = true;
                for nested_meta in &meta_list.nested {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta_name_value)) =
                        nested_meta
                    {
                        if meta_name_value.path.is_ident("autoincrement") {
                            if let syn::Lit::Bool(lit_bool) = &meta_name_value.lit {
                                autoincrement_enabled = lit_bool.value();
                            }
                        }
                    }
                }
            }
        } else if attr.path.is_ident("primary_key") {
            is_primary_key = true;
        } else if attr.path.is_ident("authority") {
            is_authority = true;
        }
    }
    NautilusAccountFieldAttributes {
        is_primary_key,
        autoincrement_enabled,
        is_authority,
    }
}

/// Attempts to parse the top-level macro attributes for `#[derive(nautilus::Table)]`, such
/// as `#[default_instructions(..)]`.
pub fn parse_top_level_attributes_for_record(
    struct_name: &str,
    attrs: &Vec<syn::Attribute>,
) -> Vec<DefaultInstruction> {
    let mut default_instructions = Vec::new();
    for attr in attrs.iter() {
        if attr.path.is_ident("default_instructions") {
            let mut parsed_instructions = DefaultInstructionParser::parse(attr, struct_name)
                .expect("Invalid format for `default_instructions` attribute");
            default_instructions.append(&mut parsed_instructions.instructions);
        }
    }
    default_instructions
}

/// Attempts to parse the top-level macro attributes for `#[derive(nautilus::State)]`, such
/// as `#[seeds(..)]`.
pub fn parse_top_level_attributes_for_account(attrs: &Vec<syn::Attribute>) -> Vec<Seed> {
    let mut seeds = Vec::new();
    for attr in attrs.iter() {
        if attr.path.is_ident("seeds") {
            let mut parsed_seeds: SeedParser =
                syn::parse2(attr.tokens.clone()).expect("Invalid format for `seeds` attribute");
            seeds.append(&mut parsed_seeds.seeds);
        };
    }
    seeds
}
