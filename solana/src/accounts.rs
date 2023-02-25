pub use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program, sysvar,
};

pub trait NautilusAccountData: BorshDeserialize + BorshSerialize + Sized {
    const TABLE_NAME: &'static str;
    const AUTO_INCREMENT: bool;

    fn span(&self) -> Result<usize, ProgramError> {
        Ok((self.clone().try_to_vec()?).len())
    }

    fn size(&self) -> Result<u64, ProgramError> {
        Ok(self.span()?.try_into().unwrap())
    }

    fn lamports_required(&self) -> Result<u64, ProgramError> {
        use sysvar::Sysvar;
        Ok((sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()?))
    }

    fn primary_key<'a>(&self) -> &'a [u8];

    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key()]
    }

    fn seeds_with_bump<'a>(&self, bump: &'a [u8]) -> [&'a [u8]; 3] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key(), bump]
    }

    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}

pub trait NautilusAccountAuth: NautilusAccountData {
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
}

pub trait NautilusAccountCreate: NautilusAccountData + NautilusAccountAuth {
    fn autoincrement(autoinc_account: AccountInfo) -> ProgramResult {
        let mut counter =
            NautilusAutoincrementData::try_from_slice(&autoinc_account.data.borrow_mut())?;
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
        let (pda, bump) = args.data.pda(args.program_id);
        assert!(args.new_account.key.eq(&pda));

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
            &[&args.data.seeds_with_bump(bump.to_le_bytes().as_ref())],
        )?;

        args.data
            .serialize(&mut &mut args.new_account.data.borrow_mut()[..])?;

        Ok(())
    }
}

pub trait NautilusAccountDelete: NautilusAccountData + NautilusAccountAuth {
    fn parse_nautilus_delete_args<'a>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<NautilusDeleteArgs<'a>, ProgramError>;

    fn nautilus_delete(args: NautilusDeleteArgs) -> ProgramResult {
        let data = Self::try_from_slice(&args.target_account.data.borrow())?;
        data.check_authorities(args.authorities)?;

        let dest_starting_lamports = args.fee_payer.lamports();
        **args.fee_payer.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(args.target_account.lamports())
            .unwrap();
        **args.target_account.lamports.borrow_mut() = 0;
        args.target_account.assign(&system_program::ID);
        args.target_account.realloc(0, false).map_err(Into::into)
    }
}

pub trait NautilusAccountUpdate: NautilusAccountData + NautilusAccountAuth {
    fn parse_nautilus_update_args<'a, T: NautilusOptionized>(
        program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        update_data: T,
    ) -> Result<NautilusUpdateArgs<'a, T>, ProgramError>;

    fn process_nautilus_update_data<T: NautilusOptionized>(&mut self, update_data: T);

    fn nautilus_update<T: NautilusOptionized>(args: NautilusUpdateArgs<T>) -> ProgramResult {
        let (pda, _) = args.update_data.pda(args.program_id);
        assert!(args.target_account.key.eq(&pda));

        let mut data = Self::try_from_slice(&args.target_account.data.borrow_mut())?;
        data.check_authorities(args.authorities)?;

        data.process_nautilus_update_data(args.update_data);

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

pub trait NautilusOptionized: NautilusAccountData {}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NautilusAutoincrementData {
    pub count: u64,
}

pub struct NautilusCreateArgs<'a, T: NautilusAccountData> {
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

pub struct NautilusUpdateArgs<'a, T: NautilusOptionized> {
    pub program_id: &'a Pubkey,
    pub target_account: AccountInfo<'a>,
    pub authorities: Vec<AccountInfo<'a>>,
    pub fee_payer: AccountInfo<'a>,
    pub system_program: AccountInfo<'a>,
    pub update_data: T,
}
