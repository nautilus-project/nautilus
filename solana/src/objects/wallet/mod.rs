use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::{create::create_account, transfer::transfer_lamports},
    Create, Mut, NautilusAccountInfo, NautilusCreate, NautilusMut, NautilusSigner,
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

impl NautilusAccountInfo for Wallet<'_> {
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

impl NautilusTransferLamports for Signer<Wallet<'_>> {
    fn transfer_lamports(self, to: impl NautilusMut, amount: u64) -> ProgramResult {
        transfer_lamports(
            self.self_account.key(),
            to.key(),
            amount,
            &[
                // self.self_account.into(),
                // *to.into(),
                *self.self_account.system_program.clone(),
            ],
        )
    }
}

impl NautilusCreate for Create<'_, Wallet<'_>> {
    fn create(&mut self) -> ProgramResult {
        create_account(
            self.fee_payer.key,
            self.key(),
            self.required_rent()?,
            self.size(),
            self.system_program.key,
            &[
                *self.fee_payer.clone(),
                // *self.self_account.account_info.clone(),
                *self.system_program.clone(),
            ],
        )
    }

    fn create_with_payer(&mut self, payer: impl NautilusSigner) -> ProgramResult {
        create_account(
            payer.key(),
            self.self_account.key(),
            self.required_rent()?,
            self.size(),
            self.system_program.key,
            &[
                // payer.into(),
                // *self.into(),
                *self.system_program.clone(),
            ],
        )
    }
}

impl<'a> IntoAccountInfo<'a> for Create<'a, Wallet<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a> NautilusMut for Mut<Wallet<'a>> {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError> {
        // self.self_account.account_info.try_borrow_mut_lamports()
        todo!()
    }
}

impl<'a> IntoAccountInfo<'a> for Mut<Wallet<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl NautilusSigner for Signer<Wallet<'_>> {}

impl<'a> IntoAccountInfo<'a> for Signer<Wallet<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}
