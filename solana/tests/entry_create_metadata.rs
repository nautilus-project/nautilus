use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn create_metadata(
        mut new_metadata: Create<Metadata>,
        mint: Mint,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        new_metadata.create(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
        )
    }
    fn create_metadata_with_payer(
        mut new_metadata: Create<Metadata>,
        mint: Mint,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        new_metadata.create_with_payer(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
            rent_payer,
        )
    }
}

#[test]
fn entry_create_metadata() {}
