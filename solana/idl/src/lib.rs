//
//
// ----------------------------------------------------------------
//                          Nautilus IDL
// ----------------------------------------------------------------
//
// Much of this IDL crate is inspired by or borrowed directly from Metaplex's
// Shank.
//
// Nautilus and its contributors intend to introduce a shared, dynamic IDL crate
// to be used by anyone across the Solana community.
//
// All credit to contributors at Metaplex for anything borrowed in this crate.
//
// Shank: https://github.com/metaplex-foundation/shank
//
//
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use self::{idl_instruction::IdlInstruction, idl_metadata::IdlMetadata, idl_type_def::IdlTypeDef};

pub mod converters;
pub mod idl_instruction;
pub mod idl_metadata;
pub mod idl_nautilus_config;
pub mod idl_type;
pub mod idl_type_def;
pub mod util;

/// The entire IDL itself.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Idl {
    pub version: String,
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instructions: Vec<IdlInstruction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<IdlTypeDef>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<IdlTypeDef>,
    pub metadata: IdlMetadata,
}

impl Idl {
    pub fn new(
        version: String,
        name: String,
        instructions: Vec<IdlInstruction>,
        accounts: Vec<IdlTypeDef>,
        types: Vec<IdlTypeDef>,
        metadata: IdlMetadata,
    ) -> Self {
        Self {
            version,
            name,
            instructions,
            accounts,
            types,
            metadata,
        }
    }

    pub fn write_to_json(&self, dir_path: &str) -> std::io::Result<()> {
        if dir_path != "." {
            fs::create_dir_all(dir_path)?;
        }
        let idl_path = Path::join(Path::new(dir_path), &format!("{}.json", &self.name));
        let mut file = File::create(idl_path)?;
        let json_string = serde_json::to_string(&self)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }
}
