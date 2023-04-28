use serde::{Deserialize, Serialize};

/// An IDL type enum for converting from Rust types to IDL type.
///
/// Copied from Shank: https://github.com/metaplex-foundation/shank
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IdlType {
    Array(Box<IdlType>, usize),
    Bool,
    Bytes,
    Defined(String),
    I128,
    I16,
    I32,
    I64,
    I8,
    Option(Box<IdlType>),
    Tuple(Vec<IdlType>),
    PublicKey,
    String,
    U128,
    U16,
    U32,
    U64,
    U8,
    Vec(Box<IdlType>),
    HashMap(Box<IdlType>, Box<IdlType>),
    BTreeMap(Box<IdlType>, Box<IdlType>),
    HashSet(Box<IdlType>),
    BTreeSet(Box<IdlType>),
}

impl From<&syn::Type> for IdlType {
    fn from(value: &syn::Type) -> Self {
        match value {
            syn::Type::Path(type_path) => {
                let ident = &type_path.path.segments.last().unwrap().ident;
                let arguments = &type_path.path.segments.last().unwrap().arguments;
                match ident.to_string().as_str() {
                    "Vec" => {
                        if let syn::PathArguments::AngleBracketed(args) = arguments {
                            if args.args.len() == 1 {
                                let inner_type = args.args.first().unwrap();
                                if let syn::GenericArgument::Type(inner_type) = inner_type {
                                    return IdlType::Vec(Box::new(Self::from(inner_type)));
                                }
                            }
                        }
                        panic!("Expected Vec<T>.");
                    }
                    "bool" => IdlType::Bool,
                    "u8" => IdlType::U8,
                    "u16" => IdlType::U16,
                    "u32" => IdlType::U32,
                    "u64" => IdlType::U64,
                    "u128" => IdlType::U128,
                    "i8" => IdlType::I8,
                    "i16" => IdlType::I16,
                    "i32" => IdlType::I32,
                    "i64" => IdlType::I64,
                    "i128" => IdlType::I128,
                    "String" => IdlType::String,
                    "Pubkey" => IdlType::PublicKey,
                    "Bytes" => IdlType::Bytes,
                    _ => IdlType::Defined(ident.to_string()),
                }
            }
            syn::Type::Array(array_type) => {
                let size = match &array_type.len {
                    syn::Expr::Lit(lit) => {
                        if let syn::Lit::Int(int_lit) = &lit.lit {
                            int_lit.base10_parse().unwrap()
                        } else {
                            panic!("Expected array length as an integer literal.");
                        }
                    }
                    _ => panic!("Expected array length as an integer literal."),
                };
                IdlType::Array(Box::new(Self::from(&*array_type.elem)), size)
            }
            syn::Type::Tuple(tuple_type) => {
                let types = tuple_type
                    .elems
                    .iter()
                    .map(|elem| Self::from(elem))
                    .collect();
                IdlType::Tuple(types)
            }
            syn::Type::Reference(type_reference) => {
                if let syn::Type::Slice(slice_type) = &*type_reference.elem {
                    if let syn::Type::Path(ref_type_path) = &*slice_type.elem {
                        let ident = &ref_type_path.path.segments.last().unwrap().ident;
                        if ident == "u8" {
                            IdlType::Bytes
                        } else {
                            panic!("Expected &[u8].");
                        }
                    } else {
                        panic!("Expected &[u8].");
                    }
                } else {
                    Self::from(&*type_reference.elem)
                }
            }
            syn::Type::Paren(paren_type) => Self::from(&*paren_type.elem),
            _ => panic!("Unsupported type."),
        }
    }
}
