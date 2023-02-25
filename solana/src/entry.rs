pub use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program, sysvar,
};

// enum ButtMuncher {
//     CreatePerson,
//     DeletePerson,
//     UpdatePerson,
// }

// pub trait NautilusEntryProcessor {}
