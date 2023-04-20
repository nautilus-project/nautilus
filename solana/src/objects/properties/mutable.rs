use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use super::NautilusAccountInfo;

#[derive(Clone)]
pub struct Mut<T>
where
    T: NautilusAccountInfo,
{
    pub self_account: Box<T>,
}

impl<T> Mut<T>
where
    T: NautilusAccountInfo,
{
    pub fn new(self_account: Box<T>) -> Self {
        Self { self_account }
    }
}

impl<T> NautilusAccountInfo for Mut<T>
where
    T: NautilusAccountInfo,
{
    fn key(&self) -> &Pubkey {
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

    fn owner(&self) -> &Pubkey {
        self.self_account.owner()
    }

    fn span(&self) -> usize {
        self.self_account.span()
    }
}

pub trait NautilusMut: NautilusAccountInfo {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError>;
}
