//! Cross-Program-Invocations to the System Program
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

use crate::{NautilusData, NautilusMut, NautilusRecord, NautilusSigner};

/// Allocate space for an account.
#[allow(clippy::boxed_local)]
pub fn allocate<'a>(new_account: impl NautilusSigner<'a>) -> ProgramResult {
    invoke(
        &system_instruction::allocate(new_account.key(), new_account.size()?),
        &[*new_account.account_info()],
    )
}

/// Assign ownership of an account from the system program.
#[allow(clippy::boxed_local)]
pub fn assign<'a>(new_account: impl NautilusSigner<'a>, owner: &Pubkey) -> ProgramResult {
    invoke(
        &system_instruction::assign(new_account.key(), owner),
        &[*new_account.account_info()],
    )
}

/// Create an account.
#[allow(clippy::boxed_local)]
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

/// Cross-Program-Invocation (CPI) to create a record.
///
/// This CPI is signed using the signer seeds of the record (PDA), and also
/// makes sure to serialized the provided data into the new account.
#[allow(clippy::boxed_local)]
pub fn create_record<'a, T: NautilusData>(
    new_account: impl NautilusRecord<'a>,
    owner: &Pubkey,
    payer: impl NautilusSigner<'a>,
    system_program: Box<AccountInfo<'a>>,
    data: Box<T>,
) -> ProgramResult {
    let (pda, bump) = new_account.pda();
    assert_eq!(
        &pda,
        new_account.key(),
        "Derived PDA does not match data for account {:#?}",
        new_account.key()
    );
    let seeds = new_account.seeds();
    let signer_seeds: [&[u8]; 3] = [seeds[0].as_slice(), seeds[1].as_slice(), &[bump]];
    invoke_signed(
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
        &[&signer_seeds],
    )?;
    let mut discriminator = [0u8; 8];
    let preimage = format!("{}:{}", "global", T::TABLE_NAME);
    // get first 8 bytes of hash
    discriminator.copy_from_slice(&solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    let account_info = new_account.account_info();
    let mut account_data = account_info.data.borrow_mut();
    // add discriminator to first 8 bytes of account data
    account_data[..8].copy_from_slice(&discriminator);
    // add account data
    data.serialize(&mut &mut account_data[8..])?;
    Ok(())
}

/// Transfer lamports from an account owned by the system program.
#[allow(clippy::boxed_local)]
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
