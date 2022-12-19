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

//             entrypoint!(MyInstructions::entrypoint)
// ---------------------------------------------------------------------
/**
 * 
 * Here's where the derive(NautilusEntrypoint) macro is invoked.
 * 
 * We need to parse out the bits about the enum and use them to 
 *      generate the proper implementation for `MyEnum::entrypoint`.
 */
pub fn entrypoint(input: ItemEnum) -> TokenStream {
    let (
        enum_name,
        enum_variants,
    ) = parse_entrypoint(input.clone()).expect("Error processing tokens");
    impl_entrypoint(input, enum_name, enum_variants)
}

// ---------------------------------------------------------------------
//             #[derive(Nautilus)]
//             #[nautilus(
//                 primary_key = id,
//                 autoincrement = true,
//             )]
//             pub struct Hero {
//                 id: u32,
//                 name: String,
//                 kills: u32,
//                 current_weapon: u32,
//             }
// ---------------------------------------------------------------------
/**
 * Here's where the derive(Nautilus) macro is invoked.
 * 
 * We need to parse out the bits about the struct and use them to
 *      generate a bunch of functions for that struct.
 * Some functions will be object methods, some will be associated functions.
 * We will implement all functions for CRUD, even if they are not used.
 * Ultimately, the NautilusEntrypoint macro above determines which ones to invoke 
 *      from an instruction.
 */
pub fn state(input: ItemStruct) -> TokenStream {
    let (
        struct_name,
        struct_fields,
        primary_key_field,
        autoincrement,
    ) = parse_state(input.clone()).expect("Error processing tokens");
    impl_state(input, struct_name, struct_fields, primary_key_field, autoincrement)
}