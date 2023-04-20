use mpl_token_metadata::state::{Metadata as MetadataState, TokenMetadataAccount};
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_metadata, objects::DATA_NOT_SET_MSG, Create, Mint, Mut,
    NautilusAccountInfo, NautilusCreateMetadata, NautilusMut, NautilusSigner, Signer,
};

#[derive(Clone)]
pub struct Metadata<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_metadata_program: Box<AccountInfo<'a>>,
    pub data: Option<Box<MetadataState>>,
}

impl<'a> Metadata<'a> {
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        let mut obj = Self {
            account_info,
            token_metadata_program,
            data: None,
        };
        if load_data {
            obj.load_data();
        };
        obj
    }

    fn load_data(&mut self) {
        match MetadataState::safe_deserialize(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(Box::new(state)),
            Err(_) => {
                msg!(
                    "Error parsing Metadata state from {}",
                    &self.account_info.key
                );
                msg!("Are you sure this is a Metadata?");
                self.data = None
            }
        }
    }

    pub fn data(&self) -> Box<MetadataState> {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a> IntoAccountInfo<'a> for Metadata<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl NautilusAccountInfo for Metadata<'_> {
    fn key(&self) -> &Pubkey {
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

    fn owner(&self) -> &Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl NautilusCreateMetadata for Create<'_, Metadata<'_>> {
    fn create(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: impl NautilusAccountInfo,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
    ) -> ProgramResult {
        create_metadata(
            self.self_account.token_metadata_program.key.clone(),
            self.self_account.key().clone(),
            mint.key().clone(),
            mint_authority.key().clone(),
            self.fee_payer.key.clone(),
            update_authority.key().clone(),
            title,
            symbol,
            uri,
            None, // TODO: Make below params available
            0,
            true,
            false,
            None,
            None,
            None,
            &[
                // *self.self_account.clone().into(),
                // mint.into(),
                // mint_authority.into(),
                *self.fee_payer.clone(),
                // *self.self_account.token_metadata_program.clone(),
                *self.rent.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        title: String,
        symbol: String,
        uri: String,
        mint: impl NautilusAccountInfo,
        mint_authority: impl NautilusSigner,
        update_authority: impl NautilusAccountInfo,
        payer: impl NautilusSigner,
    ) -> ProgramResult {
        create_metadata(
            self.self_account.token_metadata_program.key.clone(),
            self.self_account.key().clone(),
            mint.key().clone(),
            mint_authority.key().clone(),
            payer.key().clone(),
            update_authority.key().clone(),
            title,
            symbol,
            uri,
            None, // TODO: Make below params available
            0,
            true,
            false,
            None,
            None,
            None,
            &[
                // *self.self_account.clone().into(),
                // mint.into(),
                // mint_authority.into(),
                // payer.into(),
                // *self.self_account.token_metadata_program.clone(),
                *self.rent.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }
}

impl<'a> IntoAccountInfo<'a> for Create<'a, Metadata<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a> NautilusMut for Mut<Metadata<'a>> {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError> {
        // self.self_account.account_info.try_borrow_mut_lamports()
        todo!()
    }
}

impl<'a> IntoAccountInfo<'a> for Mut<Metadata<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl NautilusSigner for Signer<Metadata<'_>> {}

impl<'a> IntoAccountInfo<'a> for Signer<Metadata<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}
