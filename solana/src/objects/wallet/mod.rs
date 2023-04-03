use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::{create::create_account, transfer::transfer_lamports},
    Create, NautilusAccountInfo, NautilusCreate, NautilusMut, NautilusSigner,
    NautilusTransferLamports, Signer,
};

#[derive(Clone)]
pub struct Wallet<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub system_program: Box<AccountInfo<'a>>,
}

impl<'a> Wallet<'a> {
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        system_program: Box<AccountInfo<'a>>,
        _load_data: bool,
    ) -> Self {
        Self {
            account_info,
            system_program,
        }
    }
}

impl<'a> IntoAccountInfo<'a> for Wallet<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for Wallet<'a> {
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

impl<'a> NautilusTransferLamports<'a> for Signer<'a, Wallet<'a>> {
    fn transfer_lamports(self, to: impl NautilusMut<'a>, amount: u64) -> ProgramResult {
        let system_program = self.self_account.system_program.clone();
        transfer_lamports(self, to, amount, system_program)
    }
}

impl<'a> NautilusCreate<'a> for Create<'a, Wallet<'a>> {
    fn create(&mut self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_account(
            self.clone(),
            &self.system_program.key,
            payer,
            self.system_program.to_owned(),
        )
    }

    fn create_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        create_account(
            self.clone(),
            &self.system_program.key,
            payer,
            self.system_program.to_owned(),
        )
    }
}
