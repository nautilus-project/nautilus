use nautilus::*;
use shank::ShankAccount;

#[derive(NautilusAccount, ShankAccount)]
pub struct Hero {
    #[primary_key(autoincrement = false)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(NautilusAccount, ShankAccount)]
pub struct Villain {
    #[primary_key(autoincrement = false)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
