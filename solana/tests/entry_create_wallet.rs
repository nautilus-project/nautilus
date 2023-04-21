use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_wallet<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        new_wallet.create()
    }
    fn create_wallet_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_wallet.create_with_payer(rent_payer)
    }
}

#[test]
fn entry_create_wallet() {}
