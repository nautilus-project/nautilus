use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    create_token, Create, Metadata, Mint, NautilusAccountInfo, NautilusCreateToken, NautilusSigner,
    Signer, Wallet,
};

pub mod associated_token;
pub mod metadata;
pub mod mint;

#[derive(Clone)]
pub struct Token<'a> {
    pub account_info: AccountInfo<'a>,
    pub metadata: AccountInfo<'a>,
    pub token_program: AccountInfo<'a>,
    pub token_metadata_program: AccountInfo<'a>,
}

impl<'a> IntoAccountInfo<'a> for Token<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for Token<'a> {
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

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a> From<Token<'a>> for Mint<'a> {
    fn from(value: Token<'a>) -> Self {
        Self {
            account_info: value.account_info,
            token_program: value.token_program,
        }
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
        Self {
            account_info: value.metadata,
            token_metadata_program: value.token_metadata_program,
        }
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
            self.self_account.token_program.to_owned(),
            self.self_account.token_metadata_program.to_owned(),
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
            self.self_account.token_program.to_owned(),
            self.self_account.token_metadata_program.to_owned(),
        )
    }
}
