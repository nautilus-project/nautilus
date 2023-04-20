use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};

pub fn transfer_lamports<'a>(
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
    accounts: &[AccountInfo],
) -> ProgramResult {
    invoke(
        &solana_program::system_instruction::transfer(from, to, amount),
        accounts,
    )
}
