pub trait NautilusCreateAssociatedTokenAccount<'a> {
    fn create(
        &self,
        mint: crate::objects::token::mint::Mint<'a>,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        mint: crate::objects::token::mint::Mint<'a>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;
}

pub trait NautilusCreateMint<'a> {
    fn create_mint<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_mint_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;
}

pub trait NautilusCreateMetadata<'a> {
    fn create_metadata<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: crate::objects::token::mint::Mint<'a>,
        mint_authority: T,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_metadata_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: crate::objects::token::mint::Mint<'a>,
        mint_authority: T,
        update_authority: T,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;
}

pub trait NautilusCreateToken<'a>: NautilusCreateMint<'a> + NautilusCreateMetadata<'a> {
    fn create<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_with_metadata<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult;

    fn create_with_metadata_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;
}

pub trait NautilusTransferToken<'a> {} // TODO
