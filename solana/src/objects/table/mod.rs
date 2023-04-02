use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    create_pda, Create, NautilusAccountInfo, NautilusCreate, NautilusData, NautilusSigner,
    NautilusTable, NautilusTransferLamports, Signer, Wallet,
};

pub mod index;

#[derive(Clone)]
pub struct Table<'a, T: NautilusData + 'a> {
    pub program_id: &'a Pubkey,
    pub account_info: AccountInfo<'a>,
    pub data: T,
}

impl<'a, T: NautilusData> IntoAccountInfo<'a> for Table<'a, T> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
    }
}

impl<'a, T: NautilusData> NautilusAccountInfo<'a> for Table<'a, T> {
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

impl<'a, T: NautilusData> NautilusTable<'a> for Table<'a, T> {
    fn primary_key(&self) -> &'a [u8] {
        self.data.primary_key()
    }

    fn seeds(&self) -> [&'a [u8]; 2] {
        self.data.seeds()
    }

    fn pda(&self) -> (Pubkey, u8) {
        self.data.pda(self.program_id)
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        self.data.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.data.count_authorities()
    }
}

impl<'a, T: NautilusData + 'a> NautilusTransferLamports<'a> for Table<'a, T> {
    fn transfer_lamports(
        self,
        to: impl NautilusAccountInfo<'a>,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info;
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}

impl<'a, T: NautilusData> NautilusCreate<'a> for Create<'a, Table<'a, T>> {
    fn create(&self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
        )
    }

    fn create_with_payer(&self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
        )
    }
}
