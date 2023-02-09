//
// Test that the derive(NautilusAccount) macro will implement
//      the necessary methods to manipulate it's data
//
use nautilus::*;

#[derive(NautilusAccount)]
struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}

fn main() {

    let person = Person {
        id: 30,
        name: String::from("Joe"),
        authority: Pubkey::new_unique(),
    };

    let _ = person;
}
