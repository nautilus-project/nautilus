pub mod associated_token;
pub mod metadata;
pub mod mint;

pub use associated_token::*;
pub use metadata::*;
pub use mint::*;

use crate::{NautilusAccountInfo, ProgramResult};

pub struct Token<'a> {
    mint: Mint<'a>,
    metadata: Option<Metadata<'a>>,
}

impl<'a> Token<'a> {
    fn create<T: NautilusAccountInfo<'a>>(
        decimals: u64,
        mint_authority: T,
        freeze_authority: T,
    ) -> Self {
        Self {
            mint: Mint::create(decimals, mint_authority, freeze_authority),
            metadata: None,
        }
    }

    fn create_with_metadata<T: NautilusAccountInfo<'a>>(
        decimals: u64,
        mint_authority: T,
        freeze_authority: T,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
    ) -> Self {
        Self {
            mint: Mint::create(decimals, mint_authority, freeze_authority),
            metadata: Some(Metadata::create(title, symbol, uri, update_authority)),
        }
    }

    fn add_metadata<T: NautilusAccountInfo<'a>>(
        mut self,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
    ) {
        self.metadata = Some(Metadata::create(title, symbol, uri, update_authority));
    }

    fn transfer<T: NautilusAccountInfo<'a>>(&self, from: T, to: T, amount: u64) -> ProgramResult {
        todo!()
    }
}
