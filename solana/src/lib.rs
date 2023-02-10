//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//

extern crate self as nautilus;

pub use borsh::{ BorshDeserialize, BorshSerialize };
pub use nautilus_derive::{ NautilusAccount, NautilusEntrypoint };
pub use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint,
    entrypoint::ProgramResult,
    program::{ invoke, invoke_signed },
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program,
    sysvar,
};

pub trait NautilusAccountBorsh: BorshDeserialize + BorshSerialize {

    fn span(&self) -> Result<usize, ProgramError> {
        Ok((self.try_to_vec()?).len())
    }

    fn size(&self) -> Result<u64, ProgramError> {
        Ok(self.span()?.try_into().unwrap())
    }

    fn lamports_required(&self) -> Result<u64, ProgramError> {
        use sysvar::Sysvar;
        Ok(
            (sysvar::rent::Rent::get().unwrap())
                .minimum_balance(self.span()?)
        )
    }
}

pub struct NautilusCreateArgs<'a, T: Sized + NautilusAccountBorsh> {
    new_account: AccountInfo<'a>,
    fee_payer: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    program_id: &'a Pubkey,
    data: T,
}

pub trait NautilusAccountCreate: Sized + NautilusAccountBorsh {

    fn parse_args<'a>(
        program_id: &'a Pubkey, 
        accounts: &'a [AccountInfo<'a>], 
        create_instruction_args: Self,
    ) -> Result<NautilusCreateArgs<'a, Self>, ProgramError> {

        let accounts_iter = &mut accounts.iter();
        let new_account = next_account_info(accounts_iter)?.to_owned();
        let fee_payer = next_account_info(accounts_iter)?.to_owned();
        let system_program = next_account_info(accounts_iter)?.to_owned();

        Ok(NautilusCreateArgs { 
            new_account, 
            fee_payer, 
            system_program, 
            program_id, 
            data: create_instruction_args, 
        })
    }

    fn create(args: NautilusCreateArgs<Self>) -> ProgramResult {

        // TODO: Implement Shank
        // assert!(
        //     args.new_account == args.data.shank_pda().0
        // );

        invoke_signed(
            &system_instruction::create_account(
                args.fee_payer.key, 
                args.new_account.key, 
                args.data.lamports_required()?, 
                args.data.size()?, 
                args.program_id,
            ), 
            &[
                args.new_account.clone(), 
                args.fee_payer, 
                args.system_program,
            ], 
            // TODO: Implement Shank
            // args.data.shank_seeds_with_bump(),
            &[&[]],
        )?;

        args.data.serialize(&mut &mut args.new_account.data.borrow_mut()[..])?;

        Ok(())
    }

}

pub struct NautilusDeleteArgs<'a> {
    target_account: AccountInfo<'a>,
    fee_payer: AccountInfo<'a>,
}

pub trait NautilusAccountDelete: Sized + NautilusAccountBorsh {

    fn parse_args<'a>(
        accounts: &'a [AccountInfo<'a>], 
    ) -> Result<NautilusDeleteArgs<'a>, ProgramError> {

        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?.to_owned();
        let fee_payer = next_account_info(accounts_iter)?.to_owned();

        Ok(NautilusDeleteArgs { 
            target_account, 
            fee_payer, 
        })
    }

    fn delete(args: NautilusDeleteArgs) -> ProgramResult {

        // TODO: Implement Shank
        // assert!(
        //     args.new_account == args.data.shank_pda().0
        // );

        let dest_starting_lamports = args.fee_payer.lamports();
        **args.fee_payer.lamports.borrow_mut() =
            dest_starting_lamports.checked_add(args.target_account.lamports()).unwrap();
        **args.target_account.lamports.borrow_mut() = 0;
        args.target_account.assign(&system_program::ID);
        args.target_account.realloc(0, false).map_err(Into::into)
    }

}

pub struct NautilusUpdateArgs<'a, T: Sized + NautilusAccountBorsh> {
    target_account: AccountInfo<'a>,
    // TODO: Figure out how to check authority, if it's set
    authority: Option<AccountInfo<'a>>,
    update_data: T,
}

pub trait NautilusAccountUpdate: Sized + NautilusAccountBorsh {

    fn parse_args<'a>(
        accounts: &'a [AccountInfo<'a>], 
        update_data: Self,
    ) -> Result<NautilusUpdateArgs<'a, Self>, ProgramError> {

        let accounts_iter = &mut accounts.iter();
        let target_account = next_account_info(accounts_iter)?.to_owned();
        let authority = match next_account_info(accounts_iter) {
            Ok(account_info) => Some(account_info.to_owned()),
            Err(_) => None,
        };

        Ok(NautilusUpdateArgs { 
            target_account, 
            authority,
            update_data, 
        })
    }

    fn process_update_data(_data: Self, _update_data: Self) -> Self {
        todo!()
    }

    fn update(args: NautilusUpdateArgs<Self>) -> ProgramResult {

        // TODO: Implement Shank
        // assert!(
        //     args.new_account == args.data.shank_pda().0
        // );

        let mut data = Self::try_from_slice(&args.target_account.data.borrow_mut())?;
        data = Self::process_update_data(data, args.update_data);
        data.serialize(&mut &mut args.target_account.data.borrow_mut()[..])?;

        Ok(())
    }

}