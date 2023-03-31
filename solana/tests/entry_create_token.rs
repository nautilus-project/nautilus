use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_token(
        new_token: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
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
    fn create_token_with_payer(
        new_token: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
        rent_payer: Signer<Wallet>,
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
