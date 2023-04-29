//! Cross-Program-Invocations to the Metaplex Token Metadata Program.

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};

use crate::{Create, Metadata, NautilusAccountInfo, NautilusSigner};

#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn create_metadata_v3<'a>(
    token_metadata_program_id: &Pubkey,
    metadata: Create<'a, Metadata<'a>>,
    title: String,
    symbol: String,
    uri: String,
    mint: impl NautilusAccountInfo<'a>,
    mint_authority: impl NautilusSigner<'a>,
    update_authority: impl NautilusAccountInfo<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &mpl_token_metadata::instruction::create_metadata_accounts_v3(
            *token_metadata_program_id,
            *metadata.key(),
            *mint.key(),
            *mint_authority.key(),
            *payer.key(),
            *update_authority.key(),
            title,
            symbol,
            uri,
            None,
            0,
            true,
            false,
            None,
            None,
            None,
        ),
        &[
            *metadata.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *rent,
        ],
    )
}
