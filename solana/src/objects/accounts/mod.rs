//! The `Account<T>` Nautilus object and all associated trait implementations.
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::NautilusError, NautilusAccount, NautilusAccountData, NautilusAccountInfo, NautilusMut,
    NautilusTransferLamports,
};

/// The struct that allows you to create a plain-old program-derived address (PDA) account.
///
/// A user wraps their data type `T` with `Account<'_, T>` in order to combine the data stored within the
/// account and its underlying AccountInfo.
///
/// The `account_info` field represents the PDA itself.
#[derive(Clone)]
pub struct Account<'a, T>
where
    T: NautilusAccountData,
{
    pub program_id: &'a Pubkey,
    pub account_info: Box<AccountInfo<'a>>,
    pub data: Box<T>,
}

impl<'a, T> Account<'a, T>
where
    T: NautilusAccountData,
{
    /// Instantiate a new PDA without loading the account inner data from on-chain.
    pub fn new(program_id: &'a Pubkey, account_info: Box<AccountInfo<'a>>) -> Self {
        Self {
            program_id,
            account_info,
            data: Box::<T>::default(),
        }
    }

    /// Instantiate a new PDA and load the account inner data from on-chain.
    pub fn load(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data = match T::try_from_slice(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    T::DISCRIMINATOR_STR.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => Box::new(state_data),
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    T::DISCRIMINATOR_STR.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            program_id,
            account_info,
            data,
        })
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Account<'a, T>
where
    T: NautilusAccountData,
{
    fn account_info(&self) -> Box<AccountInfo<'a>> {
        self.account_info.clone()
    }

    fn key(&self) -> &'a Pubkey {
        self.account_info.key
    }

    fn is_signer(&self) -> bool {
        self.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.account_info.lamports()
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> Result<usize, ProgramError> {
        Ok(self.data.try_to_vec()?.len())
    }
}

impl<'a, T> NautilusAccount<'a> for Account<'a, T>
where
    T: NautilusAccountData,
{
    fn discriminator(&self) -> [u8; 8] {
        self.data.discriminator()
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        self.data.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.data.count_authorities()
    }
}

impl<'a, T> NautilusTransferLamports<'a> for Account<'a, T>
where
    T: NautilusAccountData,
{
    fn transfer_lamports(
        &self,
        to: impl NautilusMut<'a>,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info();
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}
