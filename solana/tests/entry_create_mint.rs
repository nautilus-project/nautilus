use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_mint<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))
    }
    fn create_mint_with_payer<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
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
