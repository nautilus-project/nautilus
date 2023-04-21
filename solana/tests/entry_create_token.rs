use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_token<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )
    }
    fn create_token_with_payer<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        new_token.create_with_payer(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }
}

#[test]
fn entry_create_token() {}
