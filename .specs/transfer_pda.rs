// Normal

pub fn transfer_sol_with_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;

    **payer.try_borrow_mut_lamports()? -= amount;
    **recipient.try_borrow_mut_lamports()? += amount;

    Ok(())
}

// Spec

struct ArbPda {}

struct AnotherArbPda {}

pub fn transfer_sol_with_program(from: ArbPda, to: AnotherArbPda, amount: u64) -> ProgramResult {
    from.transfer(to, amount)?;

    Ok(())
}
