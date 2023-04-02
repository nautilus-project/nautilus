use nautilus_idl::idl_nautilus_config::IdlTypeDefNautilusConfigDefaultInstruction;
use proc_macro2::{Span, TokenStream};
// use quote::quote;
use syn::{Fields, Ident, ItemStruct, Type};

#[derive(Clone, Debug)]
pub struct NautilusObjectConfig {
    pub table_name: String,
    pub data_fields: Fields,
    pub autoincrement_enabled: bool,
    pub primary_key: Ident,
    pub authorities: Vec<Ident>,
    pub default_instructions: Vec<IdlTypeDefNautilusConfigDefaultInstruction>,
}

pub struct NautilusAccountFieldAttributes {
    pub is_primary_key: bool,
    pub autoincrement_enabled: bool,
    pub is_authority: bool,
}

pub enum DefaultInstructions {
    Create,
    Delete,
    Update,
}

pub fn parse_item_struct(item_struct: &ItemStruct) -> Option<NautilusObjectConfig> {
    let ident_string = item_struct.ident.to_string();
    let _ident_optionized_struct_name =
        Ident::new(&(ident_string.clone() + "Optionized"), Span::call_site());

    let table_name = ident_string.clone().to_lowercase();
    let default_instructions = parse_top_level_attributes(&ident_string, &item_struct.attrs);

    let data_fields = item_struct.fields.clone();

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

        // let field_name = &f.ident;
        // let field_ty = &f.ty;
        // optionized_struct_fields.push(match parsed_attributes.is_primary_key {
        //     true => (
        //         field_name.clone().unwrap(),
        //         quote! { #field_ty },
        //         quote! { #field_name: #field_ty },
        //     ),
        //     false => (
        //         field_name.clone().unwrap(),
        //         quote! { std::option::Option<#field_ty> },
        //         quote! { #field_name: std::option::Option<#field_ty> },
        //     ),
        // });
    }

    let (primary_key, _) = match primary_key_ident_opt {
        Some((ident, ty)) => (ident, ty),
        None => return None,
    };

    Some(NautilusObjectConfig {
        table_name,
        data_fields,
        autoincrement_enabled,
        primary_key,
        authorities,
        default_instructions,
    })
}

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

pub fn parse_top_level_attributes(
    struct_name: &str,
    attrs: &Vec<syn::Attribute>,
) -> Vec<IdlTypeDefNautilusConfigDefaultInstruction> {
    let mut default_instructions = Vec::new();

    for attr in attrs.iter() {
        if let Ok(syn::Meta::List(ref meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("default_instructions") {
                for nested_meta in meta_list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = nested_meta {
                        let variant_string = path.get_ident().unwrap().to_string();
                        if variant_string.eq("Create") {
                            default_instructions.push(
                                IdlTypeDefNautilusConfigDefaultInstruction::Create(
                                    struct_name.to_string(),
                                ),
                            );
                        } else if variant_string.eq("Delete") {
                            default_instructions.push(
                                IdlTypeDefNautilusConfigDefaultInstruction::Delete(
                                    struct_name.to_string(),
                                ),
                            );
                        } else if variant_string.eq("Update") {
                            default_instructions.push(
                                IdlTypeDefNautilusConfigDefaultInstruction::Update(
                                    struct_name.to_string(),
                                ),
                            );
                        } else {
                            panic!("Unknown default instruction: {}", variant_string);
                        }
                    } else {
                        panic!("Invalid format for `default_instructions` attribute");
                    }
                }
            }
        }
    }

    default_instructions
}
