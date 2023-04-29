pub use mpl_token_metadata::state::{Metadata as MetadataState, TokenMetadataAccount};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::cpi;
use crate::{
    error::NautilusError, Create, Mint, NautilusAccountInfo, NautilusCreateMetadata,
    NautilusSigner, Signer, Wallet,
};

/// The Nautilus object representing a token metadata account.
///
/// The underlying account - designated in field `account_info` - is the token metadata account.
///
/// We also include the read-only Token Metadata Program for any CPI operations necessary, since we do not
/// own this account.
#[derive(Clone)]
pub struct Metadata<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_metadata_program: Box<AccountInfo<'a>>,
    pub data: MetadataState,
}

impl<'a> Metadata<'a> {
    /// Instantiate a new `Metadata` without loading the account inner data from on-chain.
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
    ) -> Self {
        Self {
            account_info,
            token_metadata_program,
            data: MetadataState::default(),
        }
    }

    /// Instantiate a new `Metadata` and load the account inner data from on-chain.
    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data = match MetadataState::safe_deserialize(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    String::from("token_metadata"),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => state_data,
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    String::from("token_metadata"),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            account_info,
            token_metadata_program,
            data,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for Metadata<'a> {
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
        Ok(MetadataState::size())
    }
}

impl<'a> NautilusCreateMetadata<'a> for Create<'a, Metadata<'a>> {
    fn create(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
    ) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        cpi::token_metadata::create_metadata_v3(
            self.self_account.token_metadata_program.key,
            self.clone(),
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
        )?;
        self.self_account = Metadata::load(
            self.self_account.account_info.clone(),
            self.self_account.token_metadata_program.clone(),
        )?;
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: super::mint::Mint<'a>,
        mint_authority: impl NautilusSigner<'a>,
        update_authority: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        cpi::token_metadata::create_metadata_v3(
            self.self_account.token_metadata_program.key,
            self.clone(),
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
        )?;
        self.self_account = Metadata::load(
            self.self_account.account_info.clone(),
            self.self_account.token_metadata_program.clone(),
        )?;
        Ok(())
    }
}
