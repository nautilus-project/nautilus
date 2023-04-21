use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn create_token<'a>(
        mut new_token: Create<'a, Token<'a>>,
        decimals: u8,
        title: String,
        symbol: String,
        uri: String,
        mint_authority: Signer<Wallet<'a>>,
        update_authority: Signer<Wallet<'a>>,
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

    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        msg!("-- New Person: {}", &new_person.key());
        new_person.create(name, authority)
    }

    fn read_person(person: Record<Person>) -> ProgramResult {
        msg!("-- Person: {}", &person.key());
        msg!("      ID:             {}", person.data().id);
        msg!("      Name:           {}", person.data().name);
        msg!("      Authority:      {}", person.data().authority);
        Ok(())
    }

    fn create_home<'a>(
        mut new_home: Create<'a, Record<'a, Home>>,
        id: u8,
        house_number: u8,
        street: String,
    ) -> ProgramResult {
        msg!("-- New Home: {}", &new_home.key());
        new_home.create(id, house_number, street)
    }

    fn read_home(home: Record<Home>) -> ProgramResult {
        msg!("-- Home: {}", &home.key());
        msg!("      ID:             {}", home.data().id);
        msg!("      House Number:   {}", home.data().house_number);
        msg!("      Street:         {}", home.data().street);
        Ok(())
    }

    fn create_car<'a>(
        mut new_car: Create<'a, Record<'a, Car>>,
        id: u8,
        make: String,
        model: String,
        purchase_authority: Pubkey,
        operating_authority: Pubkey,
    ) -> ProgramResult {
        msg!("-- New Car: {}", &new_car.key());
        new_car.create(id, make, model, purchase_authority, operating_authority)
    }

    fn read_car(car: Record<Car>) -> ProgramResult {
        msg!("-- Car: {}", &car.key());
        msg!("      ID:             {}", car.data().id);
        msg!("      Make:           {}", car.data().make);
        msg!("      Model:          {}", car.data().model);
        msg!("      Purchase Auth:  {}", car.data().purchase_authority);
        msg!("      Operating Auth: {}", car.data().operating_authority);
        Ok(())
    }
}

#[derive(Nautilus)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(Nautilus)]
struct Home {
    #[primary_key(autoincrement = false)]
    id: u8,
    house_number: u8,
    street: String,
}

#[derive(Nautilus)]
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
