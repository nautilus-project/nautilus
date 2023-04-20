use solana_program::pubkey::Pubkey;

use super::NautilusAccountInfo;

#[derive(Clone)]
pub struct Signer<T>
where
    T: NautilusAccountInfo,
{
    pub self_account: Box<T>,
}

impl<T> Signer<T>
where
    T: NautilusAccountInfo,
{
    pub fn new(self_account: Box<T>) -> Self {
        Self { self_account }
    }
}

impl<T> NautilusAccountInfo for Signer<T>
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

pub trait NautilusSigner: NautilusAccountInfo {}
