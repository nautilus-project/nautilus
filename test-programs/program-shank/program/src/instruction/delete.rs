use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint::ProgramResult, 
    system_program,
};

pub fn delete_person(
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let fee_payer = next_account_info(accounts_iter)?;

    let dest_starting_lamports = fee_payer.lamports();
    **fee_payer.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(target_account.lamports()).unwrap();
    **target_account.lamports.borrow_mut() = 0;
    target_account.assign(&system_program::ID);
    target_account.realloc(0, false).map_err(Into::into)
}