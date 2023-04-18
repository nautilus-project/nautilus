use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_pda, objects::record::DATA_NOT_SET_MSG, Create, NautilusAccountInfo,
    NautilusCreate, NautilusData, NautilusRecord, NautilusSigner, NautilusTransferLamports, Signer,
    Wallet,
};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct NautilusIndexData {
    pub index: std::collections::HashMap<String, u32>,
}

impl NautilusIndexData {
    pub fn get_count(&self, table_name: &str) -> Option<u32> {
        match self.index.get(&(table_name.to_string())) {
            Some(u) => Some(*u),
            None => None,
        }
    }

    pub fn get_next_count(&self, table_name: &str) -> u32 {
        match self.index.get(&(table_name.to_string())) {
            Some(count) => count + 1,
            None => 1,
        }
    }

    pub fn add_record(&mut self, table_name: &str) -> Result<u32, InsertRecordError> {
        match self.index.get_mut(&(table_name.to_string())) {
            Some(count) => {
                *count += 1;
                Ok(*count)
            }
            None => Err(InsertRecordError()),
        }
    }
}

#[derive(Debug)]
pub struct InsertRecordError();

impl std::fmt::Display for InsertRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to write new record.")
    }
}

impl NautilusData for NautilusIndexData {
    const TABLE_NAME: &'static str = "nautilus_index";

    const AUTO_INCREMENT: bool = false;

    fn primary_key<'a>(&self) -> &'a [u8] {
        &[0]
    }

    fn check_authorities(&self, _accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        Ok(())
    }

    fn count_authorities(&self) -> u8 {
        0
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct NautilusIndex<'a> {
    pub program_id: &'a Pubkey,
    pub account_info: Box<AccountInfo<'a>>,
    pub data: Option<NautilusIndexData>,
}

impl<'a> NautilusIndex<'a> {
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
        match NautilusIndexData::try_from_slice(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(state),
            Err(_) => {
                msg!("Error parsing Index state from {}", &self.account_info.key);
                msg!("Are you sure this is the Index?");
                self.data = None
            }
        }
    }

    pub fn data(&self) -> NautilusIndexData {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    pub fn get_count(&self, table_name: &str) -> Option<u32> {
        self.data().get_count(table_name)
    }

    pub fn get_next_count(&self, table_name: &str) -> u32 {
        self.data().get_next_count(table_name)
    }

    pub fn add_record(&mut self, table_name: &str) -> Result<u32, ProgramError> {
        let count = match self.data().add_record(table_name) {
            Ok(count) => count,
            Err(e) => return Err(ProgramError::BorshIoError(e.to_string())), // TODO wtf?
        };
        self.data()
            .serialize(&mut &mut self.account_info.data.borrow_mut()[..])?;
        Ok(count)
    }
}

impl<'a> IntoAccountInfo<'a> for NautilusIndex<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for NautilusIndex<'a> {
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

impl<'a> NautilusRecord<'a> for NautilusIndex<'a> {
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

impl<'a> NautilusTransferLamports<'a> for NautilusIndex<'a> {
    fn transfer_lamports(self, to: impl NautilusAccountInfo<'a>, amount: u64) -> ProgramResult {
        let from = self.account_info;
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}

impl<'a> NautilusCreate<'a> for Create<'a, NautilusIndex<'a>> {
    fn create(&mut self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.clone(),
            system_program: self.system_program.clone(),
        });
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.clone(),
            NautilusIndexData {
                index: std::collections::HashMap::new(),
            },
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        create_pda(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.clone(),
            NautilusIndexData {
                index: std::collections::HashMap::new(),
            },
        )?;
        self.self_account.load_data();
        Ok(())
    }
}
