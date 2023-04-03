use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_pda, objects::table::DATA_NOT_SET_MSG, Create, NautilusAccountInfo,
    NautilusCreate, NautilusData, NautilusSigner, NautilusTable, NautilusTransferLamports, Signer,
    Wallet,
};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct NautilusIndexData {
    pub index: std::collections::HashMap<String, u32>,
}

impl NautilusIndexData {
    pub const SEED_PREFIX: &'static str = "nautilus_index";

    pub fn get_count(&self, table_name: &String) -> Option<&u32> {
        self.index.get(table_name)
    }

    pub fn get_next_count(&self, table_name: &String) -> Option<u32> {
        match self.index.get(table_name) {
            Some(count) => Some(count + 1),
            None => None,
        }
    }

    pub fn add_record(&mut self, table_name: &String) -> Result<&u32, InsertRecordError> {
        match self.index.get_mut(table_name) {
            Some(count) => {
                *count += 1;
                Ok(count)
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
    fn primary_key<'a>(&self) -> &'a [u8] {
        &[0]
    }

    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::SEED_PREFIX.as_bytes(), &[0]]
    }

    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
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
    pub account_info: AccountInfo<'a>,
    pub data: Option<NautilusIndexData>,
}

impl<'a> NautilusIndex<'a> {
    pub fn new(program_id: &'a Pubkey, account_info: AccountInfo<'a>, load_data: bool) -> Self {
        let data = match load_data {
            true => {
                match NautilusIndexData::try_from_slice(match &account_info.try_borrow_mut_data() {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Could not read data from the Index account.");
                        println!("Is it empty?");
                        panic!("{}", e);
                    }
                }) {
                    Ok(state) => Some(state),
                    Err(_) => {
                        println!("Error parsing NautilusIndexData state from provided account.");
                        println!("Are you sure this is a NautilusIndexData?");
                        None
                    }
                }
            }
            false => None,
        };
        Self {
            program_id,
            account_info,
            data,
        }
    }

    pub fn data(&self) -> NautilusIndexData {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a> IntoAccountInfo<'a> for NautilusIndex<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
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

impl<'a> NautilusTable<'a> for NautilusIndex<'a> {
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
