//! Wallet tests
use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn allocate<'a>(new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        print_wallet_details(&new_wallet, "Allocate acct pre-allocate");
        //
        // /* Business Logic */
        //
        new_wallet.allocate()?;
        //
        print_wallet_details(&new_wallet, "Allocate acct post-allocate");
        Ok(())
    }

    fn assign<'a>(wallet: Signer<Wallet<'a>>, new_owner: Pubkey) -> ProgramResult {
        print_wallet_details(&wallet, "Assign acct pre-assign");
        //
        // /* Business Logic */
        //
        wallet.assign(new_owner)?;
        //
        print_wallet_details(&wallet, "Assign acct post-assign");
        Ok(())
    }

    fn create<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        print_wallet_details(&new_wallet, "Create acct pre-create");
        //
        // /* Business Logic */
        //
        new_wallet.create()?;
        //
        print_wallet_details(&new_wallet, "Create acct post-create");
        Ok(())
    }

    fn create_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        print_wallet_details(&new_wallet, "Create acct pre-create");
        print_wallet_details(&rent_payer, "Rent payer pre-create");
        //
        // /* Business Logic */
        //
        new_wallet.create_with_payer(rent_payer.clone())?; // Cloning so we can ref later
                                                           //
        print_wallet_details(&new_wallet, "Create acct post-create");
        print_wallet_details(&rent_payer, "Rent payer post-create");
        Ok(())
    }

    fn read(wallet: Wallet) -> ProgramResult {
        print_wallet_details(&wallet, "Read");
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn transfer<'a>(from: Signer<Wallet<'a>>, to: Mut<Wallet<'a>>, amount: u64) -> ProgramResult {
        print_wallet_details(&from, "From acct pre-transfer");
        print_wallet_details(&to, "To acct pre-transfer");
        msg!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        //
        // /* Business Logic */
        //
        from.transfer_lamports(to.clone(), amount)?; // Cloning so we can ref later
                                                     //
        print_wallet_details(&from, "From acct post-transfer");
        print_wallet_details(&to, "To acct post-transfer");
        Ok(())
    }
}

fn print_wallet_details<'a>(wallet: &impl NautilusAccountInfo<'a>, desc: &str) {
    msg!(" * Wallet info for: {}:", desc);
    msg!("      Address:    {}", wallet.key());
    msg!("      Owner:      {}", wallet.owner());
    msg!("      Size:       {}", wallet.size().unwrap());
    msg!("      Lamports:   {}", wallet.lamports());
}
