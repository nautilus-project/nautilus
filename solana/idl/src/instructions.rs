#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
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

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlInstructionAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
}

impl IdlInstructionAccount {
    pub fn new(name: &str, is_mut: bool, is_signer: bool, desc: &str) -> Self {
        Self {
            name: name.to_string(),
            is_mut,
            is_signer,
            desc: desc.to_string(),
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlInstructionArg {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: IdlInstructionArgType,
}

impl IdlInstructionArg {
    pub fn new(name: &str, arg_type: IdlInstructionArgType) -> Self {
        Self {
            name: name.to_string(),
            arg_type,
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlInstructionArgType {
    pub defined: String,
}

impl IdlInstructionArgType {
    pub fn new(defined: &str) -> Self {
        Self {
            defined: defined.to_string(),
        }
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, serde::Deserialize, serde::Serialize)]
pub struct IdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub discriminant_type: String,
    pub value: u8,
}

impl IdlInstructionDiscriminant {
    pub fn new(value: u8) -> Self {
        Self {
            discriminant_type: "u8".to_string(),
            value,
        }
    }
}
