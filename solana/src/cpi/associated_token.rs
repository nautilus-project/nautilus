//! Cross-Program invocations to the Associated Token Program
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};

use crate::{AssociatedTokenAccount, NautilusAccountInfo, NautilusSigner};

/// Creates an associated token account.
#[allow(clippy::boxed_local)]
pub fn create_associated_token_account<'a>(
    new_account: AssociatedTokenAccount<'a>,
    owner: impl NautilusAccountInfo<'a>,
    mint: impl NautilusAccountInfo<'a>,
    payer: impl NautilusSigner<'a>,
    system_program: Box<AccountInfo<'a>>,
    token_program: Box<AccountInfo<'a>>,
    associated_token_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer.key(),
            owner.key(),
            mint.key(),
            token_program.key,
        ),
        &[
            *mint.account_info(),
            *new_account.account_info(),
            *owner.account_info(),
            *payer.account_info(),
            *system_program,
            *token_program,
            *associated_token_program,
        ],
    )
}

/// Recover nested associated token account.
#[allow(clippy::boxed_local)]
pub fn recover_nested<'a>(
    wallet: impl NautilusAccountInfo<'a>,
    owner_mint: impl NautilusAccountInfo<'a>,
    nested_mint: impl NautilusAccountInfo<'a>,
    token_program: Box<AccountInfo<'a>>,
    associated_token_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &spl_associated_token_account::instruction::recover_nested(
            wallet.key(),
            owner_mint.key(),
            nested_mint.key(),
            token_program.key,
        ),
        &[
            *wallet.account_info(),
            *nested_mint.account_info(),
            *owner_mint.account_info(),
            *token_program,
            *associated_token_program,
        ],
    )
}
