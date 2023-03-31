use cargo_toml::Manifest;
use convert_case::{Case::Pascal, Casing};
use nautilus_idl::{
    IdlAccount, IdlAccountType, IdlAccountTypeField, IdlType, IdlTypeType, IdlTypeTypeField,
};
use proc_macro2::Span;
use quote::quote;
use shank_macro_impl::krate::CrateContext;
use syn::{
    AngleBracketedGenericArguments, Fields, FnArg, Ident, Item, ItemFn, Pat, PathArguments, Type,
    TypePath, UseTree,
};
use syn::{Meta, NestedMeta};

use crate::object::ObjectEntryConfig;
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
) -> (Ident, Vec<(Ident, Type)>, Ident, Vec<CallContext>, ItemFn) {
    let mut modified_fn = function.clone();
    let mut new_inputs = Vec::new();
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
            FnArg::Typed(arg) => match *arg.pat {
                Pat::Ident(ref pat_ident) => {
                    let (type_string, is_create, is_signer, ty_with_lifetimes) =
                        parse_type(&arg.ty);
                    for obj in nautilus_objects {
                        if obj.ident == &type_string {
                            let mut nautilus_obj = obj.clone();
                            nautilus_obj.entry_config = Some(ObjectEntryConfig {
                                arg_ident: pat_ident.ident.clone(),
                                is_create,
                                is_signer,
                            });
                            let mut new_arg = arg.clone();
                            new_arg.ty = Box::new(ty_with_lifetimes);
                            new_inputs.push(FnArg::Typed(new_arg));
                            return CallContext::Nautilus(nautilus_obj);
                        }
                    }
                    variant_args.push((pat_ident.ident.clone(), *arg.ty.clone()));
                    new_inputs.push(FnArg::Typed(arg.clone()));
                    return CallContext::Arg(pat_ident.ident.clone());
                }
                _ => panic!("Error parsing function."),
            },
            _ => panic!("Error parsing function."),
        })
        .collect();

    modified_fn.sig.inputs = syn::punctuated::Punctuated::from_iter(new_inputs.into_iter());
    modified_fn
        .sig
        .generics
        .params
        .push(syn::parse_quote! { 'a });

    (
        variant_ident,
        variant_args,
        call_ident,
        call_context,
        modified_fn,
    )
}

pub fn parse_type(ty: &Type) -> (String, bool, bool, Type) {
    let mut is_create = false;
    let mut is_signer = false;
    let mut child_type = None;

    if let Type::Path(TypePath { path, .. }) = &ty {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "Create" {
                is_create = true;
                child_type = derive_child_type(&segment.arguments)
            } else if segment.ident == "Signer" {
                is_signer = true;
                child_type = derive_child_type(&segment.arguments)
            }
        }
    }

    let type_name = if is_create || is_signer {
        if let Some(t) = &child_type {
            format!("{}", quote! { #t })
        } else {
            panic!("Could not parse provided type: {:#?}", ty);
        }
    } else {
        format!("{}", quote! { #ty })
    };

    let mut ty_with_lifetimes = ty.clone();
    if let Type::Path(ref mut type_path) = ty_with_lifetimes {
        if let Some(ref mut segment) = type_path.path.segments.first_mut() {
            let lifetime = syn::Lifetime::new("'a", proc_macro2::Span::call_site());
            if (is_create || is_signer) && child_type.is_some() {
                let mut child_ty_with_lifetime = child_type.unwrap();
                if let Type::Path(ref mut type_path) = child_ty_with_lifetime {
                    if let Some(ref mut child_segment) = type_path.path.segments.first_mut() {
                        if let PathArguments::AngleBracketed(ref mut args) = child_segment.arguments
                        {
                            insert_lifetime_first(args, &lifetime);
                        } else {
                            child_segment.arguments = new_angle_bracketed_args(lifetime.clone())
                        }
                    }
                }
                segment.arguments =
                    PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: vec![
                            syn::GenericArgument::Lifetime(lifetime),
                            syn::GenericArgument::Type(child_ty_with_lifetime),
                        ]
                        .into_iter()
                        .collect(),
                        gt_token: Default::default(),
                    });
            } else {
                match &mut segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        insert_lifetime_first(args, &lifetime);
                    }
                    _ => segment.arguments = new_angle_bracketed_args(lifetime),
                };
            }
        }
    }

    (type_name, is_create, is_signer, ty_with_lifetimes)
}

fn derive_child_type(arguments: &PathArguments) -> Option<Type> {
    if let PathArguments::AngleBracketed(args) = arguments {
        if let Some(first_arg) = args.args.first() {
            if let syn::GenericArgument::Type(t) = first_arg {
                return Some(t.clone());
            }
        }
    }
    None
}

fn new_angle_bracketed_args(lifetime: syn::Lifetime) -> PathArguments {
    PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Default::default(),
        args: vec![syn::GenericArgument::Lifetime(lifetime)]
            .into_iter()
            .collect(),
        gt_token: Default::default(),
    })
}

fn insert_lifetime_first(args: &mut AngleBracketedGenericArguments, lifetime: &syn::Lifetime) {
    args.args
        .insert(0, syn::GenericArgument::Lifetime(lifetime.clone()));
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
