// use nautilus_solana::{NautilusEntrypoint, Nautilus};
use nautilus_derive::NautilusEntrypoint;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreateHero,
    UpdateHero,
    DeleteHero,
}

// #[derive(Nautilus)]
// pub struct Hero {
//     #[primary_key(autoincrement = true)]
//     id: u32,
//     name: String,
//     #[authority]
//     authority: Pubkey,
// }