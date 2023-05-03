//! Spawns all objects from Nautilus's `src/objects/.` into `syn::ItemStruct` types
//! for `syn` processing.
use syn::{punctuated::Punctuated, Field, FieldsNamed, Ident, ItemStruct};

use super::NautilusObject;

/// Enum vehicle used to build a `syn::Field`.
enum SourceField {
    ProgramId,
    AccountInfo,
    Metadata,
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
}

impl From<&SourceField> for Field {
    /// Converts from a `SourceField` to a `syn::Field`.
    fn from(value: &SourceField) -> Self {
        match value {
            SourceField::ProgramId => source_field("program_id"),
            SourceField::AccountInfo => source_field("account_info"),
            SourceField::Metadata => source_field("metadata"),
            SourceField::SystemProgram => source_field("system_program"),
            SourceField::TokenProgram => source_field("token_program"),
            SourceField::AssociatedTokenProgram => source_field("associated_token_program"),
            SourceField::TokenMetadataProgram => source_field("token_metadata_program"),
        }
    }
}

/// Helper function to build a named `syn::Field` from defaults.
fn source_field(field_name: &str) -> Field {
    Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new(field_name, proc_macro2::Span::call_site())),
        colon_token: Some(Default::default()),
        ty: syn::parse_quote!(AccountInfo<'a>),
    }
}

/// Helper function to build a named `syn::ItemStruct` from defaults.
fn source_struct(name: &str, source_fields: Vec<SourceField>) -> ItemStruct {
    let ident = Ident::new(name, proc_macro2::Span::call_site());
    let fields = {
        let mut fields = Punctuated::new();
        for f in &source_fields {
            fields.push(f.into())
        }
        FieldsNamed {
            brace_token: Default::default(),
            named: fields,
        }
    };
    ItemStruct {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        struct_token: Default::default(),
        ident,
        generics: Default::default(),
        fields: syn::Fields::Named(fields),
        semi_token: None,
    }
}

/// Uses helpers to return a vector of all Nautilus objects from Nautilus's `src/objects/.`
/// as `syn::ItemStruct` types.
pub fn source_nautilus_objects() -> Vec<NautilusObject> {
    [
        source_struct(
            "NautilusIndex",
            vec![SourceField::ProgramId, SourceField::AccountInfo],
        ),
        source_struct(
            "Wallet",
            vec![SourceField::AccountInfo, SourceField::SystemProgram],
        ),
        source_struct(
            "Mint",
            vec![SourceField::AccountInfo, SourceField::TokenProgram],
        ),
        source_struct(
            "Metadata",
            vec![SourceField::AccountInfo, SourceField::TokenMetadataProgram],
        ),
        source_struct(
            "AssociatedTokenAccount",
            vec![
                SourceField::AccountInfo,
                SourceField::TokenProgram,
                SourceField::AssociatedTokenProgram,
            ],
        ),
        source_struct(
            "Token",
            vec![
                SourceField::AccountInfo,
                SourceField::Metadata,
                SourceField::TokenProgram,
                SourceField::TokenMetadataProgram,
            ],
        ),
        source_struct(
            "Nft",
            vec![
                SourceField::AccountInfo,
                SourceField::Metadata,
                SourceField::TokenProgram,
                SourceField::TokenMetadataProgram,
            ],
        ),
    ]
    .into_iter()
    .map(|s| NautilusObject::from_item_struct(s, super::NautilusObjectType::Account))
    .collect()
}

/// Returns a vector with just the string names of all objects from Nautilus's `src/objects/.`.
pub fn source_nautilus_names() -> Vec<String> {
    vec![
        "NautilusIndex".to_string(),
        "Wallet".to_string(),
        "Mint".to_string(),
        "Metadata".to_string(),
        "AssociatedTokenAccount".to_string(),
        "Token".to_string(),
        "Nft".to_string(),
    ]
}
