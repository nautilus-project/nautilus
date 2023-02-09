mod instruction;
mod state;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    declare_id,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use crate::instruction::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {

    let instruction = MyInstruction::try_from_slice(input)?;

    match instruction {
        MyInstruction::CreatePerson(args) => create_person(program_id, accounts, args),
        MyInstruction::DeletePerson => delete_person(accounts),
        MyInstruction::UpdatePerson(args) => update_person(program_id, accounts, args),
    }
}

declare_id!("45A6jtRE6Tr71EpRATyWF8FYUNP7LEZ7NFd3Xb9LJ4TR");
entrypoint!(process_instruction);