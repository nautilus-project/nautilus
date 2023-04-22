use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    Create, Metadata, Mint, NautilusAccountInfo, NautilusCreateMetadata, NautilusCreateMint,
    NautilusCreateToken, NautilusSigner,
};

pub mod associated_token;
pub mod metadata;
pub mod mint;

#[derive(Clone)]
pub struct Token<'a> {
    pub mint: Mint<'a>,
    pub metadata: Metadata<'a>,
}

impl<'a> Token<'a> {
    pub fn new(
        mint_account: Box<AccountInfo<'a>>,
        metadata_account: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
    ) -> Self {
        Self {
            mint: Mint::new(mint_account, token_program),
            metadata: Metadata::new(metadata_account, token_metadata_program),
        }
    }

    pub fn load(
        mint_account: Box<AccountInfo<'a>>,
        metadata_account: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        Ok(Self {
            mint: Mint::load(mint_account, token_program)?,
            metadata: Metadata::load(metadata_account, token_metadata_program)?,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for Token<'a> {
    fn account_info(&self) -> Box<AccountInfo<'a>> {
        self.mint.account_info()
    }

    fn key(&self) -> &'a Pubkey {
        self.mint.account_info.key
    }

    fn is_signer(&self) -> bool {
        self.mint.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.mint.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.mint.account_info.lamports()
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.mint.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.mint.account_info.owner
    }

    fn span(&self) -> usize {
        self.mint.account_info.data_len()
    }
}

impl<'a> From<Token<'a>> for Mint<'a> {
    fn from(value: Token<'a>) -> Self {
        value.mint
    }
}

impl<'a> From<Create<'a, Token<'a>>> for Create<'a, Mint<'a>> {
    fn from(value: Create<'a, Token<'a>>) -> Self {
        Self {
            self_account: value.self_account.into(),
            fee_payer: value.fee_payer,
            rent: value.rent,
            system_program: value.system_program,
        }
    }
}

impl<'a> From<Token<'a>> for Metadata<'a> {
    fn from(value: Token<'a>) -> Self {
        value.metadata
    }
}

impl<'a> From<Create<'a, Token<'a>>> for Create<'a, Metadata<'a>> {
    fn from(value: Create<'a, Token<'a>>) -> Self {
        Self {
            self_account: value.self_account.into(),
            fee_payer: value.fee_payer,
            rent: value.rent,
            system_program: value.system_program,
        }
    }
}

impl<'a> NautilusCreateToken<'a> for Create<'a, Token<'a>> {
    fn create(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult {
        let mut create_mint: Create<Mint> = self.clone().into();
        let mut create_metadata: Create<Metadata> = self.clone().into();
        create_mint.create(decimals, mint_authority.clone(), freeze_authority)?;
        create_metadata.create(
            title,
            symbol,
            uri,
            self.self_account.mint.to_owned(),
            mint_authority,
            update_authority,
        )?;
        Ok(())
    }

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
    ) -> ProgramResult {
        let mut create_mint: Create<Mint> = self.clone().into();
        let mut create_metadata: Create<Metadata> = self.clone().into();
        create_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            freeze_authority,
            payer.clone(),
        )?;
        create_metadata.create_with_payer(
            title,
            symbol,
            uri,
            self.self_account.mint.to_owned(),
            mint_authority,
            update_authority,
            payer,
        )?;
        Ok(())
    }
}
