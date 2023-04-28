use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

/// * This documentation will be updated when this trait is updated *
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

/// This trait provides methods only accessible to Nautilus Tables.
///
/// When you define a struct with the `#[derive(nautilus::Table)]` macro, any accounts created
/// with this data scheme can be considered records, and the struct itself can be considered a schema
/// for a "table".
///
/// This trait is where things like autoincrement and authority checks come into play.
pub trait NautilusRecord<'a>: NautilusAccountInfo<'a> {
    /// Returns the primary key of a record.
    fn primary_key(&self) -> Vec<u8>;

    /// Returns the seeds of a record.
    ///
    /// These seeds can be converted to a slice in order to sign Cross-Program-Invocations
    fn seeds(&self) -> [Vec<u8>; 2];

    /// Returns the Program-Derived-Address and Bump for a record.
    fn pda(&self) -> (Pubkey, u8);

    /// Checks against the provided list of accounts to ensure all authorities required for modifying data
    /// in this record have been provided to the program as signers.
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Returns the total count of authorities required to modify data in this record.
    fn count_authorities(&self) -> u8;
}
