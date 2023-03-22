//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//
extern crate self as nautilus;

pub mod entry;
pub mod objects;

pub use borsh::{self, BorshDeserialize, BorshSerialize};
pub use nautilus_derive::{nautilus, Nautilus};
pub use shank::{ShankAccount, ShankInstruction};
pub use solana_program::{
    account_info::{next_account_info, AccountInfo, IntoAccountInfo},
    declare_id, entrypoint,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program, sysvar,
};

pub use crate::entry::*;
pub use crate::objects::*;

#[derive(Nautilus, borsh::BorshDeserialize, borsh::BorshSerialize)]
#[default_instructions(Create, Delete)]
pub struct Darryl {
    #[primary_key]
    id: u8,
    #[authority]
    authority: Pubkey,
}
