use crate::{next_account_info, system_program, AccountInfo, ProgramError, ProgramResult, Pubkey};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The default args for implementing the `nautilus_delete(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
pub struct NautilusDeleteArgs<'a> {
    pub program_id: &'a Pubkey,
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
}

/// The trait that enables the default `nautilus_delete(..)` instruction for the PDA.
pub trait NautilusAccountDelete: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID and list of accounts into the `NautilusDeleteArgs`.
    ///
    /// Note: This instruction takes no arguments, only program ID and accounts.
    fn parse_nautilus_delete_args<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<NautilusDeleteArgs<'a>, ProgramError> {
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

        Ok(nautilus::NautilusDeleteArgs {
            program_id,
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
    fn nautilus_delete<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
    ) -> ProgramResult {
        let args = Self::parse_nautilus_delete_args(program_id, accounts)?;

        let delete_data = Self::try_from_slice(&args.target_account.data.borrow())?;
        delete_data.check_authorities(args.authorities)?;

        let dest_starting_lamports = args.fee_payer.lamports();
        **args.fee_payer.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(args.target_account.lamports())
            .unwrap();
        **args.target_account.lamports.borrow_mut() = 0;
        args.target_account.assign(&system_program::ID);
        args.target_account.realloc(0, false).map_err(Into::into)
    }
}
