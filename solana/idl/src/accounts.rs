#[derive(
    Clone,
    Debug,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct IdlAccount {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: crate::types::IdlTypeType,
}

impl IdlAccount {
    pub fn new(name: &str, data_type: crate::types::IdlTypeType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
        }
    }
}
