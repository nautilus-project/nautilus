use super::NautilusAccountInfo;

#[derive(Clone)]
pub struct Signer<T>
where
    T: Clone,
{
    pub self_account: T,
}

impl<T> Signer<T>
where
    T: Clone,
{
    pub fn new(self_account: T) -> Self {
        Self { self_account }
    }
}

pub trait NautilusSigner<'a>: NautilusAccountInfo<'a> {}
