//! Traits used for creating Nautilus objects.
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::NautilusError, NautilusMut};

use super::{signer::NautilusSigner, NautilusAccountInfo};

/// The struct to wrap an object so that it has the necessary accounts to create
/// an on-chain instance of itself. A user wraps their object `T` in `Create<'_,
/// T>` in order to make accessible the transaction fee payer, the System
/// Program, and the Rent Sysvar.
///
/// The transaction fee payer can be included by default whenever they provide a
/// signature for transaction fees, and the System Program and Rent Sysvar are
/// always read-only, which means the addition of these accounts will never
/// hinder Sealevel parallel execution.
#[derive(Clone)]
pub struct Create<'a, T>
where
    T: NautilusAccountInfo<'a> + 'a,
{
    pub fee_payer: Box<AccountInfo<'a>>,
    pub system_program: Box<AccountInfo<'a>>,
    pub rent: Box<AccountInfo<'a>>,
    pub self_account: T,
}

impl<'a, T> Create<'a, T>
where
    T: NautilusAccountInfo<'a> + 'a,
{
    pub fn new(
        fee_payer: Box<AccountInfo<'a>>,
        system_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
        self_account: T,
    ) -> Result<Self, ProgramError> {
        match check_account_does_not_exist(&self_account) {
            true => Ok(Self {
                fee_payer,
                system_program,
                rent,
                self_account,
            }),
            false => Err(NautilusError::AccountExists(self_account.key().to_string()).into()),
        }
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Create<'a, T>
where
    T: NautilusAccountInfo<'a> + 'a,
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

impl<'a, T> NautilusMut<'a> for Create<'a, T> where T: NautilusAccountInfo<'a> + 'a {}

impl<'a, T> NautilusSigner<'a> for Create<'a, T> where T: NautilusAccountInfo<'a> + 'a {}

fn check_account_does_not_exist<'a>(account: &impl NautilusAccountInfo<'a>) -> bool {
    let account_info = account.account_info();
    account_info.lamports() == 0
        && account_info.owner.eq(&solana_program::system_program::ID)
        && account_info.data_is_empty()
}
