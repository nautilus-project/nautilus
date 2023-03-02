use crate::{next_account_info, system_program, AccountInfo, ProgramError, ProgramResult};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The default accounts for implementing the `nautilus_delete(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
pub struct NautilusDeleteAccounts<'a> {
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
}

/// The trait that enables the default `nautilus_delete(..)` instruction for the PDA.
pub trait NautilusAccountDelete: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID and list of accounts into the `NautilusDeleteaccounts`.
    ///
    /// Note: This instruction takes no arguments, only program ID and accounts.
    fn parse_nautilus_delete_accounts<'a>(
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<NautilusDeleteAccounts<'a>, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?.to_owned();
        let authorities: Vec<AccountInfo> = (0..Self::count_authorities())
            .map(|_| {
                next_account_info(accounts_iter)
                    .expect("One or more authorities missing.")
                    .to_owned()
            })
            .collect();
        let fee_payer = next_account_info(accounts_iter)?.to_owned();

        Ok(nautilus::NautilusDeleteAccounts {
            target_account,
            authorities,
            fee_payer,
        })
    }

    /// The default `delete` instruction for the PDA.
    ///
    /// This instruction "deletes" an account by reallocating it's data to 0, assigning it to the System Program, and returning the rent.
    ///
    /// Note: This will not free it's assigned primary key value and autoincrement will not be affected.
    ///
    /// # Arguments
    ///
    /// * program_id - This program's ID.
    /// * accounts - The list of accounts provided.
    ///
    /// # Returns
    ///
    /// `ProgramResult`
    fn nautilus_delete<'a>(passed_accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
        let accounts = Self::parse_nautilus_delete_accounts(passed_accounts)?;

        let delete_data = Self::try_from_slice(&accounts.target_account.data.borrow())?;
        delete_data.check_authorities(accounts.authorities)?;

        let dest_starting_lamports = accounts.fee_payer.lamports();
        **accounts.fee_payer.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(accounts.target_account.lamports())
            .unwrap();
        **accounts.target_account.lamports.borrow_mut() = 0;
        accounts.target_account.assign(&system_program::ID);
        accounts
            .target_account
            .realloc(0, false)
            .map_err(Into::into)
    }
}
