use crate::state::{Hero, Villain};
use nautilus::*;

pub struct CustomAccounts<'a> {
    target_account: &'a AccountInfo<'a>,
    authority: &'a AccountInfo<'a>,
    fee_payer: &'a AccountInfo<'a>,
    system_program: &'a AccountInfo<'a>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CustomArgs {
    string1: String,
    string2: String,
}

pub fn parse_accounts<'a>(accounts: &'a [AccountInfo<'a>]) -> Result<CustomAccounts, ProgramError> {
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let fee_payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    Ok(CustomAccounts {
        target_account,
        authority,
        fee_payer,
        system_program,
    })
}

pub fn custom<'a>(
    program_id: &'a Pubkey,
    passed_accounts: &'a [AccountInfo<'a>],
    args: CustomArgs,
) -> ProgramResult {
    let _accounts = parse_accounts(passed_accounts)?;

    let hero = Hero {};
    Hero::nautilus_create(program_id, accounts, hero);

    let villain = Villain {};
    Villian::nautilus_create(program_id, accounts, villain);

    Ok(())
}
