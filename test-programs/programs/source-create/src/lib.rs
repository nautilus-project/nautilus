use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn create_wallet(new_wallet: Create<Wallet>) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_wallet.key());
        new_wallet.create()
    }
    fn create_wallet_with_payer(
        new_wallet: Create<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_wallet.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_wallet.create_with_payer(rent_payer)
    }

    fn create_mint(
        new_mint: Create<Mint>,
        decimals: u8,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_mint.key());
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))
    }
    fn create_mint_with_payer(
        new_mint: Create<Mint>,
        decimals: u8,
        mint_authority: Signer<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_mint.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )
    }

    fn create_metadata(
        new_metadata: Create<Metadata>,
        mint: Mint,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_metadata.key());
        msg!("-- Mint Public Key: {}", &mint.key());
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
        new_metadata: Create<Metadata>,
        mint: Mint,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_metadata.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
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

    fn create_associated_token(
        new_associated_token: Create<AssociatedTokenAccount>,
        mint: Mint,
        owner: Wallet,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_associated_token.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        new_associated_token.create(mint, owner)
    }
    fn create_associated_token_with_payer(
        new_associated_token: Create<AssociatedTokenAccount>,
        mint: Mint,
        owner: Wallet,
        rent_payer: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_associated_token.key());
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_associated_token.create_with_payer(mint, owner, rent_payer)
    }

    fn create_token(
        new_token: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- New Account Public Key: {}", &new_token.key());
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
        msg!("-- New Account Public Key: {}", &new_token.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
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

    fn transfer_wallet(from: Signer<Wallet>, to: Mut<Wallet>, amount: u64) -> ProgramResult {
        msg!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        from.transfer_lamports(to, amount)
    }
}
