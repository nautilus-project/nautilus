use nautilus::*;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreateHero,
    DeleteHero,
    UpdateHero,
    CreateVillain,
    DeleteVillain,
    UpdateVillain,
}

#[derive(NautilusAccount)]
pub struct Hero {
    #[primary_key(autoincrement = true)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(NautilusAccount)]
pub struct Villain {
    #[primary_key(autoincrement = true)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}

// ------------------------------------
// -- Concept --
//

#[derive(NautilusAccount)]
pub struct Sidekick {
    #[primary_key(autoincrement = true)]
    id: u32,
    #[foreign_key(table = Hero)]
    hero_id: u32
    name: String,
    #[authority(inherit_from_parent = true)]
    authority: Pubkey,
}