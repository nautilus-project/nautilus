use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

pub trait NautilusData: BorshDeserialize + BorshSerialize + Clone + Default {
    const TABLE_NAME: &'static str;
    const AUTO_INCREMENT: bool;
    fn primary_key<'a>(&self) -> &'a [u8];
    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key()]
    }
    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}

pub trait NautilusRecord<'a>: NautilusAccountInfo<'a> {
    fn primary_key(&self) -> &'a [u8];
    fn seeds(&self) -> [&'a [u8]; 2];
    fn pda(&self) -> (Pubkey, u8);
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}

// #[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
// pub struct NautilusDataDefault {}

// impl NautilusData for NautilusDataDefault {
//     const TABLE_NAME: &'static str = "__default_data__";

//     const AUTO_INCREMENT: bool = false;

//     fn primary_key<'a>(&self) -> &'a [u8] {
//         &[0]
//     }

//     fn check_authorities(&self, _accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
//         Ok(())
//     }

//     fn count_authorities(&self) -> u8 {
//         0
//     }
//     fn default() -> Self {
//         Self {}
//     }
// }
