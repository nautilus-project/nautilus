use std::fmt;

use solana_program::{msg, program_error::ProgramError};

/// Custom errors for Nautilus functionality. Convertible to `solana_program::program_error::ProgramError`.
#[derive(Clone, Debug)]
#[repr(u32)]
pub enum NautilusError {
    /// The inner data of an account could not be loaded. This usually means the account is empty.
    LoadDataFailed(String, String),
    /// The inner data of an account could not be deserialized. This usually means an account type mismatch.
    DeserializeDataFailed(String, String),
    /// Nautilus couldn't write a new record to a table. This usually means an error with the primary key provided.
    WriteRecordFailed(String),
    /// The underlying account for a `Mut<T>` declared object was not marked as mutable.
    AccountNotMutable(String),
    /// The underlying account for a `Signer<T>` declared object was not marked as signer.
    AccountNotSigner(String),
    /// The underlying account for a `Create<T>` declared object already exists.
    AccountExists(String),
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
            NautilusError::AccountNotMutable(pubkey) => write!(
                f,
                "  [Error]: This account was marked with `Mut<T>` but was not passed in as mutable: {}",
                pubkey
            ),
            NautilusError::AccountNotSigner(pubkey) => write!(
                f,
                "  [Error]: This account was marked with `Signer<T>` but was not passed in as signer: {}",
                pubkey
            ),
            NautilusError::AccountExists(pubkey) => write!(
                f,
                "  [Error]: This account was marked with `Create<T>` but it exists already: {}",
                pubkey
            )
        }
    }
}

impl From<NautilusError> for ProgramError {
    fn from(e: NautilusError) -> Self {
        ProgramError::Custom(match e {
            NautilusError::LoadDataFailed(..) => 0x200,
            NautilusError::DeserializeDataFailed(..) => 0x201,
            NautilusError::WriteRecordFailed(..) => 0x202,
            NautilusError::AccountNotMutable(..) => 0x203,
            NautilusError::AccountNotSigner(..) => 0x204,
            NautilusError::AccountExists(..) => 0x205,
        })
    }
}
