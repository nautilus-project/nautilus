use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke};

use crate::{NautilusMut, NautilusSigner};

/// Cross-Program-Invocation (CPI) to conduct a transfer via the System Program.
#[allow(clippy::boxed_local)]
pub fn transfer_lamports<'a>(
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
