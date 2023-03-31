use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    create_metadata, Create, Mint, NautilusAccountInfo, NautilusCreateMetadata, NautilusSigner,
    Signer, Wallet,
};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Metadata<'a> {
    pub account_info: AccountInfo<'a>,
    pub token_metadata_program: AccountInfo<'a>,
}

impl<'a> IntoAccountInfo<'a> for Metadata<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
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
        &self,
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
        )
    }

    fn create_with_payer(
        &self,
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
        )
    }
}
