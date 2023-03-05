use crate::{
    invoke_signed, next_account_info, system_instruction, AccountInfo, BorshDeserialize,
    NautilusAutoincrementAccount, ProgramError, ProgramResult, Pubkey,
};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The default accounts for implementing the `nautilus_create(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
pub struct NautilusCreateAccounts<'a> {
    pub autoinc_account: Option<AccountInfo<'a>>,
    pub new_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
}

/// The trait that enables the default `nautilus_create(..)` instruction for the PDA.
pub trait NautilusAccountCreate: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID, list of accounts, and instruction accounts into the `NautilusCreateaccounts`.
    fn parse_nautilus_create_accounts<'a>(
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<NautilusCreateAccounts<'a>, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let autoinc_account = match Self::AUTO_INCREMENT {
            true => Some(next_account_info(accounts_iter)?.to_owned()),
            false => None,
        };
        let new_account = next_account_info(accounts_iter)?.to_owned();
        let authorities: Vec<AccountInfo> = (0..Self::count_authorities())
            .map(|_| {
                next_account_info(accounts_iter)
                    .expect("One or more authorities missing.")
                    .to_owned()
            })
            .collect();
        let fee_payer = next_account_info(accounts_iter)?.to_owned();
        let system_program = next_account_info(accounts_iter)?.to_owned();

        Ok(nautilus::NautilusCreateAccounts {
            autoinc_account,
            new_account,
            authorities,
            fee_payer,
            system_program,
        })
    }

    /// The default `create` instruction for the PDA.
    ///
    /// Simply creates the PDA and writes the data to the account.
    ///
    /// If autoincrement is enabled, will also increment the autoincrement account.
    ///
    /// If authorities are specified, will check to make sure they are signers on the instruction.
    ///
    /// # Arguments
    ///
    /// * program_id - This program's ID.
    /// * accounts - The list of accounts provided.
    /// * create_data - The data to serialize into the new account.
    ///
    /// # Returns
    ///
    /// `ProgramResult`
    fn nautilus_create<'a>(
        program_id: &'a Pubkey,
        provided_accounts: &'a [AccountInfo<'a>],
        create_data: Self,
    ) -> ProgramResult {
        let accounts = Self::parse_nautilus_create_accounts(provided_accounts)?;
        let (pda, bump) = create_data.pda(program_id);
        assert!(accounts.new_account.key.eq(&pda));

        create_data.check_authorities(accounts.authorities)?;

        if Self::AUTO_INCREMENT {
            match accounts.autoinc_account {
                Some(autoinc_account) => {
                    if autoinc_account.lamports() == 0 {
                        NautilusAutoincrementAccount::create(
                            program_id,
                            autoinc_account,
                            accounts.fee_payer.clone(),
                            accounts.system_program.clone(),
                            &Self::TABLE_NAME,
                        )?;
                    } else {
                        let mut autoinc_data = NautilusAutoincrementAccount::try_from_slice(
                            &autoinc_account.data.borrow_mut(),
                        )?;
                        autoinc_data.autoincrement(autoinc_account)?;
                    }
                }
                None => return Err(ProgramError::NotEnoughAccountKeys),
            }
        };

        invoke_signed(
            &system_instruction::create_account(
                accounts.fee_payer.key,
                accounts.new_account.key,
                create_data.lamports_required()?,
                create_data.size()?,
                program_id,
            ),
            &[
                accounts.new_account.clone(),
                accounts.fee_payer,
                accounts.system_program,
            ],
            &[&create_data.seeds_with_bump(bump.to_le_bytes().as_ref())],
        )?;

        create_data.serialize(&mut &mut accounts.new_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
