//
//
// ----------------------------------------------------------------
//                          Nautilus
// ----------------------------------------------------------------
//
//
extern crate self as nautilus;

pub mod cpi;
pub mod objects;

pub use borsh::{self, BorshDeserialize, BorshSerialize};
pub use nautilus_derive::{nautilus, Nautilus};
pub use solana_program::{
    account_info::{next_account_info, AccountInfo, IntoAccountInfo},
    declare_id, entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program, sysvar,
};

pub use objects::{
    // properties::{create::*, mutable::*, record::*, signer::*, *},
    properties::{create::*, mutable::*, signer::*, *},
    // record::{index::*, *},
    // token::{associated_token::*, metadata::*, mint::*, *},
    wallet::*,
};

fn create_wallet_with_payer<'a>(
    mut new_wallet: Create<Wallet>,
    rent_payer: Signer<Wallet>,
) -> ProgramResult {
    new_wallet.create_with_payer(rent_payer)
}
