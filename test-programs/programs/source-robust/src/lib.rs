use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn create_token(
        new_token: Create<Token>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet>,
        update_authority: Signer<Wallet>,
    ) -> ProgramResult {
        msg!("-- Token: {}", &new_token.key());
        msg!("-- Metadata: {}", &new_token.metadata().key());
        msg!("-- Update Authority: {}", &update_authority.key());
        new_token.create(
            decimals,
            title,
            symbol,
            uri,
            mint_authority.clone(),
            update_authority,
            Some(mint_authority),
        )
    }

    fn get_token_info(token: Token) -> ProgramResult {
        msg!("-- Token: {}", &token.key());
        msg!("-- Current Token State:");
        msg!("      Decimals:       {}", token.data().decimals);
        msg!("      Title:          {}", token.data().title);
        msg!("      Symbol:         {}", token.data().symbol);
        msg!("      URI:            {}", token.data().uri);
        msg!("      Mint Auth:      {:#?}", token.data().mint_authority);
        msg!("      Freeze Auth:    {:#?}", token.data().freeze_authority);
        msg!("      Update Auth:    {}", token.data().update_authority);

        Ok(())
    }

    fn create_person(
        new_person: Create<Person>,
        id: u8,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        new_person.create(id, name, authority)
    }
}

#[nautilus]
#[default_instructions(Create, Delete, Update)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}
