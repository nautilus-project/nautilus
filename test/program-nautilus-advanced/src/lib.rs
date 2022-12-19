use nautilus::*;

mod instructions;
mod state;

use instructions::*;

#[derive(Nautilus)]
enum MyInstructions {
    CreateHero,
    UpdateHero,
    DeleteHero,
    CreateMagic,
    UpdateMagic,
    DeleteMagic,
    CreateWeapon,
    UpdateWeapon(UpdateWeaponInstruction),
    DeleteWeapon,
}

entrypoint!(MyInstructions::entrypoint)