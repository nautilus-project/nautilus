use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    create_associated_token_account, Create, Mint, NautilusAccountInfo,
    NautilusCreateAssociatedTokenAccount, NautilusSigner, Signer, Wallet,
};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct AssociatedTokenAccount<'a> {
    pub account_info: AccountInfo<'a>,
    pub token_program: AccountInfo<'a>,
    pub associated_token_program: AccountInfo<'a>,
}

impl<'a> IntoAccountInfo<'a> for AssociatedTokenAccount<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for AssociatedTokenAccount<'a> {
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

impl<'a> NautilusCreateAssociatedTokenAccount<'a> for Create<'a, AssociatedTokenAccount<'a>> {
    fn create(&self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_associated_token_account(
            self.self_account.clone(),
            mint,
            owner,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )
    }

    fn create_with_payer(
        &self,
        mint: Mint<'a>,
        owner: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        create_associated_token_account(
            self.self_account.clone(),
            mint,
            owner,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )
    }
}
