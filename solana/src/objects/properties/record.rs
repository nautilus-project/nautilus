use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

pub trait NautilusData: BorshDeserialize + BorshSerialize + Clone + Default {
    const TABLE_NAME: &'static str;
    const AUTO_INCREMENT: bool;
    fn primary_key(&self) -> Vec<u8>;
    fn seeds(&self) -> [Vec<u8>; 2] {
        [Self::TABLE_NAME.as_bytes().to_vec(), self.primary_key()]
    }
    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        let seeds_vec = &self.seeds();
        let seeds: [&[u8]; 2] = [seeds_vec[0].as_slice(), seeds_vec[1].as_slice()];
        Pubkey::find_program_address(&seeds, program_id)
    }
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}

pub trait NautilusRecord<'a>: NautilusAccountInfo<'a> {
    fn primary_key(&self) -> Vec<u8>;
    fn seeds(&self) -> [Vec<u8>; 2];
    fn pda(&self) -> (Pubkey, u8);
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}
