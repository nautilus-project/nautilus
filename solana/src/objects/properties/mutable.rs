use super::NautilusAccountInfo;

#[derive(Clone)]
pub struct Mut<T>
where
    T: Clone,
{
    pub self_account: T,
}

impl<T> Mut<T>
where
    T: Clone,
{
    pub fn new(self_account: T) -> Self {
        Self { self_account }
    }
}

pub trait NautilusMut<'a>: NautilusAccountInfo<'a> + 'a {}
