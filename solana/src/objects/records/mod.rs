//! The `Record<T>` Nautilus object and all associated trait implementations.
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi, error::NautilusError, Create, Mut, NautilusAccountInfo, NautilusIndex, NautilusMut,
    NautilusRecord, NautilusRecordData, NautilusSigner, NautilusTransferLamports, Signer, Wallet,
};

pub mod index;

/// The struct that allows you to treat a Program-Derived-Address (PDA) account
/// as a table record.
///
/// A user wraps their data type `T` with `Record<'_, T>` in order to combine
/// the data stored within the record and the accounts required to operate on
/// it.
///
/// The `account_info` field represents the PDA itself, while the `index` field
/// is one single account that accompanies a Nautilus program and keeps an index
/// of every table.
///
/// For more information on the `NautilusIndex<'_>` see the docs for that
/// struct.
#[derive(Clone)]
pub struct Record<'a, T>
where
    T: NautilusRecordData,
{
    pub program_id: &'a Pubkey,
    pub account_info: Box<AccountInfo<'a>>,
    pub index: NautilusIndex<'a>,
    pub data: Box<T>,
}

impl<'a, T> Record<'a, T>
where
    T: NautilusRecordData,
{
    /// Instantiate a new record without loading the account inner data from
    /// on-chain.
    pub fn new(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
        index: NautilusIndex<'a>,
    ) -> Self {
        Self {
            program_id,
            index,
            account_info,
            data: Box::<T>::default(),
        }
    }

    /// Instantiate a new record and load the account inner data from on-chain.
    pub fn load(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
        index: NautilusIndex<'a>,
    ) -> Result<Self, ProgramError> {
        let data = match T::try_from_slice(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    T::TABLE_NAME.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => Box::new(state_data),
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    T::TABLE_NAME.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            program_id,
            index,
            account_info,
            data,
        })
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Record<'a, T>
where
    T: NautilusRecordData,
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

impl<'a, T> NautilusRecord<'a> for Record<'a, T>
where
    T: NautilusRecordData,
{
    fn discriminator(&self) -> [u8; 8] {
        self.data.discriminator()
    }

    fn seeds(&self) -> Vec<Vec<u8>> {
        self.data.seeds()
    }

    fn pda(&self) -> (Pubkey, u8) {
        self.data.pda(self.program_id)
    }

    fn primary_key(&self) -> Vec<u8> {
        self.data.primary_key()
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        self.data.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.data.count_authorities()
    }
}

impl<'a, T> NautilusTransferLamports<'a> for Mut<Record<'a, T>>
where
    T: NautilusRecordData,
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

impl<'a, T> Create<'a, Record<'a, T>>
where
    T: NautilusRecordData,
{
    /// Allocate space for a record using the System Program.
    pub fn allocate(&self) -> ProgramResult {
        cpi::system::allocate(self.clone())
    }

    /// Create a new record.
    ///
    /// This function is specifically not named `create` because `create(&mut
    /// self, ..)` is added by the derive macro
    /// `#[derive(nautilus::Table)]`, which then drives this function.
    pub fn create_record(&mut self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        })?;
        let (pda, bump) = self.pda();
        assert_eq!(
            &pda,
            self.key(),
            "Derived PDA does not match data for account {:#?}",
            self.key()
        );
        let mut signer_seeds_vec = self.seeds();
        signer_seeds_vec.push(vec![bump]);
        let signer_seeds: Vec<&[u8]> = signer_seeds_vec.iter().map(AsRef::as_ref).collect();
        cpi::system::create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.self_account.data.clone(),
            signer_seeds,
        )
    }

    /// This function is the same as `create_record(&mut self, ..)` but allows
    /// you to specify a rent payer.
    pub fn create_record_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        let (pda, bump) = self.pda();
        assert_eq!(
            &pda,
            self.key(),
            "Derived PDA does not match data for account {:#?}",
            self.key()
        );
        let mut signer_seeds_vec = self.seeds();
        signer_seeds_vec.push(vec![bump]);
        let signer_seeds: Vec<&[u8]> = signer_seeds_vec.iter().map(AsRef::as_ref).collect();
        cpi::system::create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.self_account.data.clone(),
            signer_seeds,
        )
    }
}

impl<'a, T> NautilusRecord<'a> for Create<'a, Record<'a, T>>
where
    T: NautilusRecordData,
{
    fn discriminator(&self) -> [u8; 8] {
        self.self_account.discriminator()
    }

    fn seeds(&self) -> Vec<Vec<u8>> {
        self.self_account.seeds()
    }

    fn pda(&self) -> (Pubkey, u8) {
        self.self_account.pda()
    }

    fn primary_key(&self) -> Vec<u8> {
        self.self_account.primary_key()
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        self.self_account.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.self_account.count_authorities()
    }
}
