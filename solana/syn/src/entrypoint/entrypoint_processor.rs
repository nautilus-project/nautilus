use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn build_processor(_enum_name: &Ident) -> TokenStream {
    quote! {
        Ok(())
    }
}

pub fn processor(enum_name: &Ident) -> TokenStream {

    let processor = build_processor(enum_name);

    quote! {

        use solana_program::{
            account_info::AccountInfo,
            entrypoint,
            entrypoint::ProgramResult,
            pubkey::Pubkey,
        };
    
        fn process_instruction(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            input: &[u8],
        ) -> ProgramResult {
            
            #processor

        }

        entrypoint!(process_instruction);
    }
}

// --------------------------------------------------------------------
// Sample code to be written:
//
// use solana_program::{
//     account_info::AccountInfo,
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
// };
// 
// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     input: &[u8],
// ) -> ProgramResult {
//  
//     let instruction = MyInstruction::try_from_slice(input)?;
// 
//     match instruction {
//         MyInstruction::CreatePerson(args) => Person::create_person(program_id, accounts, args),
//         MyInstruction::DeletePerson(args) => Person::delete_person(program_id, accounts, args),
//         MyInstruction::UpdatePerson(args) => Person::update_person(program_id, accounts, args),
//     }
// 
// }
// 
// entrypoint!(process_instruction);
