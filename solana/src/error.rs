use std::fmt;

use solana_program::{msg, program_error::ProgramError};

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum NautilusError {
    LoadDataFailed(String, String),
    DeserializeDataFailed(String, String),
    WriteRecordFailed(String),
}

impl std::error::Error for NautilusError {}

impl fmt::Display for NautilusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NautilusError::LoadDataFailed(state_type, pubkey) => {
                msg!(
                    "  [Error]: Failed to load {} data from account: {}",
                    state_type,
                    pubkey
                );
                msg!("  [Error]: Could not borrow account data");
                write!(
                    f,
                    "Failed to load {} data from account: {}",
                    state_type, pubkey
                )
            }
            NautilusError::DeserializeDataFailed(state_type, pubkey) => {
                msg!(
                    "  [Error]: Failed to deserialize {} data from account: {}",
                    state_type,
                    pubkey
                );
                msg!("  [Error]: Could not deserialize");
                write!(
                    f,
                    "Failed to deserialize {} data from account: {}",
                    state_type, pubkey
                )
            }
            NautilusError::WriteRecordFailed(table_name) => {
                write!(
                    f,
                    "  [Error]: Failed to create a new record for table: {}",
                    table_name
                )
            }
        }
    }
}

impl From<NautilusError> for ProgramError {
    fn from(e: NautilusError) -> Self {
        ProgramError::Custom(match e {
            NautilusError::LoadDataFailed(..) => 0x200,
            NautilusError::DeserializeDataFailed(..) => 0x201,
            NautilusError::WriteRecordFailed(..) => 0x202,
        })
    }
}
