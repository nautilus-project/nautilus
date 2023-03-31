use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{Mint, NautilusAccountInfo, NautilusSigner};

#[derive(Clone)]
pub struct Create<'a, T: NautilusAccountInfo<'a> + 'a> {
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
    pub rent: AccountInfo<'a>,
    pub self_account: T,
}

impl<'a, T: NautilusAccountInfo<'a>> IntoAccountInfo<'a> for Create<'a, T> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a, T: NautilusAccountInfo<'a>> NautilusAccountInfo<'a> for Create<'a, T> {
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

    fn span(&self) -> usize {
        self.self_account.span()
    }
}

impl<'a, T: NautilusAccountInfo<'a> + 'a> NautilusSigner<'a> for Create<'a, T> {}

pub trait NautilusCreate<'a> {
    fn create(&self) -> ProgramResult;
    fn create_with_payer(&self, payer: impl NautilusSigner<'a>) -> ProgramResult;
}

pub trait NautilusCreateAssociatedTokenAccount<'a> {
    fn create(&self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult;

    fn create_with_payer(
        &self,
        mint: Mint<'a>,
        owner: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

pub trait NautilusCreateMint<'a> {
    fn create(
        &self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult;

    fn create_with_payer(
        &self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

pub trait NautilusCreateMetadata<'a> {
    fn create(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
    ) -> ProgramResult;

    fn create_with_payer(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

pub trait NautilusCreateToken<'a> {
    fn create(
        &self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult;

    fn create_with_payer(
        &self,
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
