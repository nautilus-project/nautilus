// Normal

fn create_account(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    invoke(
        &system_instruction::create_account(
            &payer.key,
            &new_account.key,
            1 * LAMPORTS_PER_SOL,
            0,
            &system_program::ID,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;

    Ok(())
}

// Spec

fn create_account(new_account: Create<Wallet>) -> ProgramResult {
    let account = new_account.create()?;
    Ok(())
}
