use solana_program::entrypoint::ProgramResult;

use super::signer::NautilusSigner;

pub trait NautilusUpdate<'a> {
    fn update(&self) -> ProgramResult;
    fn update_with_payer(&self, payer: impl NautilusSigner<'a>) -> ProgramResult;
}

pub trait NautilusUpdateMetadata<'a> {
    fn update_metadata(
        &self,
        title: Option<String>,
        symbol: Option<String>,
        uri: Option<String>,
        update_authority: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}

pub trait NautilusUpdateToken<'a> {
    fn update_metadata(
        &self,
        title: Option<String>,
        symbol: Option<String>,
        uri: Option<String>,
        update_authority: impl NautilusSigner<'a>,
    ) -> ProgramResult;
}
