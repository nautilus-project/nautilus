use mpl_token_metadata::state::Creator;
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
};

use crate::{
    Create, Metadata, Mint, Mut, NautilusAccountInfo, NautilusCreateMetadata, NautilusCreateMint,
    NautilusCreateToken, NautilusMut, NautilusSigner, Signer,
};

pub mod associated_token;
pub mod metadata;
pub mod mint;

#[derive(Clone)]
pub struct Token<'a> {
    pub mint: Mint<'a>,
    pub metadata: Metadata<'a>,
}

#[derive(Clone)]
pub struct TokenState {
    pub mint_authority: COption<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: COption<Pubkey>,
    pub update_authority: Pubkey,
    pub title: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    // TODO: More data to come
}

impl<'a> Token<'a> {
    pub fn new(
        mint_account: Box<AccountInfo<'a>>,
        metadata_account: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        Self {
            mint: Mint::new(mint_account, token_program, load_data),
            metadata: Metadata::new(metadata_account, token_metadata_program, load_data),
        }
    }

    pub fn data(&self) -> TokenState {
        let mint_state = self.mint.data();
        let metadata_state = self.metadata.data();
        TokenState {
            mint_authority: mint_state.mint_authority,
            supply: mint_state.supply,
            decimals: mint_state.decimals,
            is_initialized: mint_state.is_initialized,
            freeze_authority: mint_state.freeze_authority,
            update_authority: metadata_state.update_authority,
            title: metadata_state.data.name,
            symbol: metadata_state.data.symbol,
            uri: metadata_state.data.uri,
            seller_fee_basis_points: metadata_state.data.seller_fee_basis_points,
            creators: metadata_state.data.creators,
        }
    }
}

impl<'a> IntoAccountInfo<'a> for Token<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.mint.account_info
    }
}

impl NautilusAccountInfo for Token<'_> {
    fn key(&self) -> &Pubkey {
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

    fn owner(&self) -> &Pubkey {
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
            self_account: Box::new(value.self_account.mint),
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
            self_account: Box::new(value.self_account.metadata),
            fee_payer: value.fee_payer,
            rent: value.rent,
            system_program: value.system_program,
        }
    }
}

impl NautilusCreateToken for Create<'_, Token<'_>> {
    fn create(
        &mut self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
        freeze_authority: Option<impl NautilusAccountInfo>,
    ) -> ProgramResult {
        // let mut create_mint: Create<Mint> = self.clone().into();
        // create_mint.create(decimals, mint_authority.clone(), freeze_authority)?;
        // let mut create_metadata: Create<Metadata> = self.clone().into();
        // create_metadata.create(
        //     title,
        //     symbol,
        //     uri,
        //     self.self_account.mint.to_owned(),
        //     mint_authority,
        //     update_authority,
        // )?;
        Ok(())
    }

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
    ) -> ProgramResult {
        // let mut create_mint: Create<Mint> = self.clone().into();
        // let mut create_metadata: Create<Metadata> = self.clone().into();
        // create_mint.create_with_payer(
        //     decimals,
        //     mint_authority.clone(),
        //     freeze_authority,
        //     payer.clone(),
        // )?;
        // create_metadata.create_with_payer(
        //     title,
        //     symbol,
        //     uri,
        //     self.self_account.mint.to_owned(),
        //     mint_authority,
        //     update_authority,
        //     payer,
        // )?;
        Ok(())
    }
}

impl<'a> IntoAccountInfo<'a> for Create<'a, Token<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a> NautilusMut for Mut<Token<'a>> {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError> {
        // self.self_account.account_info.try_borrow_mut_lamports()
        todo!()
    }
}

impl<'a> IntoAccountInfo<'a> for Mut<Token<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl NautilusSigner for Signer<Token<'_>> {}

impl<'a> IntoAccountInfo<'a> for Signer<Token<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}
