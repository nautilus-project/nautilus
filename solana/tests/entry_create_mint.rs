use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_mint(
        mut new_mint: Create<Mint>,
        decimals: u8,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))
    }
    fn create_mint_with_payer(
        mut new_mint: Create<Mint>,
        decimals: u8,
        mint_authority: Signer<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }
}

#[test]
fn entry_create_mint() {}
