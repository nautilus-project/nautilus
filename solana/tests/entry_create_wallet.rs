use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_wallet(new_wallet: Create<Wallet>) -> ProgramResult {
        new_wallet.create()
    }
    fn create_wallet_with_payer(
        new_wallet: Create<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        new_wallet.create_with_payer(rent_payer)
    }
}

#[test]
fn entry_create_wallet() {}
