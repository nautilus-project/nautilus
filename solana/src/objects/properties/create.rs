use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{Mint, NautilusData, NautilusMut};

use super::{signer::NautilusSigner, NautilusAccountInfo};

/// The trait that allow for creation of new on-chain instances of the Nautilus object.
///
/// `NautilusCreate<'_>` is meant to be dynamic for any implementing object to use to create new instances.
///
/// Object-specific create traits then drive this trait's c
pub trait NautilusCreate<'a> {
    /// Creates a new on-chain instance of this Nautilus object with the **transaction fee payer** as the rent payer.
    fn create(&mut self) -> ProgramResult;

    /// Creates a new on-chain instance of this Nautilus object with a provided rent payer.
    fn create_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult;
}

/// * This documentation will be updated when this trait is updated *
pub trait NautilusCreateMint<'a> {
    fn create(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        token_22: Option<bool>,
    ) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        token_22: Option<bool>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

/// * This documentation will be updated when this trait is updated *
pub trait NautilusCreateMetadata<'a> {
    #[allow(clippy::too_many_arguments)]
    fn create(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
    ) -> ProgramResult;

    #[allow(clippy::too_many_arguments)]
    fn create_with_payer(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

/// * This documentation will be updated when this trait is updated *
pub trait NautilusCreateToken<'a> {
    #[allow(clippy::too_many_arguments)]
    fn create(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult;

    #[allow(clippy::too_many_arguments)]
    fn create_with_payer(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

/// * This documentation will be updated when this trait is updated *
pub trait NautilusCreateAssociatedTokenAccount<'a> {
    fn create(&mut self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        mint: Mint<'a>,
        owner: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

/// * This documentation will be updated when this trait is updated *
pub trait NautilusCreateRecord<'a, T: NautilusData> {
    fn create_record(&mut self) -> ProgramResult;
    fn create_record_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult;
}

/// The struct to wrap an object so that it has the necessary accounts to create an on-chain instance of itself.
/// A user wraps their object `T` in `Create<'_, T>` in order to make accessible the transaction fee payer,
/// the System Program, and the Rent Sysvar.
///
/// The transaction fee payer can be included by default whenever they provide a signature for transaction fees, and
/// the System Program and Rent Sysvar are always read-only, which means the addition of these accounts will never hinder
/// Sealevel parallel execution.
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
    ) -> Self {
        Self {
            fee_payer,
            system_program,
            rent,
            self_account,
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
