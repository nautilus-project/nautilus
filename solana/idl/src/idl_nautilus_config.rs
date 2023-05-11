use serde::{Deserialize, Serialize};

use crate::idl_type::IdlType;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IdlSeed {
    Lit { value: String },
    Field { key: String },
    Param { key: String, value: IdlType },
}

/// Additional Nautilus-specific IDL configurations.
///
/// These configurations are additional (and mostly optional) configs for the
/// client to use to perform certain actions such as SQL queries and
/// autoincrement.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlTypeDefNautilusConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrminator_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoincrement: Option<bool>,
    pub authorities: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_instructions: Vec<IdlTypeDefNautilusConfigDefaultInstruction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seeds: Vec<IdlSeed>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IdlTypeDefNautilusConfigDefaultInstruction {
    Create(String),
    Delete(String),
    Update(String),
}
