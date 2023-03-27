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

impl crate::NautilusData for NautilusIndexData {
    fn primary_key<'a>(&self) -> &'a [u8] {
        &[0]
    }

    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::SEED_PREFIX.as_bytes(), &[0]]
    }

    fn pda<'a>(
        &self,
        program_id: &'a solana_program::pubkey::Pubkey,
    ) -> (solana_program::pubkey::Pubkey, u8) {
        solana_program::pubkey::Pubkey::find_program_address(&self.seeds(), program_id)
    }

    fn check_authorities(
        &self,
        _accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError> {
        Ok(())
    }

    fn count_authorities(&self) -> u8 {
        0
    }
}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct NautilusIndex<'a> {
    pub program_id: &'a solana_program::pubkey::Pubkey,
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub data: NautilusIndexData,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for NautilusIndex<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::properties::NautilusAccountInfo<'a> for NautilusIndex<'a> {
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
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

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a> crate::NautilusPda<'a> for NautilusIndex<'a> {
    fn primary_key(&self) -> &'a [u8] {
        crate::NautilusData::primary_key(&self.data)
    }

    fn seeds(&self) -> [&'a [u8]; 2] {
        crate::NautilusData::seeds(&self.data)
    }

    fn pda(&self) -> (solana_program::pubkey::Pubkey, u8) {
        crate::NautilusData::pda(&self.data, self.program_id)
    }

    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError> {
        crate::NautilusData::check_authorities(&self.data, accounts)
    }

    fn count_authorities(&self) -> u8 {
        crate::NautilusData::count_authorities(&self.data)
    }
}

impl<'a> crate::properties::NautilusTransferLamports<'a> for NautilusIndex<'a> {
    fn transfer_lamports<U: crate::properties::NautilusAccountInfo<'a>>(
        self,
        to: U,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info;
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}

impl<'a> crate::properties::NautilusCreate<'a>
    for crate::properties::Create<'a, NautilusIndex<'a>>
{
    fn create(&self) -> solana_program::entrypoint::ProgramResult {
        use crate::{NautilusAccountInfo, NautilusData, NautilusPda};

        let payer = self.fee_payer.clone();
        let system_program = self.system_program.clone();
        let (_, bump) = self.self_account.pda();
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key,
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer,
                self.self_account.account_info.clone(),
                system_program,
            ],
            &[&self.self_account.data.seeds(), &[&[bump]]],
        )
    }

    fn create_with_payer<U: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        payer: U,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::{NautilusAccountInfo, NautilusData, NautilusPda};

        let system_program = self.system_program.clone();
        let (_, bump) = self.self_account.pda();
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key(),
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer.into(),
                self.self_account.account_info.clone(),
                system_program,
            ],
            &[&self.self_account.data.seeds(), &[&[bump]]],
        )
    }
}
