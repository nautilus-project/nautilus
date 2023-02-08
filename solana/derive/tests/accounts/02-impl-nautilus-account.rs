use nautilus_derive::Nautilus;
use solana_program::pubkey::Pubkey;

#[derive(Nautilus)]
struct Person {
    id: u32,
    name: String,
    authority: Pubkey,
}

fn main() {
    let builder = Person::builder();

    let _ = builder;
}
