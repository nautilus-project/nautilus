//! Wallet tests
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn allocate<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        // /* Business Logic */
        //
        new_wallet.allocate()?;
        //
        print_wallet_details(&new_wallet, "Allocate");
        Ok(())
    }

    fn assign<'a>(wallet: Signer<Wallet<'a>>, new_owner: Pubkey) -> ProgramResult {
        // /* Business Logic */
        //
        wallet.assign(&new_owner)?;
        //
        print_wallet_details(&wallet, "Assign");
        Ok(())
    }

    fn create<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        // /* Business Logic */
        //
        new_wallet.create()?;
        //
        print_wallet_details(&new_wallet, "Create");
        Ok(())
    }

    fn create_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        // /* Business Logic */
        //
        new_wallet.create_with_payer(rent_payer)?;
        //
        print_wallet_details(&new_wallet, "Create w/ payer");
        print_wallet_details(&rent_payer, "Rent payer");
        Ok(())
    }

    fn read(wallet: Wallet) -> ProgramResult {
        // /* Business Logic */
        //
        print_wallet_details(&wallet, "Read");
        Ok(())
    }

    fn transfer<'a>(from: Signer<Wallet<'a>>, to: Mut<Wallet<'a>>, amount: u64) -> ProgramResult {
        //
        print_wallet_details(&from, "From acct pre-transfer");
        print_wallet_details(&to, "To acct pre-transfer");
        msg!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        // /* Business Logic */
        //
        from.transfer_lamports(to, amount)?;
        //
        print_wallet_details(&from, "From acct post-transfer");
        print_wallet_details(&to, "To acct post-transfer");
        Ok(())
    }
}

fn print_wallet_details(wallet: &impl NautilusAccountInfo, desc: &str) {
    msg!(" * Wallet info for: {}:", desc);
    msg!("      Address:    {}", wallet.key());
    msg!("      Owner:      {}", wallet.owner());
    msg!("      Size:       {}", wallet.size().unwrap());
    msg!("      Lamports:   {}", wallet.lamports());
    msg!("      Address:    {}", wallet.key());
    msg!("      Address:    {}", wallet.key());
}
