use crate::{AccountInfo, ProgramError};

use super::data::NautilusAccountData;

/// The trait that will enable authority checks.
pub trait NautilusAccountAuth: NautilusAccountData {
    /// Checks the specified authorities for the PDA.
    ///
    /// Using the #[authority] attribute on a PDA account's fields will add a check to this method.
    ///
    /// # Arguments
    ///
    /// * accounts - a vector containing what should be the authority accounts to check.
    ///
    /// # Returns
    ///
    /// `Result<(), ProgramError>`
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    /// Gathers the authorities from the `accounts_iter` iterator inside of any of the default `nautilus_<crud>(..)` operations.
    ///
    /// When the operation is ready to gather authorities for the struct, based on the fields annotated with the #[authority] attribute, it calls this method to gather them from the iterator.
    fn count_authorities<'a>() -> u8;
}
