//
// Test that the derive(NautilusAccount) macro will implement
//      the necessary methods to use borsh methods
//
use nautilus::*;
use std::io::Result;

#[derive(NautilusAccount)]
struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}

fn main() -> Result<()> {

    let person = Person {
        id: 30,
        name: String::from("Joe"),
        authority: Pubkey::new_unique(),
    };

    person.test_fn();

    person.serialize(&mut [..])?;
    // let person_struct = match Person::try_from_slice(&person_slice) {
    //     Ok(p) => p,
    //     Err(_) => panic!("Failed to deserialize"),
    // };

    // let person_span = person_struct.span();
    // let person_size = person_struct.size();
    // let person_lamports = person_struct.lamports_required();

    let _ = person;

    Ok(())
}
