use nautilus::spl_token::instruction::AuthorityType;
use nautilus::splogger::{info, Splog};
use nautilus::*;

#[nautilus]
mod program_nautilus {

    // Mints

    fn create_mint<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * New Mint Public Key: {}", &new_mint.key());
        //
        // /* Business Logic */
        //
        new_mint.create(decimals, mint_authority.clone(), Some(mint_authority))?;
        //
        print_mint_data(&new_mint.self_account.data, "Create");
        Ok(())
    }

    fn create_mint_with_payer<'a>(
        mut new_mint: Create<'a, Mint<'a>>,
        decimals: u8,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * New Mint Public Key: {}", &new_mint.key());
        info!(" * Rent Payer Public Key: {}", &rent_payer.key());
        //
        // /* Business Logic */
        //
        new_mint.create_with_payer(
            decimals,
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )?;
        //
        print_mint_data(&new_mint.self_account.data, "Create with payer");
        Ok(())
    }

    fn mint_mint_to<'a>(
        mint: Mut<Mint<'a>>,
        to: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        print_associated_token_data(&to.self_account.data, "To acct pre-mint");
        info!(" * Mint Public Key: {}", &mint.key());
        print_mint_data(&mint.self_account.data, "MintTo");
        info!("Minting {} tokens to: {}", amount, to.key());
        //
        // /* Business Logic */
        //
        mint.mint_to(to.clone(), authority, amount)?; // Cloning so we can ref later
                                                      //
        print_associated_token_data(&to.self_account.data, "To acct post-mint");
        Ok(())
    }

    fn mint_disable_minting<'a>(
        mint: Mut<Mint<'a>>,
        authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * Mint Public Key: {}", &mint.key());
        print_mint_data(&mint.self_account.data, "Mint pre-disabling");
        //
        // /* Business Logic */
        //
        mint.set_authority(None, AuthorityType::MintTokens, authority)?;
        //
        print_mint_data(&mint.self_account.data, "Mint post-disabling");
        Ok(())
    }

    fn read_mint(mint: Mint) -> ProgramResult {
        info!(" * Mint Public Key: {}", &mint.key());
        print_mint_data(&mint.data, "Read");
        //
        // /* Business Logic */
        //
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
        info!(" * New Metadata Public Key: {}", &new_metadata.key());
        info!(" * Mint Public Key: {}", &mint.key());
        //
        // /* Business Logic */
        //
        new_metadata.create(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
        )?;
        //
        print_metadata_data(&new_metadata.self_account.data, "Create");
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
        info!(" * New Metadata Public Key: {}", &new_metadata.key());
        info!(" * Mint Public Key: {}", &mint.key());
        info!(" * Rent Payer Public Key: {}", &rent_payer.key());
        //
        // /* Business Logic */
        //
        new_metadata.create_with_payer(
            title,
            symbol,
            uri,
            mint,
            mint_authority.clone(),
            mint_authority,
            rent_payer,
        )?;
        //
        print_metadata_data(&new_metadata.self_account.data, "Create with payer");
        Ok(())
    }

    fn read_metadata(metadata: Metadata) -> ProgramResult {
        info!(" * Metadata Public Key: {}", &metadata.key());
        print_metadata_data(&metadata.data, "Read");
        //
        // /* Business Logic */
        //
        Ok(())
    }

    // Associated Token Accounts

    fn create_associated_token<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
    ) -> ProgramResult {
        info!(
            " * New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        info!(" * Mint Public Key: {}", &mint.key());
        info!(" * Owner Public Key: {}", &owner.key());
        //
        // /* Business Logic */
        //
        new_associated_token.create(mint, owner)?;
        //
        print_associated_token_data(&new_associated_token.self_account.data, "Create");
        Ok(())
    }

    fn create_associated_token_with_payer<'a>(
        mut new_associated_token: Create<'a, AssociatedTokenAccount<'a>>,
        mint: Mint<'a>,
        owner: Wallet<'a>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(
            " * New AssociatedTokenAccount Public Key: {}",
            &new_associated_token.key()
        );
        info!(" * Mint Public Key: {}", &mint.key());
        info!(" * Owner Public Key: {}", &owner.key());
        info!(" * Rent Payer Public Key: {}", &rent_payer.key());
        //
        // /* Business Logic */
        //
        new_associated_token.create_with_payer(mint, owner, rent_payer)?;
        //
        print_associated_token_data(&new_associated_token.self_account.data, "Create with payer");
        Ok(())
    }

    fn read_associated_token(associated_token: AssociatedTokenAccount) -> ProgramResult {
        info!(
            " * AssociatedTokenAccount Public Key: {}",
            &associated_token.key()
        );
        print_associated_token_data(&associated_token.data, "Read");
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn burn_tokens<'a>(
        mint: Mint<'a>,
        from: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        print_associated_token_data(&from.self_account.data, "From acct pre-burn");
        info!("Burning {} tokens from: {} ", amount, from.key(),);
        //
        // /* Business Logic */
        //
        from.burn(mint, authority, amount)?; // Cloning so we can ref later
                                             //
        print_associated_token_data(&from.self_account.data, "From acct post-burn");
        Ok(())
    }

    fn transfer_tokens<'a>(
        from: Mut<AssociatedTokenAccount<'a>>,
        to: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        print_associated_token_data(&from.self_account.data, "From acct pre-transfer");
        print_associated_token_data(&to.self_account.data, "To acct pre-transfer");
        info!(
            "Transferring {} tokens from: {} to: {}",
            amount,
            from.key(),
            to.key()
        );
        //
        // /* Business Logic */
        //
        from.transfer(to.clone(), authority, amount)?; // Cloning so we can ref later
                                                       //
        print_associated_token_data(&from.self_account.data, "From acct post-transfer");
        print_associated_token_data(&to.self_account.data, "To acct post-transfer");
        Ok(())
    }

    fn freeze_account<'a>(
        mint: Mint<'a>,
        account: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * AssociatedTokenAccount Public Key: {}", &account.key());
        print_associated_token_data(&account.self_account.data, "Freeze (pre)");
        //
        // /* Business Logic */
        //
        account.freeze(mint, authority)?; // Cloning so we can ref later
                                          //
        print_associated_token_data(&account.self_account.data, "Freeze (post)");
        Ok(())
    }

    fn thaw_account<'a>(
        mint: Mint<'a>,
        account: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * AssociatedTokenAccount Public Key: {}", &account.key());
        print_associated_token_data(&account.self_account.data, "Thaw (pre)");
        //
        // /* Business Logic */
        //
        account.thaw(mint, authority)?; // Cloning so we can ref later
                                        //
        print_associated_token_data(&account.self_account.data, "Thaw (post)");
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
        info!(" * New Token Public Key: {}", &new_token.key());
        //
        // /* Business Logic */
        //
        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )?;
        //
        print_mint_data(&new_token.self_account.mint.data, "Create");
        print_metadata_data(&new_token.self_account.metadata.data, "Create");
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
        info!(" * New Token Public Key: {}", &new_token.key());
        info!(" * Rent Payer Public Key: {}", &rent_payer.key());
        //
        // /* Business Logic */
        //
        new_token.create_with_payer(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )?;
        //
        print_mint_data(&new_token.self_account.mint.data, "Create with payer");
        print_metadata_data(&new_token.self_account.metadata.data, "Create with payer");
        Ok(())
    }

    fn token_mint_to<'a>(
        token: Mut<Token<'a>>,
        to: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        print_associated_token_data(&to.self_account.data, "To acct pre-mint");
        info!(" * Token Public Key: {}", &token.key());
        print_mint_data(&token.self_account.mint.data, "MintTo");
        info!("Minting {} tokens to: {}", amount, to.key());
        //
        // /* Business Logic */
        //
        token.mint_to(to.clone(), authority, amount)?; // Cloning so we can ref later
                                                       //
        print_associated_token_data(&to.self_account.data, "To acct post-mint");
        Ok(())
    }

    fn token_disable_minting<'a>(
        token: Mut<Token<'a>>,
        authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * Mint Public Key: {}", &token.key());
        print_mint_data(&token.self_account.mint.data, "Token mint pre-disabling");
        //
        // /* Business Logic */
        //
        token.set_authority(None, AuthorityType::MintTokens, authority)?;
        //
        print_mint_data(&token.self_account.mint.data, "token mint post-disabling");
        Ok(())
    }

    fn read_token(token: Token) -> ProgramResult {
        info!(" * Token Public Key: {}", &token.key());
        print_mint_data(&token.mint.data, "Read");
        print_metadata_data(&token.metadata.data, "Read");
        //
        // /* Business Logic */
        //
        Ok(())
    }

    // NFTs

    fn create_nft<'a>(
        mut new_nft: Create<'a, Nft<'a>>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * New NFT Public Key: {}", &new_nft.key());
        //
        // /* Business Logic */
        //
        new_nft.create(
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
        )?;
        //
        print_mint_data(&new_nft.self_account.mint.data, "Create");
        print_metadata_data(&new_nft.self_account.metadata.data, "Create");
        Ok(())
    }

    fn create_nft_with_payer<'a>(
        mut new_nft: Create<'a, Nft<'a>>,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        rent_payer: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        info!(" * New NFT Public Key: {}", &new_nft.key());
        info!(" * Rent Payer Public Key: {}", &rent_payer.key());
        //
        // /* Business Logic */
        //
        new_nft.create_with_payer(
            title,
            symbol,
            uri,
            mint_authority.clone(),
            mint_authority.clone(),
            Some(mint_authority),
            rent_payer,
        )?;
        //
        print_mint_data(&new_nft.self_account.mint.data, "Create with payer");
        print_metadata_data(&new_nft.self_account.metadata.data, "Create with payer");
        Ok(())
    }

    fn nft_mint_to<'a>(
        nft: Mut<Nft<'a>>,
        to: Mut<AssociatedTokenAccount<'a>>,
        authority: Signer<Wallet<'a>>,
    ) -> ProgramResult {
        print_associated_token_data(&to.self_account.data, "To acct pre-mint");
        info!(" * NFT Public Key: {}", &nft.key());
        print_mint_data(&nft.self_account.mint.data, "MintTo");
        info!("Minting NFT to: {}", to.key());
        //
        // /* Business Logic */
        //
        nft.mint_to(to.clone(), authority)?; // Cloning so we can ref later
                                             //
        print_associated_token_data(&to.self_account.data, "To acct post-mint");
        Ok(())
    }

    fn read_nft(nft: Nft) -> ProgramResult {
        info!(" * NFT Public Key: {}", &nft.key());
        print_mint_data(&nft.mint.data, "Read");
        print_metadata_data(&nft.metadata.data, "Read");
        //
        // /* Business Logic */
        //
        Ok(())
    }
}

fn print_mint_data(data: &MintState, desc: &str) {
    info!(" * Mint Data for: {}:", desc);
    info!("      Mint Authority:         {:#?}", data.mint_authority);
    info!("      Supply:                 {}", data.supply);
    info!("      Decimals:               {}", data.decimals);
    info!("      Is Initialized:         {}", data.is_initialized);
    info!("      Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState, desc: &str) {
    info!(" * Metadata Data for: {}:", desc);
    info!("      Mint:                   {:#?}", data.mint);
    info!(
        "      Primary Sale Happened:  {}",
        data.primary_sale_happened
    );
    info!("      Is Mutable:             {}", data.is_mutable);
    info!("      Edition Nonce:          {:#?}", data.edition_nonce);
    info!("      Title:                  {}", data.data.name);
    info!("      Symbol:                 {}", data.data.symbol);
    info!("      URI:                    {}", data.data.uri);
}

fn print_associated_token_data(data: &AssociatedTokenAccountState, desc: &str) {
    info!(" * Associated Token Data for: {}:", desc);
    info!("      Mint:                   {:#?}", data.mint);
    info!("      Owner:                  {:#?}", data.owner);
    info!("      Amount:                 {}", data.amount);
    info!("      Delegate:               {:#?}", data.delegate);
    info!("      Is Native:              {:#?}", data.is_native);
    info!("      Delegated Amount:       {}", data.delegated_amount);
    info!("      Close Authority:        {:#?}", data.close_authority);
}
