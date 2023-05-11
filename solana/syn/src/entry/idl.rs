//! Converters to allow for dissolving of Nautilus token-generation
//! configuration structs,
// such as `NautilusEntrypointEnum` and `NautilusEntrypointEnumVariant`,
// into IDL components.
use nautilus_idl::{
    idl_instruction::{
        IdlInstruction, IdlInstructionAccount, IdlInstructionArg, IdlInstructionDiscriminant,
    },
    idl_nautilus_config::{
        IdlSeed, IdlTypeDefNautilusConfig, IdlTypeDefNautilusConfigDefaultInstruction,
    },
    idl_type_def::IdlTypeDef,
};

use crate::object::{
    default_instructions::DefaultInstruction, parser::NautilusObjectConfig, seeds::Seed,
    NautilusObject, NautilusObjectRawType,
};

use super::{entry_variant::NautilusEntrypointEnumVariant, required_account::RequiredAccount};

/// Converts the `NautilusEntrypointEnumVariant` into an IDL instruction.
///
/// This will use the configurations from the variant to build the necessary
/// instruction in the IDL - including all required accounts.
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

/// Straightforward conversion from a `RequiredAccount` into its IDL
/// representation, including configs for `is_mut` and `is_signer`.
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

/// Straightforward conversion from a `NautilusObject` into its IDL type
/// definition.
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

/// Converts the object configurations for a `NautilusObject` into IDL
/// configurations.
///
/// These configurations are additional (and mostly optional) configs for the
/// client to use to perform certain actions such as SQL queries and
/// autoincrement.
impl From<&NautilusObjectConfig> for IdlTypeDefNautilusConfig {
    fn from(value: &NautilusObjectConfig) -> Self {
        match value {
            NautilusObjectConfig::RecordConfig {
                table_name,
                data_fields: _, // Unused in additional config.
                autoincrement_enabled,
                primary_key_ident,
                primary_key_ty: _, // Unused, points to field name instead.
                authorities,
                default_instructions,
            } => Self {
                discrminator_str: None,
                table_name: Some(table_name.clone()),
                primary_key: Some(primary_key_ident.to_string()),
                autoincrement: Some(*autoincrement_enabled),
                authorities: authorities.iter().map(|a| a.to_string()).collect(),
                default_instructions: default_instructions
                    .iter()
                    .map(|s| s.clone().into())
                    .collect(),
                seeds: vec![],
            },
            NautilusObjectConfig::AccountConfig {
                discrminator_str,
                data_fields: _, // Unused in additional config.
                authorities,
                seeds,
            } => Self {
                discrminator_str: Some(discrminator_str.clone()),
                table_name: None,
                primary_key: None,
                autoincrement: None,
                authorities: authorities.iter().map(|a| a.to_string()).collect(),
                default_instructions: vec![],
                seeds: seeds.iter().map(|s| s.into()).collect(),
            },
        }
    }
}

/// Converts a `Seed` from the `syn` crate into an `IdlSeed` from the `idl`
/// crate.
impl From<&Seed> for IdlSeed {
    fn from(value: &Seed) -> Self {
        match value {
            Seed::Lit { value } => IdlSeed::Lit {
                value: value.clone(),
            },
            Seed::Field { ident } => IdlSeed::Field {
                key: ident.to_string(),
            },
            Seed::Param { ident, ty } => IdlSeed::Param {
                key: ident.to_string(),
                value: ty.into(),
            },
        }
    }
}

/// Converts a `DefaultInstruction` from the `syn` crate into an
/// `IdlTypeDefNautilusConfigDefaultInstruction` from the `idl` crate.
impl From<DefaultInstruction> for IdlTypeDefNautilusConfigDefaultInstruction {
    fn from(value: DefaultInstruction) -> Self {
        match value {
            DefaultInstruction::Create {
                struct_ident: _,
                instruction,
            } => IdlTypeDefNautilusConfigDefaultInstruction::Create(instruction),
            DefaultInstruction::Delete {
                struct_ident: _,
                instruction,
            } => IdlTypeDefNautilusConfigDefaultInstruction::Delete(instruction),
            DefaultInstruction::Update {
                struct_ident: _,
                instruction,
            } => IdlTypeDefNautilusConfigDefaultInstruction::Update(instruction),
        }
    }
}
