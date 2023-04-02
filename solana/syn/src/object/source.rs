use syn::{punctuated::Punctuated, Field, FieldsNamed, Ident, ItemStruct};

use super::NautilusObject;

enum SourceField {
    AccountInfo,
    Metadata,
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
}

impl From<&SourceField> for Field {
    fn from(value: &SourceField) -> Self {
        match value {
            SourceField::AccountInfo => source_field("account_info"),
            SourceField::Metadata => source_field("metadata"),
            SourceField::SystemProgram => source_field("system_program"),
            SourceField::TokenProgram => source_field("token_program"),
            SourceField::AssociatedTokenProgram => source_field("associated_token_program"),
            SourceField::TokenMetadataProgram => source_field("token_metadata_program"),
        }
    }
}

fn source_field(field_name: &str) -> Field {
    Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new(field_name, proc_macro2::Span::call_site())),
        colon_token: Some(Default::default()),
        ty: syn::parse_quote!(AccountInfo<'a>),
    }
}

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

pub fn source_nautilus_objects() -> Vec<NautilusObject> {
    [
        &source_struct(
            "Wallet",
            vec![SourceField::AccountInfo, SourceField::SystemProgram],
        ),
        &source_struct(
            "Mint",
            vec![SourceField::AccountInfo, SourceField::TokenProgram],
        ),
        &source_struct(
            "Metadata",
            vec![SourceField::AccountInfo, SourceField::TokenMetadataProgram],
        ),
        &source_struct(
            "AssociatedTokenAccount",
            vec![
                SourceField::AccountInfo,
                SourceField::TokenProgram,
                SourceField::AssociatedTokenProgram,
            ],
        ),
        &source_struct(
            "Token",
            vec![
                SourceField::AccountInfo,
                SourceField::Metadata,
                SourceField::TokenProgram,
                SourceField::TokenMetadataProgram,
            ],
        ),
    ]
    .into_iter()
    .map(|s| s.into())
    .collect()
}
