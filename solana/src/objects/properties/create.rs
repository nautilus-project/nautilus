use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{Mint, NautilusData};

use super::{signer::NautilusSigner, NautilusAccountInfo};

#[derive(Clone)]
pub struct Create<'a, T>
where
    T: NautilusAccountInfo,
{
    pub fee_payer: Box<AccountInfo<'a>>,
    pub system_program: Box<AccountInfo<'a>>,
    pub rent: Box<AccountInfo<'a>>,
    pub self_account: Box<T>,
}

impl<'a, T> Create<'a, T>
where
    T: NautilusAccountInfo,
{
    pub fn new(
        fee_payer: Box<AccountInfo<'a>>,
        system_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
        self_account: Box<T>,
    ) -> Self {
        Self {
            fee_payer,
            system_program,
            rent,
            self_account,
        }
    }
}

impl<T> NautilusAccountInfo for Create<'_, T>
where
    T: NautilusAccountInfo,
{
    fn key(&self) -> &Pubkey {
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

    fn owner(&self) -> &Pubkey {
        self.self_account.owner()
    }

    fn span(&self) -> usize {
        self.self_account.span()
    }
}

impl<T> NautilusSigner for Create<'_, T> where T: NautilusAccountInfo {}

pub trait NautilusCreate: Clone {
    fn create(&mut self) -> ProgramResult;
    fn create_with_payer(&mut self, payer: impl NautilusSigner) -> ProgramResult;
}

pub trait NautilusCreateMint: Clone {
    fn create(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner,
        freeze_authority: Option<impl NautilusAccountInfo>,
    ) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner,
        freeze_authority: Option<impl NautilusAccountInfo>,
        payer: impl NautilusSigner,
    ) -> ProgramResult;
}

pub trait NautilusCreateMetadata: Clone {
    fn create(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: impl NautilusAccountInfo,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
    ) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: impl NautilusAccountInfo,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
        payer: impl NautilusSigner,
    ) -> ProgramResult;
}

pub trait NautilusCreateToken: Clone {
    fn create(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
        freeze_authority: Option<impl NautilusAccountInfo>,
    ) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
        freeze_authority: Option<impl NautilusAccountInfo>,
        payer: impl NautilusSigner,
    ) -> ProgramResult;
}

pub trait NautilusCreateAssociatedTokenAccount: Clone {
    fn create(
        &mut self,
        mint: impl NautilusAccountInfo,
        owner: impl NautilusAccountInfo,
    ) -> ProgramResult;

    fn create_with_payer(
        &mut self,
        mint: impl NautilusAccountInfo,
        owner: impl NautilusAccountInfo,
        payer: impl NautilusSigner,
    ) -> ProgramResult;
}

pub trait NautilusCreateRecord<T>: Clone
where
    T: NautilusData,
{
    fn create_record(&mut self, data: T) -> ProgramResult;
    fn create_record_with_payer(&mut self, data: T, payer: impl NautilusSigner) -> ProgramResult;
}
