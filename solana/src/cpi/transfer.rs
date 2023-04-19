use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};

use crate::{NautilusMut, NautilusSigner};

pub fn transfer_lamports<'a>(
    from: impl NautilusSigner,
    to: impl NautilusMut,
    amount: u64,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &solana_program::system_instruction::transfer(from.key(), to.key(), amount),
        &[from.into(), to.into(), *system_program],
    )
}
