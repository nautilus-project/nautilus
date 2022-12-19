use nautilus_solana::nautilus_derive::NautilusEntrypoint;
// use nautilus_solana::nautilus_derive_crud::Nautilus;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreateHero,
    UpdateHero,
    DeleteHero,
}

// #[derive(Nautilus)]
// #[nautilus(
//     primary_key = id,
//     autoincrement = true,
// )]
// pub struct Hero {
//     id: u32,
//     name: String,
//     kills: u32,
//     current_weapon: u32,
// }