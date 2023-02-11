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

pub trait NautilusAccountBase: BorshDeserialize + BorshSerialize + Sized {

    const AUTO_INCREMENT: bool;

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

    fn seeds<'a>(&self) -> &[&'a [u8]];

    fn seeds_with_bump<'a>(&self, program_id: &Pubkey) -> &[&'a [u8]];

    fn pda(&self, program_id: &Pubkey) -> (Pubkey, u8);

    fn check_pda(&self, program_id: &Pubkey, key: &Pubkey) -> Result<(), ProgramError> {
        if key != &self.pda(program_id).0 {
            return Err(ProgramError::InvalidSeeds)
        }
        Ok(())
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;

    fn autoincrement(autoinc_account: AccountInfo) -> ProgramResult {
        let mut counter = NautilusAutoincrementData::try_from_slice(
            &autoinc_account.data.borrow_mut()
        )?;
        counter.count += 1;
        counter.serialize(&mut &mut autoinc_account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn parse_nautilus_create_args<'a>(
        program_id: &'a Pubkey, 
        accounts: &'a [AccountInfo<'a>], 
        create_instruction_args: Self,
    ) -> Result<NautilusCreateArgs<'a, Self>, ProgramError>;

    fn nautilus_create(args: NautilusCreateArgs<Self>) -> ProgramResult {

        args.data.check_pda(args.program_id, args.fee_payer.key)?;
        args.data.check_authorities(args.authorities)?;

        if Self::AUTO_INCREMENT { 
            match args.autoinc_account {
                Some(autoinc_account) => Self::autoincrement(autoinc_account)?,
                None => return Err(ProgramError::NotEnoughAccountKeys),
            }
         };

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
            // &[&args.data.seeds_with_bump(args.program_id)],
            &[&[]] // TODO: seeds
        )?;

        args.data.serialize(&mut &mut args.new_account.data.borrow_mut()[..])?;

        Ok(())
    }

    fn parse_nautilus_delete_args<'a>(
        program_id: &'a Pubkey, 
        accounts: &'a [AccountInfo<'a>], 
    ) -> Result<NautilusDeleteArgs<'a>, ProgramError>;

    fn nautilus_delete(args: NautilusDeleteArgs) -> ProgramResult {

        let data = Self::try_from_slice(&args.target_account.data.borrow())?;
        data.check_authorities(args.authorities)?;

        let dest_starting_lamports = args.fee_payer.lamports();
        **args.fee_payer.lamports.borrow_mut() =
            dest_starting_lamports.checked_add(args.target_account.lamports()).unwrap();
        **args.target_account.lamports.borrow_mut() = 0;
        args.target_account.assign(&system_program::ID);
        args.target_account.realloc(0, false).map_err(Into::into)
    }

    fn parse_nautilus_update_args<'a>(
        program_id: &'a Pubkey, 
        accounts: &'a [AccountInfo<'a>], 
        update_data: Self,
    ) -> Result<NautilusUpdateArgs<'a, Self>, ProgramError>;

    fn process_nautilus_update_data(_data: Self, _update_data: Self) -> Self {
        todo!()
    }

    fn nautilus_update(args: NautilusUpdateArgs<Self>) -> ProgramResult {

        args.update_data.check_pda(args.program_id, args.target_account.key)?;

        let mut data = Self::try_from_slice(&args.target_account.data.borrow_mut())?;
        data.check_authorities(args.authorities)?;

        data = Self::process_nautilus_update_data(data, args.update_data);
        data.serialize(&mut &mut args.target_account.data.borrow_mut()[..])?;

        Ok(())
    }

}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NautilusAutoincrementData {
    pub count: u64,
}

pub struct NautilusCreateArgs<'a, T: NautilusAccountBase> {
    pub program_id: &'a Pubkey, 
    pub autoinc_account: Option<AccountInfo<'a>>,
    pub new_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
    pub data: T,
}

pub struct NautilusDeleteArgs<'a> {
    pub program_id: &'a Pubkey, 
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
}

pub struct NautilusUpdateArgs<'a, T: NautilusAccountBase> {
    pub program_id: &'a Pubkey, 
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub update_data: T,
}