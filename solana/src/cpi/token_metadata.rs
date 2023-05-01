//! Cross-Program-Invocations to the Metaplex Token Metadata Program.

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};

use crate::{Create, Metadata, NautilusAccountInfo, NautilusMut, NautilusSigner};

/// Creates a Metadata account with the Token Metadata Program.
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

/// Creates a MasterEdition account with the Token Metadata Program.
#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn create_master_edition_v3<'a>(
    token_metadata_program_id: &Pubkey,
    edition: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    metadata: impl NautilusAccountInfo<'a>,
    update_authority: impl NautilusSigner<'a>,
    mint_authority: impl NautilusSigner<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
    max_supply: Option<u64>,
) -> ProgramResult {
    invoke(
        &mpl_token_metadata::instruction::create_master_edition_v3(
            *token_metadata_program_id,
            *edition.key(),
            *mint.key(),
            *update_authority.key(),
            *mint_authority.key(),
            *metadata.key(),
            *payer.key(),
            max_supply,
        ),
        &[
            *edition.account_info(),
            *metadata.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *rent,
        ],
    )
}

/// Mints a new Edition from a MasterEdition.
#[allow(clippy::boxed_local)]
#[allow(clippy::too_many_arguments)]
pub fn mint_edition_from_master_edition<'a>(
    token_metadata_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    metadata: impl NautilusAccountInfo<'a>,
    edition: impl NautilusMut<'a>,
    master_edition: impl NautilusAccountInfo<'a>,
    master_edition_mint: impl NautilusAccountInfo<'a>,
    master_edition_metadata: impl NautilusAccountInfo<'a>,
    to: impl NautilusMut<'a>,
    to_authority: impl NautilusSigner<'a>,
    mint_authority: impl NautilusSigner<'a>,
    update_authority: impl NautilusSigner<'a>,
    payer: impl NautilusSigner<'a>,
    rent: Box<AccountInfo<'a>>,
    edition_val: u64,
) -> ProgramResult {
    invoke(
        &mpl_token_metadata::instruction::mint_new_edition_from_master_edition_via_token(
            *token_metadata_program_id,
            *metadata.key(),
            *edition.key(),
            *master_edition.key(),
            *mint.key(),
            *mint_authority.key(),
            *payer.key(),
            *to_authority.key(),
            *to.key(),
            *update_authority.key(),
            *master_edition_metadata.key(),
            *master_edition_mint.key(),
            edition_val,
        ),
        &[
            *metadata.account_info(),
            *edition.account_info(),
            *master_edition.account_info(),
            *mint.account_info(),
            *mint_authority.account_info(),
            *payer.account_info(),
            *to_authority.account_info(),
            *to.account_info(),
            *update_authority.account_info(),
            *master_edition_metadata.account_info(),
            *master_edition_mint.account_info(),
            *rent,
        ],
    )
}
