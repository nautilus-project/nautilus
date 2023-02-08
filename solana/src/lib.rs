use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use solana_program::sysvar::{
    rent::Rent,
    Sysvar,
};
use std::io::Result;

pub trait NautilusAccount: BorshDeserialize + BorshSerialize + Sized {

    fn span(&self) -> Result<usize> {
        Ok((self.try_to_vec()?).len())
    }

    fn lamports_required(&self) -> Result<u64> {
        Ok((Rent::get().unwrap()).minimum_balance(self.span()?))
    }
    
    fn size(&self) -> Result<u64> {
        Ok(self.span()?.try_into().unwrap())
    }

    fn new() -> Self;

    fn from_data_slice(slice :&[u8]) -> Result<Self> {
        Self::try_from_slice(slice)
    }
}
