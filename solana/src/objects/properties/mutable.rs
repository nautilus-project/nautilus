use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

/// The trait that ensures an object's underlying `AccountInfo` must be mutable.
pub trait NautilusMut<'a>: NautilusAccountInfo<'a> {}

/// The struct to wrap an object so that it adheres to the `NautilusMut<'_>` trait.
/// A user wraps their object `T` in `Mut<T>` in order to comply with various method constraints and ensure the underlying account is marked as mutable.
#[derive(Clone)]
pub struct Mut<T>
where
    T: Clone,
{
    pub self_account: T,
}

impl<T> Mut<T>
where
    T: Clone,
{
    pub fn new(self_account: T) -> Self {
        Self { self_account }
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Mut<T>
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

impl<'a, T> NautilusMut<'a> for Mut<T> where T: NautilusAccountInfo<'a> {}
