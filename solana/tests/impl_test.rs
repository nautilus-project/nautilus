use borsh::BorshSerialize;
use nautilus_derive::{ 
    // Nautilus, 
    NautilusEntrypoint,
};
// use solana_program::pubkey::Pubkey;

pub fn impl_test() {
    println!();
    // println!("----------------------------------------------------------------------------");
    // println!("Testing Nautilus functionality for implementing NautilusCrud trait...");
    // println!();
    // println!();
    // println!("  Type Attributes:");
    // println!("  -- Table Name       : {}", Person::TABLE_NAME);
    // println!("  -- Primary Key      : {}", Person::PRIMARY_KEY);
    // println!("  -- Auto Increment   : {}", Person::AUTO_INCREMENT);
    // println!();
    // let person = Person::new_inner(1, "Joe C".to_string());
    // println!("  Object Attributes:");
    // println!("  -- Span             : {:?}", person.span().unwrap());
    // println!("  -- Size             : {:?}", person.size().unwrap());
    // // println!("  -- Lamports         : {:?}", person.lamports_required());
    // println!("  -- Address          : {}", person.address(&solana_program::system_program::ID).0);
    // println!("  -- Bump             : {}", person.address(&solana_program::system_program::ID).1);
    // println!();
    // println!("  Object Fields:");
    // println!("  -- Id               : {}", person.id);
    // println!("  -- Name             : {}", person.name);
    // println!();
    // println!("  Done.");
    // println!();
    // println!("----------------------------------------------------------------------------");
}

struct Joe {
    id: u32,
}

#[derive(NautilusEntrypoint)]
enum MyInstructions {
    CreateHero,
    UpdateHero,
    DeleteHero,
}

// #[derive(Nautilus, BorshSerialize)]
// #[nautilus()]
// pub struct Person {
//     id: u32,
//     name: String,
//     // authority: Pubkey,
// }
