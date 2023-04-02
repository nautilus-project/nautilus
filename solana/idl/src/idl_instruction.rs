use serde::{Deserialize, Serialize};

use super::idl_type::IdlType;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlInstructionAccount>,
    pub args: Vec<IdlInstructionArg>,
    pub discriminant: IdlInstructionDiscriminant,
}

impl IdlInstruction {
    pub fn new(
        name: &str,
        accounts: Vec<IdlInstructionAccount>,
        args: Vec<IdlInstructionArg>,
        discriminant: IdlInstructionDiscriminant,
    ) -> Self {
        Self {
            name: name.to_string(),
            accounts,
            args,
            discriminant,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstructionAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    #[serde(rename = "type")]
    pub account_type: String,
    pub desc: String,
}

impl IdlInstructionAccount {
    pub fn new(
        name: String,
        is_mut: bool,
        is_signer: bool,
        account_type: String,
        desc: String,
    ) -> Self {
        Self {
            name,
            is_mut,
            is_signer,
            account_type,
            desc,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstructionArg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: IdlType,
}

impl IdlInstructionArg {
    pub fn new(name: String, arg_type: IdlType) -> Self {
        Self { name, arg_type }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub discriminant_type: IdlType,
    pub value: u8,
}

impl IdlInstructionDiscriminant {
    pub fn new(value: u8) -> Self {
        Self {
            discriminant_type: IdlType::U8,
            value,
        }
    }
}
