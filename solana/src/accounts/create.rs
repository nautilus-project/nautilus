use crate::{
    invoke_signed, next_account_info, system_instruction, AccountInfo, BorshDeserialize,
    NautilusAutoincrementAccount, ProgramError, ProgramResult, Pubkey,
};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The default args for implementing the `nautilus_create(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
pub struct NautilusCreateArgs<'a, T: NautilusAccountData> {
    pub program_id: &'a Pubkey,
    pub autoinc_account: Option<AccountInfo<'a>>,
    pub new_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
    pub create_data: T,
}

/// The trait that enables the default `nautilus_create(..)` instruction for the PDA.
pub trait NautilusAccountCreate: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID, list of accounts, and instruction args into the `NautilusCreateArgs`.
    fn parse_nautilus_create_args<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        create_data: Self,
    ) -> Result<NautilusCreateArgs<'a, Self>, ProgramError> {
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

        Ok(nautilus::NautilusCreateArgs {
            program_id,
            autoinc_account,
            new_account,
            authorities,
            fee_payer,
            system_program,
            create_data,
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
    /// * `NautilusCreateArgs` - the necessary accounts and data for creating this PDA.
    ///
    /// # Returns
    ///
    /// `ProgramResult`
    fn nautilus_create<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        create_data: Self,
    ) -> ProgramResult {
        let args = Self::parse_nautilus_create_args(program_id, accounts, create_data)?;
        let (pda, bump) = args.create_data.pda(args.program_id);
        assert!(args.new_account.key.eq(&pda));

        args.create_data.check_authorities(args.authorities)?;

        if Self::AUTO_INCREMENT {
            match args.autoinc_account {
                Some(autoinc_account) => {
                    if autoinc_account.lamports() == 0 {
                        NautilusAutoincrementAccount::create(
                            args.program_id,
                            autoinc_account,
                            args.fee_payer.clone(),
                            args.system_program.clone(),
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
                args.fee_payer.key,
                args.new_account.key,
                args.create_data.lamports_required()?,
                args.create_data.size()?,
                args.program_id,
            ),
            &[
                args.new_account.clone(),
                args.fee_payer,
                args.system_program,
            ],
            &[&args
                .create_data
                .seeds_with_bump(bump.to_le_bytes().as_ref())],
        )?;

        args.create_data
            .serialize(&mut &mut args.new_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
