use serde::{Deserialize, Serialize};

use super::{idl_nautilus_config::IdlTypeDefNautilusConfig, idl_type::IdlType};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlTypeDef {
    pub name: String,
    #[serde(rename = "type")]
    pub idl_type: IdlTypeDefType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<IdlTypeDefNautilusConfig>,
}

impl IdlTypeDef {
    pub fn new(
        name: String,
        idl_type: IdlTypeDefType,
        config: Option<IdlTypeDefNautilusConfig>,
    ) -> Self {
        Self {
            name,
            idl_type,
            config,
        }
    }
}

impl From<&syn::ItemStruct> for IdlTypeDef {
    fn from(value: &syn::ItemStruct) -> Self {
        Self {
            name: value.ident.to_string(),
            idl_type: IdlTypeDefType::Struct {
                fields: value.fields.iter().map(|f| f.into()).collect(),
            },
            config: None,
        }
    }
}

impl From<&syn::ItemEnum> for IdlTypeDef {
    fn from(value: &syn::ItemEnum) -> Self {
        Self {
            name: value.ident.to_string(),
            idl_type: IdlTypeDefType::Enum {
                variants: value.variants.iter().map(|v| v.into()).collect(),
            },
            config: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefType {
    Struct { fields: Vec<IdlTypeStructField> },
    Enum { variants: Vec<IdlTypeEnumVariant> },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlTypeStructField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_data_type: IdlType,
}

impl IdlTypeStructField {
    pub fn new(name: String, field_data_type: IdlType) -> Self {
        Self {
            name,
            field_data_type,
        }
    }
}

impl From<&syn::Field> for IdlTypeStructField {
    fn from(value: &syn::Field) -> Self {
        let name = match &value.ident {
            Some(ident) => ident.to_string(),
            None => panic!("Expected named field."),
        };
        let ty = &value.ty;
        Self {
            name,
            field_data_type: ty.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdlTypeEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<IdlTypeEnumFields>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdlTypeEnumFields {
    Named(Vec<IdlTypeStructField>),
}

impl From<&syn::Variant> for IdlTypeEnumVariant {
    fn from(value: &syn::Variant) -> Self {
        let fields = match &value.fields {
            syn::Fields::Named(named_fields) => {
                let fields = named_fields
                    .named
                    .iter()
                    .map(|field| field.into())
                    .collect();
                Some(IdlTypeEnumFields::Named(fields))
            }
            syn::Fields::Unit => None,
            syn::Fields::Unnamed(_) => panic!("Expected named fields in enum variant."),
        };
        Self {
            name: value.ident.to_string(),
            fields,
        }
    }
}
