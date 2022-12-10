use borsh::BorshSerialize;
use nautilus_solana::nautilus_derive_crud::Nautilus;

#[derive(Nautilus, BorshSerialize)]
#[nautilus(
    create,
    update,
    delete,
    primary_key = id,
    autoincrement = true,
)]
pub struct Hero {
    id: u32,
    name: String,
    kills: u32,
    current_weapon: u32,
}

#[derive(Nautilus, BorshSerialize)]
#[nautilus(
    create,
    update,
    delete,
    primary_key = id,
    autoincrement = true,
)]
pub struct Magic {
    id: u32,
    name: String,
    damage_boost: u32,
}

#[derive(Nautilus, BorshSerialize)]
#[nautilus(
    create,
    update,
    delete,
    primary_key = id,
    autoincrement = true,
)]
pub struct EquippedMagic {
    id: u32,
    hero_id: u32,
    magic_id: u32,
}

#[derive(Nautilus, BorshSerialize)]
#[nautilus(
    create,
    update,
    delete,
    primary_key = id,
    autoincrement = true,
)]
pub struct Weapon {
    id: u32,
    name: String,
    damage: u32,
}