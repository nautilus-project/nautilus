use cargo_toml::Manifest;
use convert_case::{Case::Pascal, Casing};
use nautilus_idl::{
    IdlAccount, IdlAccountType, IdlAccountTypeField, IdlType, IdlTypeType, IdlTypeTypeField,
};
use proc_macro2::Span;
use quote::quote;
use shank_macro_impl::krate::CrateContext;
use syn::{Fields, FnArg, Ident, Item, ItemFn, Pat, Type, UseTree};
use syn::{Meta, NestedMeta};

use crate::object::{
    parser::{parse_field_attributes, parse_top_level_attributes},
    NautilusObject,
};

use super::entry_variant::CallContext;

pub fn parse_manifest() -> (String, String) {
    let manifest = Manifest::from_path("Cargo.toml")
        .expect("Failed to detect `Cargo.toml`. Is your Cargo.toml file structured properly ?");
    let package = manifest
        .package
        .expect("Failed to parse `Cargo.toml`. Is your Cargo.toml file structured properly ?");
    let crate_version = package
        .version
        .get()
        .expect("Failed to parse crate version from `Cargo.toml`. Did you provide one ?");
    (String::from(crate_version), package.name)
}

pub fn parse_crate_context() -> (Vec<NautilusObject>, Vec<IdlAccount>, Vec<IdlType>) {
    let root = std::env::current_dir().unwrap().join("src/lib.rs");
    let crate_context = CrateContext::parse(root).expect(
        "Failed to detect `src/lib.rs`. Are you sure you've built your program with `--lib` ?",
    );

    let mut idl_accounts: Vec<IdlAccount> = vec![];
    let mut idl_types: Vec<IdlType> = vec![];

    let mut nautilus_objects: Vec<NautilusObject> = crate_context
        .structs()
        .filter_map(|s| {
            if let Some(attr) = s.attrs.iter().find(|attr| attr.path.is_ident("derive")) {
                if let Ok(meta) = attr.parse_meta() {
                    if let Meta::List(meta_list) = meta {
                        if meta_list.nested.iter().any(|nested| {
                            if let NestedMeta::Meta(Meta::Path(path)) = nested {
                                path.is_ident("Nautilus")
                            } else {
                                idl_types.push(IdlType::new(
                                    &s.ident.to_string(),
                                    idl_type_type_from_struct_fields(&s.fields),
                                ));
                                false
                            }
                        }) {
                            let account_ident_string = s.ident.to_string();
                            idl_accounts.push(IdlAccount::new(
                                &account_ident_string,
                                idl_account_type_from_struct_fields(&s.fields),
                                parse_top_level_attributes(&account_ident_string, &s.attrs),
                            ));
                            return Some(s.clone().into());
                        }
                    }
                }
            }
            None
        })
        .collect();
    nautilus_objects.extend(NautilusObject::source_nautilus_objects());

    // TODO: Enums
    // crate_context.enums().iter().for_each(|e| idl_types.push(e.into()))

    (nautilus_objects, idl_accounts, idl_types)
}

pub fn parse_function(
    nautilus_objects: &Vec<NautilusObject>,
    function: ItemFn,
) -> (Ident, Vec<(Ident, Type)>, Ident, Vec<CallContext>) {
    let variant_ident = Ident::new(
        &function.sig.ident.to_string().to_case(Pascal),
        Span::call_site(),
    );
    let call_ident = function.sig.ident.clone();
    let mut variant_args = vec![];
    let call_context = function
        .sig
        .inputs
        .into_iter()
        .map(|input| match input {
            FnArg::Typed(pat_type) => match *pat_type.pat {
                Pat::Ident(ref pat_ident) => {
                    for obj in nautilus_objects {
                        if obj
                            .ident
                            .to_string()
                            .eq(&type_to_string(&pat_type.ty).unwrap())
                        {
                            let mut nautilus_obj = obj.clone();
                            nautilus_obj.arg_ident = Some(pat_ident.ident.clone());
                            return CallContext::Nautilus(nautilus_obj);
                        }
                    }
                    variant_args.push((pat_ident.ident.clone(), *pat_type.ty.clone()));
                    return CallContext::Arg(pat_ident.ident.clone());
                }
                _ => panic!("Error parsing function."),
            },
            _ => panic!("Error parsing function."),
        })
        .collect();
    (variant_ident, variant_args, call_ident, call_context)
}

pub fn is_use_super_star(item: &Item) -> bool {
    if let Item::Use(use_item) = item {
        if let UseTree::Path(use_path) = &use_item.tree {
            if let UseTree::Glob(_) = &*use_path.tree {
                return use_path.ident == Ident::new("super", use_path.ident.span());
            }
        }
    }
    false
}

pub fn type_to_string(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

//

pub fn idl_account_type_from_struct_fields<'a>(fields: &'a Fields) -> IdlAccountType {
    IdlAccountType::new(
        "struct",
        match fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_type = format!("{}", quote! { #field.ty });
                    let nautilus_attributes = parse_field_attributes(field);
                    IdlAccountTypeField::new(
                        &field_name,
                        &field_type,
                        nautilus_attributes.is_primary_key,
                        nautilus_attributes.is_authority,
                    )
                })
                .collect(),
            _ => vec![],
        },
    )
}

pub fn idl_type_type_from_struct_fields<'a>(fields: &'a Fields) -> IdlTypeType {
    IdlTypeType::new(
        "struct",
        match fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    let field_type = format!("{}", quote! { #field.ty });
                    IdlTypeTypeField::new(&field_name, &field_type)
                })
                .collect(),
            _ => vec![],
        },
    )
}
