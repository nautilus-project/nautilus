use nautilus_derive::NautilusEntrypoint;

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreatePerson,
    UpdatePerson,
    DeletePerson,
    CreateHero,
    UpdateHero,
    DeleteHero,
}

pub fn impl_test() {
    println!();
}