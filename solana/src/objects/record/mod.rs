use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::create::create_pda, Create, NautilusAccountInfo, NautilusCreateRecord, NautilusData,
    NautilusIndex, NautilusMut, NautilusRecord, NautilusSigner, NautilusTransferLamports,
};

use super::DATA_NOT_SET_MSG;

pub mod index;

#[derive(Clone)]
pub struct Record<'a, T: NautilusData + 'a> {
    pub program_id: &'a Pubkey,
    pub index: NautilusIndex<'a>,
    pub account_info: Box<AccountInfo<'a>>,
    pub data: Option<Box<T>>,
}

impl<'a, T> Record<'a, T>
where
    T: NautilusData,
{
    pub fn new(
        program_id: &'a Pubkey,
        index_account_info: Box<AccountInfo<'a>>,
        account_info: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        let index = NautilusIndex::new(program_id, index_account_info, true);
        let mut obj = Self {
            program_id,
            index,
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
            Ok(state) => self.data = Some(Box::new(state)),
            Err(_) => {
                msg!("Error parsing Record state from {}", &self.account_info.key);
                msg!(
                    "Are you sure this is the correct data type for table `{}`?",
                    T::TABLE_NAME
                );
                self.data = None
            }
        }
    }

    pub fn data(&self) -> Box<T> {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.account_info.try_borrow_mut_lamports()
    }
}

impl<'a, T> IntoAccountInfo<'a> for Record<'a, T>
where
    T: NautilusData,
{
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl<T> NautilusAccountInfo for Record<'_, T>
where
    T: NautilusData,
{
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

impl<T> NautilusRecord for Record<'_, T>
where
    T: NautilusData,
{
    fn primary_key(&self) -> &[u8] {
        match &self.data {
            Some(data) => data.primary_key(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }

    fn seeds(&self) -> [&[u8]; 2] {
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

impl<T> NautilusTransferLamports for Record<'_, T>
where
    T: NautilusData,
{
    fn transfer_lamports(
        self,
        to: impl NautilusMut,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        // **self.mut_lamports()? -= amount;
        // **to.mut_lamports()? += amount;
        // Ok(())
        todo!()
    }
}

impl<T> NautilusCreateRecord<T> for Create<'_, Record<'_, T>>
where
    T: NautilusData,
{
    fn create_record(&mut self, data: T) -> ProgramResult {
        let (_, bump) = self.self_account.pda();
        let seeds = self.self_account.seeds();
        create_pda(
            self.fee_payer.key,
            self.self_account.key(),
            self.self_account.required_rent()?,
            self.self_account.size(),
            self.self_account.program_id,
            &[
                *self.fee_payer.clone(),
                // *self.into(),
                *self.system_program.clone(),
            ],
            &seeds,
            bump,
        )?;
        data.serialize(&mut &mut self.self_account.account_info.data.borrow_mut()[..])?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_record_with_payer(&mut self, data: T, payer: impl NautilusSigner) -> ProgramResult {
        let (_, bump) = self.self_account.pda();
        let seeds = self.self_account.seeds();
        create_pda(
            payer.key(),
            self.self_account.key(),
            self.self_account.required_rent()?,
            self.self_account.size(),
            self.self_account.program_id,
            &[
                // *payer.into(),
                // *self.into(),
                *self.system_program.clone(),
            ],
            &seeds,
            bump,
        )?;
        data.serialize(&mut &mut self.self_account.account_info.data.borrow_mut()[..])?;
        self.self_account.load_data();
        Ok(())
    }
}
