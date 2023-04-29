//! Cross-Program-Invocations to the System Program
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
    system_instruction,
};

use crate::{NautilusMut, NautilusSigner};

/// Allocate space for an account.
#[allow(clippy::boxed_local)]
pub fn allocate<'a>(
    new_account: impl NautilusSigner<'a>,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &system_instruction::allocate(new_account.key(), new_account.size()?),
        &[*new_account.account_info(), *system_program],
    )
}

/// Assign ownership of an account from the system program.
#[allow(clippy::boxed_local)]
pub fn assign<'a>(
    new_account: impl NautilusSigner<'a>,
    owner: &Pubkey,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &system_instruction::assign(new_account.key(), owner),
        &[*new_account.account_info(), *system_program],
    )
}

/// Create an account.
#[allow(clippy::boxed_local)]
pub fn create_account<'a>(
    new_account: impl NautilusSigner<'a>,
    owner: &Pubkey,
    payer: impl NautilusSigner<'a>,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &system_instruction::create_account(
            payer.key(),
            new_account.key(),
            new_account.required_rent()?,
            new_account.size()?,
            owner,
        ),
        &[
            *payer.account_info(),
            *new_account.account_info(),
            *system_program,
        ],
    )
}

/// Transfer lamports from an account owned by the system program.
#[allow(clippy::boxed_local)]
pub fn transfer<'a>(
    from: impl NautilusSigner<'a>,
    to: impl NautilusMut<'a>,
    amount: u64,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &solana_program::system_instruction::transfer(from.key(), to.key(), amount),
        &[*from.account_info(), *to.account_info(), *system_program],
    )
}
