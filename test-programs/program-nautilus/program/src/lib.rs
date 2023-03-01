use nautilus::*;
use shank::{ShankAccount, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, ShankInstruction)]
enum MyInstructions {
    CreateHero(Hero),
    DeleteHero,
    UpdateHero(HeroOptionized),
    CreateVillain(Villain),
    DeleteVillain,
    UpdateVillain(VillainOptionized),
}

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = MyInstructions::try_from_slice(input)?;
    match instruction {
        MyInstructions::CreateHero(args) => Hero::nautilus_create(program_id, accounts, args),
        MyInstructions::DeleteHero => Hero::nautilus_delete(program_id, accounts),
        MyInstructions::UpdateHero(args) => Hero::nautilus_update(program_id, accounts, args),
        MyInstructions::CreateVillain(args) => Villain::nautilus_create(program_id, accounts, args),
        MyInstructions::DeleteVillain => Villain::nautilus_delete(program_id, accounts),
        MyInstructions::UpdateVillain(args) => Villain::nautilus_update(program_id, accounts, args),
    }
}

// ----------------------------------------------------

#[derive(NautilusAccount, ShankAccount)]
pub struct Hero {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(NautilusAccount, ShankAccount)]
pub struct Villain {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
