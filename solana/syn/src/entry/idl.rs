use nautilus_idl::{
    idl_instruction::{
        IdlInstruction, IdlInstructionAccount, IdlInstructionArg, IdlInstructionDiscriminant,
    },
    idl_nautilus_config::IdlTypeDefNautilusConfig,
    idl_type_def::IdlTypeDef,
};

use crate::object::{parser::NautilusObjectConfig, NautilusObject, NautilusObjectRawType};

use super::{entry_variant::NautilusEntrypointEnumVariant, required_account::RequiredAccount};

impl From<&NautilusEntrypointEnumVariant> for IdlInstruction {
    fn from(value: &NautilusEntrypointEnumVariant) -> Self {
        let mut name = value.variant_ident.to_string();
        name.replace_range(..1, &name[..1].to_lowercase());
        IdlInstruction {
            name,
            accounts: value.required_accounts.iter().map(|a| a.into()).collect(),
            args: value
                .variant_args
                .iter()
                .map(|(ident, ty)| IdlInstructionArg::new(ident.to_string(), ty.into()))
                .collect(),
            discriminant: IdlInstructionDiscriminant::new(value.discriminant),
        }
    }
}

impl From<&RequiredAccount> for IdlInstructionAccount {
    fn from(value: &RequiredAccount) -> Self {
        Self {
            name: value.name.clone(),
            is_mut: value.is_mut,
            is_signer: value.is_signer,
            account_type: value.account_type.to_string(),
            desc: value.desc.clone(),
        }
    }
}

impl From<&NautilusObject> for IdlTypeDef {
    fn from(value: &NautilusObject) -> Self {
        let mut default_type_def: IdlTypeDef = match &value.raw_type {
            NautilusObjectRawType::Struct(raw) => raw.into(),
            NautilusObjectRawType::Enum(raw) => raw.into(),
        };
        match &value.object_config {
            Some(config) => default_type_def.config = Some(config.into()),
            None => (),
        }
        default_type_def
    }
}

impl From<&NautilusObjectConfig> for IdlTypeDefNautilusConfig {
    fn from(value: &NautilusObjectConfig) -> Self {
        Self {
            table_name: value.table_name.clone(),
            primary_key: value.primary_key.to_string(),
            autoincrement: value.autoincrement_enabled,
            authorities: value.authorities.iter().map(|a| a.to_string()).collect(),
            default_instructions: value.default_instructions.clone(),
        }
    }
}
