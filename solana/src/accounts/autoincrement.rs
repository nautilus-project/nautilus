use crate::{
    invoke_signed, system_instruction, AccountInfo, BorshDeserialize, BorshSerialize,
    ProgramResult, Pubkey,
};

/// The inner data for a table's associated autoincrement account.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NautilusAutoincrementAccount {
    pub count: u64,
}

impl NautilusAutoincrementAccount {
    /// Creates a new instance of the counter data for an autoincrement account.
    pub fn new() -> Self {
        Self { count: 1 }
    }

    /// If autoincrement is enabled for the PDA, increments the autoincrement account by 1.
    pub fn autoincrement(&mut self, autoinc_account: AccountInfo) -> ProgramResult {
        self.count += 1;
        self.serialize(&mut &mut autoinc_account.data.borrow_mut()[..])?;
        Ok(())
    }

    /// Used to create the autoincrement account.
    pub fn create<'a>(
        program_id: &Pubkey,
        autoinc_account: AccountInfo<'a>,
        fee_payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
        table_name: &'a str,
    ) -> ProgramResult {
        use nautilus::sysvar::Sysvar;

        let autoinc_data = Self::new();

        let (pda, bump) =
            Pubkey::find_program_address(&[table_name.as_bytes(), b"autoincrement"], program_id);
        assert!(autoinc_account.key.eq(&pda));

        let span = (autoinc_data.try_to_vec()?).len();
        let rent = (nautilus::sysvar::rent::Rent::get().unwrap()).minimum_balance(span);

        invoke_signed(
            &system_instruction::create_account(
                fee_payer.key,
                autoinc_account.key,
                rent,
                span as u64,
                program_id,
            ),
            &[autoinc_account.clone(), fee_payer, system_program],
            &[&[table_name.as_bytes(), b"autoincrement", &[bump]]],
        )?;

        autoinc_data.serialize(&mut &mut autoinc_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
