use nautilus::*;

#[nautilus]
mod program_nautilus {

    // Wallets

    fn create_wallet<'a>(mut new_wallet: Create<'a, Wallet<'a>>) -> ProgramResult {
        msg!("-- New Wallet Public Key: {}", &new_wallet.key());
        new_wallet.create()
    }

    fn read_wallet(new_wallet: Wallet) -> ProgramResult {
        msg!("-- New Wallet Public Key: {}", &new_wallet.key());
        Ok(())
    }

    fn create_wallet_with_payer<'a>(
        mut new_wallet: Create<'a, Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Wallet Public Key: {}", &new_wallet.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_wallet.create_with_payer(rent_payer)
    }

    fn read_wallet_created_with_payer(new_wallet: Wallet) -> ProgramResult {
        msg!("-- New Wallet Public Key: {}", &new_wallet.key());
        Ok(())
    }

    // Mints

    fn create_mint<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Mint Public Key: {}", &new_mint.key());
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority), None)
    }

    fn read_mint(new_mint: Mint) -> ProgramResult {
        msg!("-- New Mint Public Key: {}", &new_mint.key());
        print_mint_data(new_mint.data);
        Ok(())
    }

    fn create_mint_with_payer<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Mint Public Key: {}", &new_mint.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            None,
            rent_payer,
        )
    }

    fn read_mint_created_with_payer(new_mint: Mint) -> ProgramResult {
        msg!("-- New Mint Public Key: {}", &new_mint.key());
        print_mint_data(new_mint.data);
        Ok(())
    }

    // Metadatas

    fn create_metadata<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Metadata Public Key: {}", &new_metadata.key());
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

    fn read_metadata(new_metadata: Metadata) -> ProgramResult {
        msg!("-- New Metadata Public Key: {}", &new_metadata.key());
        print_metadata_data(new_metadata.data);
        Ok(())
    }

    fn create_metadata_with_payer<'a>(
        mut new_metadata: Create<'a, Metadata<'a>>,
        mint: Mint<'a>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Metadata Public Key: {}", &new_metadata.key());
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

    fn read_metadata_created_with_payer(new_metadata: Metadata) -> ProgramResult {
        msg!("-- New Metadata Public Key: {}", &new_metadata.key());
        print_metadata_data(new_metadata.data);
        Ok(())
    }

    // Associated Token Accounts

    fn create_associated_token<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
    ) -> ProgramResult {
        msg!(
            "-- New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        new_associated_token.create(mint, owner)
    }

    fn read_associated_token(new_associated_token: AssociatedTokenAccount) -> ProgramResult {
        msg!(
            "-- New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        print_associated_token_data(new_associated_token.data);
        Ok(())
    }

    fn create_associated_token_with_payer<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!(
            "-- New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        msg!("-- Mint Public Key: {}", &mint.key());
        msg!("-- Owner Public Key: {}", &owner.key());
        msg!("-- Rent Payer Public Key: {}", &rent_payer.key());
        new_associated_token.create_with_payer(mint, owner, rent_payer)
    }

    fn read_associated_token_created_with_payer(
        new_associated_token: AssociatedTokenAccount,
    ) -> ProgramResult {
        msg!(
            "-- New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        print_associated_token_data(new_associated_token.data);
        Ok(())
    }

    // Tokens

    fn create_token<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        msg!("-- New Token Public Key: {}", &new_token.key());
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

    fn read_token(new_token: Token) -> ProgramResult {
        msg!("-- New Token Public Key: {}", &new_token.key());
        print_mint_data(new_token.mint.data);
        print_metadata_data(new_token.metadata.data);
        Ok(())
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
        msg!("-- New Token Public Key: {}", &new_token.key());
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

    fn read_token_created_with_payer(new_token: Token) -> ProgramResult {
        msg!("-- New Token Public Key: {}", &new_token.key());
        print_mint_data(new_token.mint.data);
        print_metadata_data(new_token.metadata.data);
        Ok(())
    }

    // Transfers

    fn transfer_wallet<'a>(
        from: Signer<Wallet<'a>>,
        to: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        msg!(
            "Transferring {} From: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        from.transfer_lamports(to, amount)
    }
}

fn print_mint_data(data: MintState) {
    msg!("-- Mint Data:");
    msg!("  Mint Authority:         {:#?}", data.mint_authority);
    msg!("  Supply:                 {}", data.supply);
    msg!("  Decimals:               {}", data.decimals);
    msg!("  Is Initialized:         {}", data.is_initialized);
    msg!("  Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: MetadataState) {
    msg!("-- Metadata Data:");
    msg!("  Mint:                   {:#?}", data.mint);
    msg!("  Primary Sale Happened:  {}", data.primary_sale_happened);
    msg!("  Is Mutable:             {}", data.is_mutable);
    msg!("  Edition Nonce:          {:#?}", data.edition_nonce);
    msg!("  Title:                  {}", data.data.name);
    msg!("  Symbol:                 {}", data.data.symbol);
    msg!("  URI:                    {}", data.data.uri);
}

fn print_associated_token_data(data: AssociatedTokenAccountState) {
    msg!("-- Associated Token Data:");
    msg!("  Mint:                   {:#?}", data.mint);
    msg!("  Owner:                  {:#?}", data.owner);
    msg!("  Amount:                 {}", data.amount);
    msg!("  Delegate:               {:#?}", data.delegate);
    msg!("  Is Native:              {:#?}", data.is_native);
    msg!("  Delegated Amount:       {}", data.delegated_amount);
    msg!("  Close Authority:        {:#?}", data.close_authority);
}
