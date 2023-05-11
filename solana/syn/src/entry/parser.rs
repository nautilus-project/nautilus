//! Parses information about the user's entire crate.
use cargo_toml::Manifest;
use convert_case::{Case::Pascal, Casing};
use nautilus_idl::idl_type_def::IdlTypeDef;
use proc_macro2::Span;
use quote::quote;
use shank_macro_impl::krate::CrateContext;
use syn::{FnArg, Ident, Item, ItemFn, Pat, PathArguments, Type, TypePath, UseTree};
use syn::{Meta, NestedMeta};

use crate::object::source::source_nautilus_objects;
use crate::object::ObjectEntryConfig;
use crate::object::{NautilusObject, NautilusObjectType};

use super::entry_variant::CallContext;

/// Parses metadata from the user's `Cargo.toml`
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

/// Uses Metaplex's `shank_macro_impl` to parse all of the contents of the
/// user's crate.
///
/// It uses this information to build the rest of the IDL (accounts and types),
/// and return all defined Nautilus objects annotated with a Nautilus derive
/// macro.
///
/// Consider the return type: (`Vec<NautilusObject>`, `Vec<IdlTypeDef>`,
/// `Vec<IdlTypeDef>`):
/// * `Vec<NautilusObject>`: All Nautilus objects defined in the crate using
///   Nautilus derive macros.
/// * `Vec<IdlTypeDef>` (first): All accounts for the IDL (Nautilus objects).
/// * `Vec<IdlTypeDef>` (second): All types for the IDL (non-Nautilus objects
///   defined in the crate).
pub fn parse_crate_context() -> (Vec<NautilusObject>, Vec<IdlTypeDef>, Vec<IdlTypeDef>) {
    let root = std::env::current_dir().unwrap().join("src/lib.rs");
    let crate_context = CrateContext::parse(root).expect(
        "Failed to detect `src/lib.rs`. Are you sure you've built your program with `--lib` ?",
    );

    let mut idl_accounts: Vec<IdlTypeDef> = vec![];
    let mut idl_types: Vec<IdlTypeDef> = vec![];

    let mut nautilus_objects: Vec<NautilusObject> = crate_context
        .structs()
        .filter_map(|s| {
            if let Some(attr) = s.attrs.iter().find(|attr| attr.path.is_ident("derive")) {
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    let matched_macro =
                        meta_list
                            .nested
                            .iter()
                            .find_map(|nested_meta| match nested_meta {
                                NestedMeta::Meta(Meta::Path(path)) => {
                                    if path.is_ident("Table") {
                                        Some(NautilusObjectType::Record)
                                    } else if path.is_ident("State") {
                                        Some(NautilusObjectType::Account)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            });

                    if let Some(nautilus_ty) = matched_macro {
                        let nautilus_obj: NautilusObject =
                            NautilusObject::from_item_struct(s.clone(), nautilus_ty);
                        let i = &nautilus_obj;
                        idl_accounts.push(i.into());
                        return Some(nautilus_obj);
                    }
                }
            }
            idl_types.push(s.into());
            None
        })
        .collect();

    nautilus_objects.extend(source_nautilus_objects());

    crate_context.enums().for_each(|e| idl_types.push(e.into()));

    (nautilus_objects, idl_accounts, idl_types)
}

/// Parses all required information from a user's defined function.
///
/// All Nautilus objects - both from the source crate itself and the user's
/// crate - are provided as a parameter in order to decipher whether or not a
/// function's parameter is a Nautilus object.
///
/// Consider the return type: (`Ident`, `Vec<(Ident, Type)>`, `Ident`,
/// `Vec<CallContext>`):
/// * `Ident` (first): The identifier of this instruction's variant in the
///   program instruction enum.
/// * `Vec<(Ident, Type)>` (second): The arguments required for this
///   instruction's variant in the program instruction enum.
/// * `Ident` (second): The "call context" of each declared parameter in the
///   user's defined function signature.
/// * `Vec<CallContext>`: The "call context" of each declared parameter in the
///   user's defined function signature.
///
/// You can see these return values are directly used to build a
/// `NautilusEntrypointEnumVariant`.
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
            FnArg::Typed(arg) => match *arg.pat {
                Pat::Ident(ref pat_ident) => {
                    let (type_string, is_create, is_signer, is_mut) = parse_type(&arg.ty);
                    for obj in nautilus_objects {
                        if obj.ident == &type_string {
                            let mut nautilus_obj = obj.clone();
                            nautilus_obj.entry_config = Some(ObjectEntryConfig {
                                arg_ident: pat_ident.ident.clone(),
                                is_create,
                                is_signer,
                                is_mut,
                            });
                            return CallContext::Nautilus(nautilus_obj);
                        }
                    }
                    variant_args.push((pat_ident.ident.clone(), *arg.ty.clone()));
                    return CallContext::Arg(pat_ident.ident.clone());
                }
                _ => panic!("Error parsing function."),
            },
            _ => panic!("Error parsing function."),
        })
        .collect();
    (variant_ident, variant_args, call_ident, call_context)
}

/// Parses the type of a parameter of a user's defined function signature.
pub fn parse_type(ty: &Type) -> (String, bool, bool, bool) {
    let mut is_create = false;
    let mut is_signer = false;
    let mut is_mut = false;
    let mut is_pda = false;
    let mut child_type = None;
    if let Type::Path(TypePath { path, .. }) = &ty {
        if let Some(segment) = path.segments.first() {
            if segment.ident == "Create" {
                is_create = true;
                is_signer = true;
                is_mut = true;
                (child_type, is_pda) = derive_child_type(&segment.arguments)
            } else if segment.ident == "Signer" {
                is_signer = true;
                (child_type, is_pda) = derive_child_type(&segment.arguments)
            } else if segment.ident == "Mut" {
                is_mut = true;
                (child_type, is_pda) = derive_child_type(&segment.arguments)
            } else if segment.ident == "Record" || segment.ident == "Account" {
                is_pda = true;
                (child_type, _) = derive_child_type(&segment.arguments)
            }
        }
    }
    is_mut = is_create || is_signer || is_mut;
    if is_pda {
        is_signer = false;
    }
    let type_name = if is_create || is_signer || is_mut || is_pda {
        if let Some(t) = &child_type {
            format!("{}", quote! { #t })
        } else {
            panic!("Could not parse provided type: {:#?}", ty);
        }
    } else {
        let mut new_t = ty.clone();
        remove_lifetimes_from_type(&mut new_t);
        format!("{}", quote! { #new_t })
    };
    (type_name, is_create, is_signer, is_mut)
}

/// Derives the child type of a compound object with angle-bracket generic
/// arguments, ie: `Object<T>`.
fn derive_child_type(arguments: &PathArguments) -> (Option<Type>, bool) {
    if let PathArguments::AngleBracketed(args) = arguments {
        for arg in &args.args {
            if let syn::GenericArgument::Type(ty) = arg {
                let mut new_ty = ty.clone();
                remove_lifetimes_from_type(&mut new_ty);
                if let Type::Path(TypePath { path, .. }) = &new_ty {
                    if let Some(segment) = path.segments.first() {
                        if segment.ident == "Record" || segment.ident == "Account" {
                            return derive_child_type(&segment.arguments);
                        }
                    }
                }
                return (Some(new_ty), false);
            }
        }
    }
    (None, false)
}

/// Removes all lifetime specifiers from a `syn::Type`.
///
/// This is not currently used to replace code but to generate a string
/// representation of a type.
fn remove_lifetimes_from_type(t: &mut Type) {
    match t {
        Type::Path(ref mut tp) => {
            if let Some(segment) = tp.path.segments.last_mut() {
                if let PathArguments::AngleBracketed(ref mut abga) = segment.arguments {
                    if abga.args.len() == 1
                        && abga
                            .args
                            .iter()
                            .any(|arg| matches!(arg, syn::GenericArgument::Lifetime(_)))
                    {
                        segment.arguments = PathArguments::None;
                    }
                }
            }
        }
        Type::Reference(ref mut tr) => {
            tr.lifetime = None;
            remove_lifetimes_from_type(&mut tr.elem);
        }
        Type::Paren(ref mut tp) => {
            remove_lifetimes_from_type(&mut tp.elem);
        }
        Type::Tuple(ref mut tt) => {
            for elem in &mut tt.elems {
                remove_lifetimes_from_type(elem);
            }
        }
        _ => (),
    }
}

/// Is the item `use super::*;`
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
