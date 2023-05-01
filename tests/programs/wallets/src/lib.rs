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
        splog_info!(
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

    /// A simluated "complex" program instruction to test Nautilus.
    /// The logic herein is just for example.
    fn complex<'a>(
        _authority1: Signer<Wallet<'a>>, // Marking this as `Signer` will ensure it's a signer on the tx.
        authority2: Signer<Wallet<'a>>,
        rent_payer1: Signer<Wallet<'a>>,
        rent_payer2: Signer<Wallet<'a>>,
        wallet_to_allocate: Create<'a, Wallet<'a>>, // Marking this as `Create` will ensure it hasn't been created.
        mut wallet_to_create: Create<'a, Wallet<'a>>,
        wallet_to_create_with_transfer_safe: Create<'a, Wallet<'a>>,
        wallet_to_create_with_transfer_unsafe: Mut<Wallet<'a>>,
        some_other_transfer_recipient: Mut<Wallet<'a>>,
        amount_to_fund: u64,
        amount_to_transfer: u64,
    ) -> ProgramResult {
        //
        // /* Business Logic */
        //

        // Some random checks to simulate how custom checks might look.
        assert!(rent_payer1
            .owner()
            .eq(&nautilus::solana_program::system_program::ID));
        assert!(rent_payer2
            .owner()
            .eq(&nautilus::solana_program::system_program::ID));

        // Even though the check will be applied via `Signer` in the function sig, you can still
        // check yourself if you choose to.
        assert!(authority2.is_signer());

        // Even though the check will be applied via `Create` in the function sig, you can still
        // check yourself if you choose to.
        assert!(wallet_to_allocate.lamports() == 0);
        assert!(wallet_to_allocate.is_writable());
        wallet_to_allocate.allocate()?;

        assert!(wallet_to_create.lamports() == 0);
        assert!(wallet_to_create.is_writable());
        wallet_to_create.create()?;

        // Safe - checked at entry with `Create`.
        rent_payer1.transfer_lamports(wallet_to_create_with_transfer_safe, amount_to_fund)?;

        // Unsafe - not marked with `Create`.
        rent_payer2.transfer_lamports(wallet_to_create_with_transfer_unsafe, amount_to_fund)?;

        // Transfer with balance assertions
        let from_beg_balance = authority2.lamports();
        let to_beg_balance = some_other_transfer_recipient.lamports();
        authority2.transfer_lamports(some_other_transfer_recipient.clone(), amount_to_transfer)?;
        let from_end_balance = authority2.lamports();
        let to_end_balance = some_other_transfer_recipient.lamports();
        assert!(from_beg_balance - from_end_balance == amount_to_transfer);
        assert!(to_end_balance - to_beg_balance == amount_to_transfer);
        //
        Ok(())
    }
}

fn print_wallet_details<'a>(wallet: &impl NautilusAccountInfo<'a>, desc: &str) {
    splog_info!(" * Wallet info for: {}:", desc);
    splog_info!("      Address:    {}", wallet.key());
    splog_info!("      Owner:      {}", wallet.owner());
    splog_info!("      Size:       {}", wallet.size().unwrap());
    splog_info!("      Lamports:   {}", wallet.lamports());
}
