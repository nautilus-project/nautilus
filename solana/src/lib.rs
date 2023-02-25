//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//
extern crate self as nautilus;

pub mod accounts;
pub mod entry;

pub use borsh::{BorshDeserialize, BorshSerialize};
pub use nautilus_derive::{Nautilus, NautilusAccount};
pub use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program, sysvar,
};

pub use crate::accounts::*;
pub use crate::entry::*;
