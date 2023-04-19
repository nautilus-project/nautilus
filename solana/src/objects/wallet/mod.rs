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

// impl<'a> IntoAccountInfo<'a> for Wallet<'a> {
//     fn into_account_info(self) -> AccountInfo<'a> {
//         *self.account_info
//     }
// }

impl NautilusAccountInfo for Wallet<'_> {
    // fn key<'b>(&self) -> &'b Pubkey {
    //     self.account_info.key
    // }

    fn is_signer(&self) -> bool {
        self.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.account_info.lamports()
    }

    // fn mut_lamports<'a>(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
    //     self.account_info.try_borrow_mut_lamports()
    // }

    // fn owner<'a>(&self) -> &'a Pubkey {
    //     self.account_info.owner
    // }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl NautilusTransferLamports for Signer<Wallet<'_>> {
    fn transfer_lamports(self, to: impl NautilusMut, amount: u64) -> ProgramResult {
        todo!()
    }
}

impl NautilusCreate for Create<'_, Wallet<'_>> {
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

    fn create_with_payer(&mut self, payer: impl NautilusSigner) -> ProgramResult {
        create_account(
            self.clone(),
            &self.system_program.key,
            payer,
            self.system_program.to_owned(),
        )
    }
}

// ---

// impl IntoAccountInfo for Create<'_, Wallet<'_>> {
//     fn into_account_info(self) -> AccountInfo<'a> {
//         *self.self_account.account_info
//     }
// }

impl NautilusAccountInfo for Create<'_, Wallet<'_>> {
    // fn key<'a>(&self) -> &'a Pubkey {
    //     self.self_account.account_info.key
    // }

    fn is_signer(&self) -> bool {
        self.self_account.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.self_account.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.self_account.account_info.lamports()
    }

    // fn mut_lamports<'a>(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
    //     self.self_account.account_info.try_borrow_mut_lamports()
    // }

    // fn owner<'a>(&self) -> &'a Pubkey {
    //     self.self_account.account_info.owner
    // }

    fn span(&self) -> usize {
        self.self_account.account_info.data_len()
    }

    // fn key(&self) -> Pubkey {
    //     self.self_account.account_info.key.clone()
    // }
}
