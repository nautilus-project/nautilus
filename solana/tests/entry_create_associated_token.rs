use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_associated_token<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
    ) -> ProgramResult {
        new_associated_token.create(mint, owner)
    }
    fn create_associated_token_with_payer<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_associated_token.create_with_payer(mint, owner, rent_payer)
    }
}

#[test]
fn entry_create_associated_token() {}
