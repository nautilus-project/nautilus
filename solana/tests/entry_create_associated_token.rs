use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_associated_token(
        mut new_associated_token: Create<AssociatedTokenAccount>,
        mint: Mint,
        owner: Wallet,
    ) -> ProgramResult {
        new_associated_token.create(mint, owner)
    }
    fn create_associated_token_with_payer(
        mut new_associated_token: Create<AssociatedTokenAccount>,
        mint: Mint,
        owner: Wallet,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        new_associated_token.create_with_payer(mint, owner, rent_payer)
    }
}

#[test]
fn entry_create_associated_token() {}
