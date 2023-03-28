use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn test_wallets(from: Wallet, to: Wallet) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
    fn test_mints(from: Mint, to: Mint) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
    fn test_metadatas(from: Metadata, to: Metadata) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
    fn test_associated_tokens(
        from: AssociatedTokenAccount,
        to: AssociatedTokenAccount,
    ) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
    fn test_tokens(from: Token, to: Token) -> ProgramResult {
        msg!("{}", from.key());
        msg!("{}", to.key());
        Ok(())
    }
}
