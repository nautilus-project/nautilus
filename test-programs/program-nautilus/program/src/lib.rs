mod custom_instruction;
mod instruction;
mod state;

use nautilus::*;

use crate::custom_instruction::custom;
use crate::instruction::MyInstructions;
use crate::state::{Hero, Villain};

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = MyInstructions::try_from_slice(input)?;
    match instruction {
        MyInstructions::CreateHero(args) => Hero::nautilus_create(program_id, accounts, args),
        MyInstructions::DeleteHero => Hero::nautilus_delete(accounts),
        MyInstructions::UpdateHero(args) => Hero::nautilus_update(program_id, accounts, args),
        MyInstructions::CreateVillain(args) => Villain::nautilus_create(program_id, accounts, args),
        MyInstructions::DeleteVillain => Villain::nautilus_delete(accounts),
        MyInstructions::UpdateVillain(args) => Villain::nautilus_update(program_id, accounts, args),
        MyInstructions::CustomInstruction(args) => custom(program_id, accounts, args),
    }
}
