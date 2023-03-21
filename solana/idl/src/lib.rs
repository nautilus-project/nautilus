mod accounts;
mod instructions;
mod types;
mod util;

pub use accounts::*;
pub use instructions::*;
pub use types::*;
pub use util::*;

#[derive(
    Clone,
    Debug,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Idl {
    pub version: String,
    pub name: String,
    pub instructions: Vec<crate::instructions::IdlInstruction>,
    pub accounts: Vec<crate::accounts::IdlAccount>,
    pub types: Vec<crate::types::IdlType>,
    pub metadata: IdlMetadata,
}

impl Idl {
    pub fn new(
        version: &str,
        name: &str,
        instructions: Vec<crate::instructions::IdlInstruction>,
        accounts: Vec<crate::accounts::IdlAccount>,
        types: Vec<crate::types::IdlType>,
        metadata: IdlMetadata,
    ) -> Self {
        Self {
            version: version.to_string(),
            name: name.to_string(),
            instructions,
            accounts,
            types,
            metadata,
        }
    }

    pub fn write(&self) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(format!("{}.json", &self.name)).unwrap();
        let json_string = serde_json::to_string(&self).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }
}

#[derive(
    Clone,
    Debug,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct IdlMetadata {
    origin: String,
    address: String,
}

impl IdlMetadata {
    pub fn new(address: &str) -> Self {
        Self {
            origin: "nautilus".to_string(),
            address: address.to_string(),
        }
    }
}
