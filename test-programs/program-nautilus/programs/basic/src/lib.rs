use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn test_wallets(from: Wallet, to: Wallet) -> ProgramResult {
        test_output("From", from);
        test_output("To", to);
        Ok(())
    }
    fn test_mints(from: Mint, to: Mint) -> ProgramResult {
        test_output("From", from);
        test_output("To", to);
        Ok(())
    }
    fn test_metadatas(from: Metadata, to: Metadata) -> ProgramResult {
        test_output("From", from);
        test_output("To", to);
        Ok(())
    }
    fn test_associated_tokens(
        from: AssociatedTokenAccount,
        to: AssociatedTokenAccount,
    ) -> ProgramResult {
        test_output("From", from);
        test_output("To", to);
        Ok(())
    }
    fn test_tokens(from: Token, to: Token) -> ProgramResult {
        test_output("From", from);
        test_output("To", to);
        Ok(())
    }
}

fn test_output<'a, T: NautilusAccountInfo<'a>>(name: &str, object: T) {
    msg!("{} Key:           {}", name, object.key());
    msg!("{} Is Signer:     {}", name, object.is_signer());
    msg!("{} Is Writable:   {}", name, object.is_writable());
    msg!("{} Lamports:      {}", name, object.lamports());
    msg!("{} Owner:         {}", name, object.owner());
    msg!("{} Span:          {}", name, object.span());
}
