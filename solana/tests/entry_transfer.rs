use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn transfer_wallet(from: Signer<Wallet>, to: Mut<Wallet>, amount: u64) -> ProgramResult {
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
