use mpl_token_metadata::state::Creator;
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_token, Create, Metadata, Mint, NautilusAccountInfo, NautilusCreateToken,
    NautilusSigner, Signer, Wallet,
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
        mint_account: AccountInfo<'a>,
        metadata_account: AccountInfo<'a>,
        token_program: AccountInfo<'a>,
        token_metadata_program: AccountInfo<'a>,
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
        self.mint.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for Token<'a> {
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
        &self,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        let create_mint: Create<Mint> = self.clone().into();
        let create_metadata: Create<Metadata> = self.clone().into();
        create_token(
            create_mint,
            create_metadata,
            decimals,
            title,
            symbol,
            uri,
            mint_authority,
            freeze_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.mint.token_program.to_owned(),
            self.self_account.metadata.token_metadata_program.to_owned(),
        )
    }

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
    ) -> ProgramResult {
        let create_mint: Create<Mint> = self.clone().into();
        let create_metadata: Create<Metadata> = self.clone().into();
        create_token(
            create_mint,
            create_metadata,
            decimals,
            title,
            symbol,
            uri,
            mint_authority,
            freeze_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.mint.token_program.to_owned(),
            self.self_account.metadata.token_metadata_program.to_owned(),
        )
    }
}
