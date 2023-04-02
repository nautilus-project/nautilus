use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlTypeDefNautilusConfig {
    pub table_name: String,
    pub primary_key: String,
    pub autoincrement: bool,
    pub authorities: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_instructions: Vec<IdlTypeDefNautilusConfigDefaultInstruction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum IdlTypeDefNautilusConfigDefaultInstruction {
    Create(String),
    Delete(String),
    Update(String),
}
