//! Traits used for managing the account data of Nautilus objects.
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

/// The trait that represents account data for non-record Nautilus accounts.
///
/// This type of account is a standard program-derived address account (PDA).
///
/// Note: `seeds(..)` and `pda(..)` are derived from the derive macro - since their arguments will vary.
pub trait NautilusAccountData: BorshDeserialize + BorshSerialize + Clone + Default {
    const DISCRIMINATOR_STR: &'static str;

    /// The 8-bit discriminator applied to this account as a prefix on any account data containing
    /// this data type.
    fn discriminator(&self) -> [u8; 8] {
        discriminator(Self::DISCRIMINATOR_STR)
    }

    /// Checks authorities against the data's declared authorities.
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Counts the data's declared authorities.
    fn count_authorities(&self) -> u8;
}

/// The trait that represents account data for record-based, SQL-friendly Nautilus accounts.
///
/// This type of account is a program-derived address account (PDA) but can be treated like a record
/// in a database table, with the data scheme (struct) being the schema of the table.
pub trait NautilusRecordData: BorshDeserialize + BorshSerialize + Clone + Default {
    const TABLE_NAME: &'static str;
    const AUTO_INCREMENT: bool;

    /// The 8-bit discriminator applied to this account as a prefix on any account data containing
    /// this data type.
    fn discriminator(&self) -> [u8; 8] {
        discriminator(Self::TABLE_NAME)
    }

    /// The primary key of this particular record's account data type. This will return the value of
    /// whichever field has been declared as the primary key for this table.
    fn primary_key(&self) -> Vec<u8>;

    /// The seeds used to derive the program-derived address of this account.
    ///
    /// Accessible through the account's data type since some parameters for seeds may be based on
    /// fields in the data.
    fn seeds(&self) -> Vec<Vec<u8>> {
        vec![Self::TABLE_NAME.as_bytes().to_vec(), self.primary_key()]
    }

    /// Returns the program-derived address and bump for an account containing this data.
    ///
    /// Accessible through the account's data type since some parameters for seeds may be based on
    /// fields in the data.
    fn pda(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        let seeds_vec = self.seeds();
        let seeds: Vec<&[u8]> = seeds_vec.iter().map(AsRef::as_ref).collect();
        Pubkey::find_program_address(&seeds, program_id)
    }

    /// Checks authorities against the data's declared authorities.
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Counts the data's declared authorities.
    fn count_authorities(&self) -> u8;
}

/// This trait provides methods accessible to Nautilus Accounts (PDAs).
///
/// When you define a struct with the `#[derive(nautilus::State)]` macro, any accounts created
/// with this data scheme will have seeds defined and inherit PDA functionality.
///
/// Note: `seeds(..)` and `pda(..)` are derived from the derive macro - since their arguments will vary.
pub trait NautilusAccount<'a>: NautilusAccountInfo<'a> {
    /// Returns the data's discriminator.
    fn discriminator(&self) -> [u8; 8];

    /// Checks authorities against the data's declared authorities.
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Counts the data's declared authorities.
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
    /// Returns the data's discriminator.
    fn discriminator(&self) -> [u8; 8];

    /// The seeds used to derive the program-derived address of this account.
    fn seeds(&self) -> Vec<Vec<u8>>;

    /// Returns the program-derived address and bump for an account.
    fn pda(&self) -> (Pubkey, u8);

    /// Returns the primary key of a record.
    fn primary_key(&self) -> Vec<u8>;

    /// Checks authorities against the data's declared authorities.
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Counts the data's declared authorities.
    fn count_authorities(&self) -> u8;
}

/// Helper function to return the 8-bit discriminator of an account data type.
fn discriminator(discrim_str: &str) -> [u8; 8] {
    let mut discriminator = [0u8; 8];
    let preimage = format!("{}:{}", "global", discrim_str);
    discriminator.copy_from_slice(&solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]); // First 8 bytes
    discriminator
}
