//
// Test that the derive(NautilusEntrypoint) macro can actually
//      parse this enum
//
use nautilus::*;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreatePerson,
    UpdatePerson,
    DeletePerson,
}

fn main() {}
