use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_pda, Create, NautilusAccountInfo, NautilusCreate, NautilusData,
    NautilusSigner, NautilusTable, NautilusTransferLamports, Signer, Wallet,
};

use super::DATA_NOT_SET_MSG;

pub mod index;

#[derive(Clone)]
pub struct Table<'a, T: NautilusData + 'a> {
    pub program_id: &'a Pubkey,
    pub account_info: Box<AccountInfo<'a>>,
    pub data: Option<T>,
}

impl<'a, T: NautilusData> Table<'a, T> {
    pub fn new(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        let mut obj = Self {
            program_id,
            account_info,
            data: None,
        };
        if load_data {
            obj.load_data();
        };
        obj
    }

    fn load_data(&mut self) {
        match T::try_from_slice(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(state),
            Err(_) => {
                msg!("Error parsing Mint state from {}", &self.account_info.key);
                msg!("Are you sure this is the correct data type?"); // TODO: Get type name in here
                self.data = None
            }
        }
    }

    pub fn data(&self) -> T {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a, T: NautilusData> IntoAccountInfo<'a> for Table<'a, T> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
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
        match &self.data {
            Some(data) => data.primary_key(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn seeds(&self) -> [&'a [u8]; 2] {
        match &self.data {
            Some(data) => data.seeds(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn pda(&self) -> (Pubkey, u8) {
        match &self.data {
            Some(data) => data.pda(self.program_id),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        match &self.data {
            Some(data) => data.check_authorities(accounts),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn count_authorities(&self) -> u8 {
        match &self.data {
            Some(data) => data.count_authorities(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
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
    fn create(&mut self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
        )?;
        self.self_account.load_data();
        Ok(())
    }
}
