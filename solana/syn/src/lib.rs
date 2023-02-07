mod entrypoint;
mod parser;
mod state;

use proc_macro2::TokenStream;
use syn::{
    ItemEnum,
    ItemStruct,
};

use entrypoint::*;
use parser::*;
use state::*;

// ---------------------------------------------------------------------
//             use nautilus::*;

//             #[derive(NautilusEntrypoint)]
//             enum MyInstructions {
//                 CreateHero,
//                 UpdateHero,
//                 DeleteHero,
//             }
// ---------------------------------------------------------------------
pub fn entrypoint(input: ItemEnum) -> TokenStream {
    let test = impl_entrypoint(input);
    println!("ENTRYPOINT:");
    println!("{:}", test);
    test
}

// ---------------------------------------------------------------------
//            #[derive(Nautilus)]
//            pub struct Hero {
//                #[primary_key(autoincrement = true)]
//                id: u32,
//                name: String,
//                kills: u32,
//                current_weapon: u32,
//                #[authority]
//                authority: Pubkey,
//            }
// ---------------------------------------------------------------------
pub fn state(input: ItemStruct) -> TokenStream {
    let (
        struct_name,
        struct_fields,
        primary_key_field,
        autoincrement,
    ) = parse_state(input.clone()).expect("Error processing tokens");
    impl_state(input, struct_name, struct_fields, primary_key_field, autoincrement)
}