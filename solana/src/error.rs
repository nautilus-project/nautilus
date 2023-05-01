use num_traits::{FromPrimitive, ToPrimitive};
use solana_program::{
    decode_error::DecodeError,
    program_error::{PrintProgramError, ProgramError},
};
use splogger::{error, Splog};
use thiserror::Error;

/// Custom errors for Nautilus functionality. Convertible to `solana_program::program_error::ProgramError`.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum NautilusError {
    /// The inner data of an account could not be loaded. This usually means the account is empty.
    #[error("The inner data of an account could not be loaded. This usually means the account is empty.")]
    LoadDataFailed(String, String),
    /// The inner data of an account could not be deserialized. This usually means an account type mismatch.
    #[error("The inner data of an account could not be deserialized. This usually means an account type mismatch.")]
    DeserializeDataFailed(String, String),
    /// Nautilus couldn't write a new record to a table. This usually means an error with the primary key provided.
    #[error("Nautilus couldn't write a new record to a table. This usually means an error with the primary key provided.")]
    WriteRecordFailed(String),
    /// The underlying account for a `Mut<T>` declared object was not marked as mutable.
    #[error("The underlying account for a `Mut<T>` declared object was not marked as mutable.")]
    AccountNotMutable(String),
    /// The underlying account for a `Signer<T>` declared object was not marked as signer.
    #[error("The underlying account for a `Signer<T>` declared object was not marked as signer.")]
    AccountNotSigner(String),
    /// The underlying account for a `Create<T>` declared object already exists.
    #[error("The underlying account for a `Create<T>` declared object already exists.")]
    AccountExists(String),
}

impl<T> DecodeError<T> for NautilusError {
    fn type_of() -> &'static str {
        "NautilusError"
    }
}

impl FromPrimitive for NautilusError {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            200 => Some(Self::LoadDataFailed(String::default(), String::default())),
            201 => Some(Self::DeserializeDataFailed(
                String::default(),
                String::default(),
            )),
            202 => Some(Self::WriteRecordFailed(String::default())),
            203 => Some(Self::AccountNotMutable(String::default())),
            204 => Some(Self::AccountNotSigner(String::default())),
            205 => Some(Self::AccountExists(String::default())),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}

impl ToPrimitive for NautilusError {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Self::LoadDataFailed(..) => Some(200),
            Self::DeserializeDataFailed(..) => Some(201),
            Self::WriteRecordFailed(..) => Some(202),
            Self::AccountNotMutable(..) => Some(203),
            Self::AccountNotSigner(..) => Some(204),
            Self::AccountExists(..) => Some(205),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().map(|n| n as u64)
    }
}

impl PrintProgramError for NautilusError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            Self::LoadDataFailed(state_type, pubkey) => {
                error!(
                    "Failed to load {} data from account: {}",
                    state_type, pubkey
                );
                error!("Could not borrow account data");
                error!(
                    "Failed to load {} data from account: {}",
                    state_type, pubkey
                );
            }
            Self::DeserializeDataFailed(state_type, pubkey) => {
                error!(
                    "Failed to deserialize {} data from account: {}",
                    state_type, pubkey
                );
                error!("Could not deserialize");
                error!(
                    "Failed to deserialize {} data from account: {}",
                    state_type, pubkey
                );
            }
            Self::WriteRecordFailed(table_name) => {
                error!("Failed to create a new record for table: {}", table_name);
            }
            Self::AccountNotMutable(pubkey) => error!(
                "This account was marked with `Mut<T>` but was not passed in as mutable: {}",
                pubkey
            ),
            Self::AccountNotSigner(pubkey) => error!(
                "This account was marked with `Signer<T>` but was not passed in as signer: {}",
                pubkey
            ),
            Self::AccountExists(pubkey) => error!(
                "This account was marked with `Create<T>` but it exists already: {}",
                pubkey
            ),
        }
    }
}

impl From<NautilusError> for ProgramError {
    fn from(e: NautilusError) -> Self {
        e.print::<NautilusError>();
        ProgramError::Custom(e.to_u32().unwrap())
    }
}
