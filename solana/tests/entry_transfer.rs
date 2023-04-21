use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn transfer_wallet<'a>(
        from: Signer<Wallet<'a>>,
        to: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        println!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        from.transfer_lamports(to, amount)
    }
}

#[test]
fn test_transfer() {}
