mod custom_instruction;
mod instruction;
mod state;

use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_hero(new_hero: Create<Hero>, fee_payer: Wallet, id: u8, name: String) -> ProgramResult {
        new_hero.create(id, name)
    }

    fn create_villain(new_villain: Create<Villain>, fee_payer: Wallet, id: u8, name: String) -> ProgramResult {
        new_villain.create(id, name, (id))
    }
}

#[derive(Nautilus)]
pub struct Hero {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(Nautilus)]
pub struct Villain {
    #[primary_key(autoincrement = false)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}