use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn wallet_test(from: Wallet, to: Wallet) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
    fn wallet_test_2(from: Wallet, to: Wallet) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
}
