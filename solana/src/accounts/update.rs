use crate::{
    invoke, next_account_info, system_instruction, AccountInfo, ProgramError, ProgramResult, Pubkey,
};

use super::{auth::NautilusAccountAuth, data::NautilusAccountData};

/// The trait that marks an "optionized" struct as being a valid optionized version of a PDA struct's data and thus allowing it to be used inside `NautilusUpdateArgs`.
pub trait NautilusOptionized: NautilusAccountData {
    /// Evaluates if an "optionized" field is either `Some(T)` or `None`.
    ///
    /// * If `Some(T)`, it will replace that field on the object with the new value from the `Some(T)` arg.
    /// * If `None`, it will do nothing to that field.
    fn process_nautilus_update_data<T: NautilusAccountData>(data: T, update_data: Self) -> T;
}

/// The default args for implementing the `nautilus_update(..)` instruction.
///
/// Includes all necessary accounts for the operation plus any specified authorities on the account from the #[authority] attribute.
///
/// Notice the generic value `T` is trait-enforced to implement `NautilusOptionized`.
///
/// This is because the data for the `update_data` field is actually an optionized version of the original struct, where all fields **except the primary key** are wrapped in `Option<>`.
pub struct NautilusUpdateArgs<'a, T: NautilusOptionized> {
    pub program_id: &'a Pubkey,
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
    pub update_data: T,
}

/// The trait that enables the default `nautilus_update(..)` instruction for the PDA.
pub trait NautilusAccountUpdate: NautilusAccountData + NautilusAccountAuth {
    /// Parses the program ID, list of accounts, and instruction args into the `NautilusUpdateArgs`.
    fn parse_nautilus_update_args<'a, T: NautilusOptionized>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        update_data: T,
    ) -> Result<NautilusUpdateArgs<'a, T>, ProgramError> {
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

        Ok(nautilus::NautilusUpdateArgs {
            program_id,
            target_account,
            authorities,
            fee_payer,
            system_program,
            update_data,
        })
    }

    /// Evaluates if an "optionized" field is either `Some(T)` or `None`.
    ///
    /// * If `Some(T)`, it will replace that field on the object with the new value from the `Some(T)` arg.
    /// * If `None`, it will do nothing to that field.
    ///
    /// Note: This function, under the `NautilusAccountUpdate` trait, calls on the same function in the `NautilusOptionized` trait to accopmlish this process.
    fn process_nautilus_update_data<T: NautilusOptionized>(data: Self, update_data: T) -> Self {
        T::process_nautilus_update_data(data, update_data)
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
    /// * update_data - The "optionized" data struct for this PDA, where each `Some(T)` is a field to be updated.
    ///
    /// # Returns
    ///
    /// `ProgramResult`
    fn nautilus_update<'a, T: NautilusOptionized>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        update_data: T,
    ) -> ProgramResult {
        let args = Self::parse_nautilus_update_args(program_id, accounts, update_data)?;

        let (pda, _) = args.update_data.pda(args.program_id);
        assert!(args.target_account.key.eq(&pda));

        let mut data = Self::try_from_slice(&args.target_account.data.borrow_mut())?;
        data.check_authorities(args.authorities)?;
        data = Self::process_nautilus_update_data(data, args.update_data);

        let diff = data.lamports_required()? - args.target_account.lamports();

        invoke(
            &system_instruction::transfer(args.fee_payer.key, args.target_account.key, diff),
            &[
                args.fee_payer.clone(),
                args.target_account.clone(),
                args.system_program.clone(),
            ],
        )?;

        args.target_account.realloc(data.span()?, false)?;

        data.serialize(&mut &mut args.target_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
