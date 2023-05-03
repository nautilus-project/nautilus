//! Cross-Program invocations to the System Program
use borsh::BorshSerialize;
use solana_program::{
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

use crate::{NautilusAccountInfo, NautilusMut, NautilusSigner};

/// Allocate space for an account.
pub fn allocate<'a>(new_account: impl NautilusSigner<'a>) -> ProgramResult {
    invoke(
        &system_instruction::allocate(new_account.key(), new_account.size()?),
        &[*new_account.account_info()],
    )
}

/// Assign ownership of an account from the system program.
pub fn assign<'a>(new_account: impl NautilusSigner<'a>, owner: &Pubkey) -> ProgramResult {
    invoke(
        &system_instruction::assign(new_account.key(), owner),
        &[*new_account.account_info()],
    )
}

/// Create an account.
pub fn create_account<'a>(
    new_account: impl NautilusSigner<'a>,
    owner: &Pubkey,
    payer: impl NautilusSigner<'a>,
) -> ProgramResult {
    invoke(
        &system_instruction::create_account(
            payer.key(),
            new_account.key(),
            new_account.required_rent()?,
            new_account.size()?,
            owner,
        ),
        &[*payer.account_info(), *new_account.account_info()],
    )
}

/// Cross-Program Invocation (CPI) to create a program-derived address account (PDA).
#[allow(clippy::boxed_local)]
pub fn create_pda<'a, T: BorshSerialize>(
    new_account: impl NautilusAccountInfo<'a>,
    owner: &Pubkey,
    payer: impl NautilusSigner<'a>,
    data: Box<T>,
    signer_seeds: Vec<&[u8]>,
) -> ProgramResult {
    invoke_signed(
        &system_instruction::create_account(
            payer.key(),
            new_account.key(),
            new_account.required_rent()?,
            new_account.size()?,
            owner,
        ),
        &[*payer.account_info(), *new_account.account_info()],
        &[&signer_seeds],
    )?;
    data.serialize(&mut &mut new_account.account_info().data.borrow_mut()[..])?;
    Ok(())
}

/// Transfer lamports from an account owned by the system program.
pub fn transfer<'a>(
    from: impl NautilusSigner<'a>,
    to: impl NautilusMut<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &solana_program::system_instruction::transfer(from.key(), to.key(), amount),
        &[*from.account_info(), *to.account_info()],
    )
}
