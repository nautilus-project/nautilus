use crate::{
    invoke, next_account_info, system_instruction, AccountInfo, ProgramError, ProgramResult, Pubkey,
};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The default accounts for implementing the `nautilus_update(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
pub struct NautilusUpdateAccounts<'a> {
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
}

/// The trait that enables the default `nautilus_update(..)` instruction for the PDA.
pub trait NautilusAccountUpdate: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID, list of accounts, and instruction accounts into the `NautilusUpdateAccounts`.
    fn parse_nautilus_update_accounts<'a>(
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<NautilusUpdateAccounts<'a>, ProgramError> {
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
        let system_program = next_account_info(accounts_iter)?.to_owned();

        Ok(nautilus::NautilusUpdateAccounts {
            target_account,
            authorities,
            fee_payer,
            system_program,
        })
    }

    /// The default `update` instruction for the PDA.
    ///
    /// Simply updates the PDA's inner data.
    ///
    /// If authorities are specified, will check to make sure they are signers on the instruction.
    ///
    /// Note: As of right now, this instruction **cannot change the primary key**.
    ///
    /// # Arguments
    ///
    /// * program_id - This program's ID.
    /// * accounts - The list of accounts provided.
    /// * update_data - The data to serialize into the existing account.
    ///
    /// # Returns
    ///
    /// `ProgramResult`
    fn nautilus_update<'a>(
        program_id: &'a Pubkey,
        passed_accounts: &'a [AccountInfo<'a>],
        update_data: Self,
    ) -> ProgramResult {
        let accounts = Self::parse_nautilus_update_accounts(passed_accounts)?;

        let (pda, _) = update_data.pda(program_id);
        assert!(accounts.target_account.key.eq(&pda));

        let data = Self::try_from_slice(&accounts.target_account.data.borrow_mut())?;
        data.check_authorities(accounts.authorities)?;
        assert!(data.primary_key().eq(update_data.primary_key()));

        let diff = data.lamports_required()? - accounts.target_account.lamports();

        invoke(
            &system_instruction::transfer(
                accounts.fee_payer.key,
                accounts.target_account.key,
                diff,
            ),
            &[
                accounts.fee_payer.clone(),
                accounts.target_account.clone(),
                accounts.system_program.clone(),
            ],
        )?;

        accounts
            .target_account
            .realloc(update_data.span()?, false)?;

        update_data.serialize(&mut &mut accounts.target_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
