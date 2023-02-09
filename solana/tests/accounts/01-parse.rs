//
// Test that the derive(NautilusAccount) macro can actually
//      parse this struct
//
use nautilus::*;

#[derive(NautilusAccount)]
struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}

fn main() {}
