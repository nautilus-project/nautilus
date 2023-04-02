use std::marker::PhantomData;

use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::NautilusAccountInfo;

#[derive(Clone)]
pub struct Mut<'a, T: NautilusAccountInfo<'a> + 'a> {
    phantom: PhantomData<&'a T>,
    pub self_account: T,
}

impl<'a, T: NautilusAccountInfo<'a> + 'a> Mut<'a, T> {
    pub fn new(self_account: T) -> Self {
        Self {
            self_account,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: NautilusAccountInfo<'a>> IntoAccountInfo<'a> for Mut<'a, T> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a, T: NautilusAccountInfo<'a>> NautilusAccountInfo<'a> for Mut<'a, T> {
    fn key(&self) -> &'a Pubkey {
        self.self_account.key()
    }

    fn is_signer(&self) -> bool {
        self.self_account.is_signer()
    }

    fn is_writable(&self) -> bool {
        self.self_account.is_writable()
    }

    fn lamports(&self) -> u64 {
        self.self_account.lamports()
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.self_account.mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.self_account.owner()
    }

    fn span(&self) -> usize {
        self.self_account.span()
    }
}

pub trait NautilusMut<'a>: NautilusAccountInfo<'a> + 'a {}

impl<'a, T: NautilusAccountInfo<'a> + 'a> NautilusMut<'a> for Mut<'a, T> {}
