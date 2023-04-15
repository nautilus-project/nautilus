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

    fn create_person(new_person: Create<Person>, name: String, authority: Pubkey) -> ProgramResult {
        msg!("-- New Person: {}", &new_person.key());
        new_person.create(name, authority)
    }

    fn read_person(person: Person) -> ProgramResult {
        msg!("-- Person: {}", &person.key());
        msg!("      ID:             {}", person.data().id);
        msg!("      Name:           {}", person.data().name);
        msg!("      Authority:      {}", person.data().authority);
        Ok(())
    }

    fn create_home(
        new_home: Create<Home>,
        id: u8,
        house_number: u8,
        street: String,
    ) -> ProgramResult {
        msg!("-- New Home: {}", &new_home.key());
        new_home.create(id, house_number, street)
    }

    fn read_home(home: Home) -> ProgramResult {
        msg!("-- Home: {}", &home.key());
        msg!("      ID:             {}", home.data().id);
        msg!("      House Number:   {}", home.data().house_number);
        msg!("      Street:         {}", home.data().street);
        Ok(())
    }
}

#[nautilus]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[nautilus]
struct Home {
    #[primary_key(autoincrement = false)]
    id: u8,
    house_number: u8,
    street: String,
}

#[nautilus]
#[default_instructions(Create, Delete, Update)]
struct Car {
    #[primary_key(autoincrement = true)]
    id: u8,
    make: String,
    model: String,
    #[authority]
    purchase_authority: Pubkey,
    #[authority]
    operating_authority: Pubkey,
}
