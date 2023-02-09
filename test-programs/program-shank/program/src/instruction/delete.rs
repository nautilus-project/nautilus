use solana_program::{
    account_info::{AccountInfo, next_account_info}, 
    entrypoint::ProgramResult, 
    system_program,
};

pub fn delete_person(
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let person_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;

    let dest_starting_lamports = payer.lamports();
    **payer.lamports.borrow_mut() =
        dest_starting_lamports.checked_add(person_account.lamports()).unwrap();
    **person_account.lamports.borrow_mut() = 0;
    person_account.assign(&system_program::ID);
    person_account.realloc(0, false).map_err(Into::into)
}