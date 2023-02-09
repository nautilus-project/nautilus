use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize 
    },
    solana_program::{
        account_info::{AccountInfo, next_account_info}, 
        entrypoint::ProgramResult, 
        program::invoke,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
};
use crate::state::Person;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePersonArgs {
    pub id: u32,
    pub name: Option<String>,
    pub authority: Option<Pubkey>,
}

pub fn update_person(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: UpdatePersonArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let person_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // let (person_account_pda, person_account_bump) = Person::shank_pda(program_id, args.id);
    let (person_account_pda, _) = Pubkey::find_program_address(
        &[
            b"person", 
            args.id.to_le_bytes().as_ref()
        ],
        program_id,
    );
    assert!(&person_account_pda == person_account.key);

    let person_data = Person::try_from_slice(&person_account.data.borrow())?;

    let mut new_person_data = person_data.clone();
    match args.name {
        Some(name) => new_person_data.name = name,
        None => (),
    }
    match args.authority {
        Some(authority) => new_person_data.authority = authority,
        None => (),
    }

    let account_span = (new_person_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);
    
    let diff = lamports_required - person_account.lamports();
    invoke(
        &system_instruction::transfer(payer.key, person_account.key, diff),
        &[payer.clone(), person_account.clone(), system_program.clone()],
    )?;

    person_account.realloc(account_span, false)?;
    
    person_data.serialize(&mut &mut person_account.data.borrow_mut()[..])?;

    Ok(())
}