#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlType {
    pub name: String,
    #[serde(rename = "type")]
    pub idl_type: IdlTypeType,
}

impl IdlType {
    pub fn new(name: &str, idl_type: IdlTypeType) -> Self {
        Self {
            name: name.to_string(),
            idl_type,
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlTypeType {
    pub kind: String,
    pub fields: Vec<IdlTypeTypeField>,
}

impl IdlTypeType {
    pub fn new(kind: &str, fields: Vec<IdlTypeTypeField>) -> Self {
        Self {
            kind: kind.to_string(),
            fields,
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlTypeTypeField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_data_type: String,
}

impl IdlTypeTypeField {
    pub fn new(name: &str, field_data_type: &str) -> Self {
        Self {
            name: name.to_string(),
            field_data_type: field_data_type.to_string(),
        }
    }
}
