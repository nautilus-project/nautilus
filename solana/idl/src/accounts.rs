#[derive(
    Clone,
    Debug,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct IdlAccount {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: IdlAccountType,
    pub default_instructions: Vec<IdlAccountNautilusDefaultInstruction>,
}

impl IdlAccount {
    pub fn new(
        name: &str,
        data_type: IdlAccountType,
        default_instructions: Vec<IdlAccountNautilusDefaultInstructionType>,
    ) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            default_instructions: default_instructions
                .into_iter()
                .map(|d| d.to_string())
                .collect(),
        }
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
#[serde(rename_all = "camelCase")]
pub struct IdlAccountType {
    pub kind: String,
    pub fields: Vec<IdlAccountTypeField>,
}

impl IdlAccountType {
    pub fn new(kind: &str, fields: Vec<IdlAccountTypeField>) -> Self {
        Self {
            kind: kind.to_string(),
            fields,
        }
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
#[serde(rename_all = "camelCase")]
pub struct IdlAccountTypeField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_data_type: String,
    pub is_primary_key: bool,
    pub is_authority: bool,
}

impl IdlAccountTypeField {
    pub fn new(
        name: &str,
        field_data_type: &str,
        is_primary_key: bool,
        is_authority: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            field_data_type: field_data_type.to_string(),
            is_primary_key,
            is_authority,
        }
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
#[serde(rename_all = "camelCase")]
pub struct IdlAccountNautilusDefaultInstruction {
    pub default: String,
    pub instruction: String,
}

pub enum IdlAccountNautilusDefaultInstructionType {
    Create(String),
    Delete(String),
    Update(String),
}

impl IdlAccountNautilusDefaultInstructionType {
    fn to_string(&self) -> IdlAccountNautilusDefaultInstruction {
        match self {
            IdlAccountNautilusDefaultInstructionType::Create(struct_name) => {
                IdlAccountNautilusDefaultInstruction {
                    default: "create".to_string(),
                    instruction: "create".to_string() + struct_name,
                }
            }
            IdlAccountNautilusDefaultInstructionType::Delete(struct_name) => {
                IdlAccountNautilusDefaultInstruction {
                    default: "delete".to_string(),
                    instruction: "delete".to_string() + struct_name,
                }
            }
            IdlAccountNautilusDefaultInstructionType::Update(struct_name) => {
                IdlAccountNautilusDefaultInstruction {
                    default: "update".to_string(),
                    instruction: "update".to_string() + struct_name,
                }
            }
        }
    }
}
