use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::NautilusError, NautilusMut};

use super::NautilusAccountInfo;

/// The trait that ensures an object's underlying `AccountInfo` must be a signer.
pub trait NautilusSigner<'a>: NautilusAccountInfo<'a> {}

/// The struct to wrap an object so that it adheres to the `NautilusSigner<'_>` trait.
/// A user wraps their object `T` in `Signer<T>` in order to comply with various method constraints and ensure the underlying account is marked as a signer.
#[derive(Clone)]
pub struct Signer<T>
where
    T: Clone,
{
    pub self_account: T,
}

impl<'a, T> Signer<T>
where
    T: Clone + NautilusAccountInfo<'a>,
{
    pub fn new(self_account: T) -> Result<Self, ProgramError> {
        match self_account.is_signer() {
            true => Ok(Self { self_account }),
            false => Err(NautilusError::AccountNotSigner(self_account.key().to_string()).into()),
        }
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Signer<T>
where
    T: NautilusAccountInfo<'a>,
{
    fn account_info(&self) -> Box<AccountInfo<'a>> {
        self.self_account.account_info()
    }

    fn key(&self) -> &'a Pubkey {
        self.self_account.key()
    }

    fn is_signer(&self) -> bool {
        self.self_account.is_signer()
    }

    fn is_writable(&self) -> bool {
        self.self_account.is_writable()
    }

    fn lamports(&self) -> u64 {
        self.self_account.lamports()
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.self_account.mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.self_account.owner()
    }

    fn span(&self) -> Result<usize, ProgramError> {
        self.self_account.span()
    }
}

impl<'a, T> NautilusMut<'a> for Signer<T> where T: NautilusAccountInfo<'a> {}

impl<'a, T> NautilusSigner<'a> for Signer<T> where T: NautilusAccountInfo<'a> {}
