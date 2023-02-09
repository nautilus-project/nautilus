//
// --------------------
//      Nautilus
// --------------------
//
extern crate self as nautilus;

pub use borsh::{ BorshDeserialize, BorshSerialize };
pub use nautilus_derive::NautilusAccount;
pub use nautilus_derive::NautilusEntrypoint;
pub use solana_program::account_info::AccountInfo;
pub use solana_program::entrypoint;
pub use solana_program::entrypoint::ProgramResult;
pub use solana_program::pubkey::Pubkey;
pub use solana_program::sysvar::{
    rent::Rent,
    Sysvar,
};
use std::io::Result;

pub trait NautilusAccountBorsh: BorshDeserialize + BorshSerialize {

    fn span(&self) -> Result<usize> {
        Ok((self.try_to_vec()?).len())
    }

    fn size(&self) -> Result<u64> {
        Ok(self.span()?.try_into().unwrap())
    }

    fn lamports_required(&self) -> Result<u64> {
        Ok((Rent::get().unwrap()).minimum_balance(self.span()?))
    }
}