//
// Test that the derive(NautilusAccount) macro will implement
//      the necessary methods to use borsh methods
//
use nautilus::*;

#[derive(NautilusAccount)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u32,
    name: String,
    #[authority]
    authority: Pubkey,
}

fn main() {}
