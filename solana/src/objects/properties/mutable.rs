use std::marker::PhantomData;

use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::NautilusAccountInfo;

#[derive(Clone)]
pub struct Mut<T>
where
    T: NautilusAccountInfo,
{
    pub self_account: T,
}

impl<T> Mut<T>
where
    T: NautilusAccountInfo,
{
    pub fn new(self_account: T) -> Self {
        Self { self_account }
    }
}

// impl<'a, T: NautilusAccountInfo> IntoAccountInfo for Mut<'a, T> {
//     fn into_account_info(self) -> AccountInfo<'a> {
//         todo!()
//     }
// }

impl<T> NautilusAccountInfo for Mut<T>
where
    T: NautilusAccountInfo,
{
    // fn key<'a>(&self) -> &'a Pubkey {
    //     self.self_account.key()
    // }

    fn is_signer(&self) -> bool {
        self.self_account.is_signer()
    }

    fn is_writable(&self) -> bool {
        self.self_account.is_writable()
    }

    fn lamports(&self) -> u64 {
        self.self_account.lamports()
    }

    // fn mut_lamports<'a>(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
    //     self.self_account.mut_lamports()
    // }

    // fn owner<'a>(&self) -> &'a Pubkey {
    //     self.self_account.owner()
    // }

    fn span(&self) -> usize {
        self.self_account.span()
    }

    // fn key(&self) -> Pubkey {
    //     self.self_account.key()
    // }
}

pub trait NautilusMut: NautilusAccountInfo {}

impl<T> NautilusMut for Mut<T> where T: NautilusAccountInfo {}
