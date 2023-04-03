use borsh::BorshDeserialize;
use mpl_token_metadata::state::Metadata as MetadataState;
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_metadata, objects::DATA_NOT_SET_MSG, Create, Mint, NautilusAccountInfo,
    NautilusCreateMetadata, NautilusSigner, Signer, Wallet,
};

#[derive(Clone)]
pub struct Metadata<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_metadata_program: Box<AccountInfo<'a>>,
    pub data: Option<MetadataState>,
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
        match MetadataState::try_from_slice(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(state),
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

    pub fn data(&self) -> MetadataState {
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

impl<'a> NautilusAccountInfo<'a> for Metadata<'a> {
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
        create_metadata(
            self.clone(),
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
            self.self_account.token_metadata_program.to_owned(),
        )?;
        self.self_account.load_data();
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
        create_metadata(
            self.clone(),
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
            self.rent.to_owned(),
            self.self_account.token_metadata_program.to_owned(),
        )?;
        self.self_account.load_data();
        Ok(())
    }
}
