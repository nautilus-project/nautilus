use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize 
    },
    solana_program::{
        account_info::{AccountInfo, next_account_info}, 
        entrypoint::ProgramResult, 
        program::invoke_signed,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
};
use crate::state::Person;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreatePersonArgs {
    pub id: u32,
    pub name: String,
    pub authority: Pubkey,
}

pub fn create_person(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreatePersonArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let autoinc_account = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let fee_payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // let (new_account_pda, new_account_bump) = Person::shank_pda(program_id, args.id);
    let (new_account_pda, new_account_bump) = Pubkey::find_program_address(
        &[
            b"person", 
            args.id.to_le_bytes().as_ref()
        ],
        program_id,
    );
    assert!(&new_account_pda == new_account.key);

    let person_data = Person {
        id: args.id,
        name: args.name,
        authority: args.authority,
    };

    let account_span = (person_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke_signed(
        &system_instruction::create_account(
            &fee_payer.key,
            &new_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            fee_payer.clone(), new_account.clone(), system_program.clone()
        ],
        // Person::shank_seeds_with_bump(args.id, &[new_account_bump]),
        &[&[
            b"person",
            person_data.id.to_le_bytes().as_ref(),
            new_account_bump.to_le_bytes().as_ref(),
        ]]
    )?;
    
    person_data.serialize(&mut &mut new_account.data.borrow_mut()[..])?;

    Ok(())
}