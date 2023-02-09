//
// Test that the derive(NautilusEntrypoint) macro will implement
//      the necessary entrypoint & processor for the program
//
use nautilus::*;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreatePerson,
    UpdatePerson,
    DeletePerson,
}

fn main() {}
