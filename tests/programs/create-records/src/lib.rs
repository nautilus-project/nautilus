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
        msg!("-- Token:             {}", &new_token.key());
        msg!(
            "-- Metadata:          {}",
            &new_token.self_account.metadata.key()
        );
        msg!("-- Mint Authority:    {}", &mint_authority.key());
        msg!("-- Update Authority:  {:#?}", &update_authority.key());
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

    fn read_token(token: Token) -> ProgramResult {
        token.print();
        Ok(())
    }

    // Right now, the Nautilus Index must be initialized ahead of time.
    // Perhaps we can do this with the CLI.

    fn initialize<'a>(mut nautilus_index: Create<'a, NautilusIndex<'a>>) -> ProgramResult {
        nautilus_index.create()
    }

    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        msg!("-- New Person:        {}", &new_person.key());
        msg!("-- Authority:         {}", &authority);
        new_person.create(name, authority)
    }

    fn read_person<'a>(person: Record<'a, Person>) -> ProgramResult {
        person.print();
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

    fn read_home<'a>(home: Record<'a, Home>) -> ProgramResult {
        home.print();
        Ok(())
    }

    fn create_car<'a>(
        mut new_car: Create<'a, Record<'a, Car>>,
        make: String,
        model: String,
        purchase_authority: Pubkey,
        operating_authority: Pubkey,
    ) -> ProgramResult {
        msg!("-- New Car: {}", &new_car.key());
        new_car.create(make, model, purchase_authority, operating_authority)
    }

    fn read_car<'a>(car: Record<'a, Car>) -> ProgramResult {
        car.print();
        Ok(())
    }
}

#[derive(Table)]
struct Person {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(Table)]
struct Home {
    #[primary_key(autoincrement = false)]
    id: u8,
    house_number: u8,
    street: String,
}

#[derive(Table)]
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

//

pub trait TestPrint {
    fn print(&self);
}

impl TestPrint for Token<'_> {
    fn print(&self) {
        msg!("-- Token: {}", self.key());
        print_mint_data(&self.mint.data);
        print_metadata_data(&self.metadata.data);
    }
}

impl TestPrint for Record<'_, Person> {
    fn print(&self) {
        msg!("-- Person: {}", self.key());
        msg!("      ID:             {}", self.data.id);
        msg!("      Name:           {}", self.data.name);
        msg!("      Authority:      {}", self.data.authority);
    }
}

impl TestPrint for Record<'_, Home> {
    fn print(&self) {
        msg!("-- Home: {}", self.key());
        msg!("      ID:             {}", self.data.id);
        msg!("      House Number:   {}", self.data.house_number);
        msg!("      Street:         {}", self.data.street);
    }
}

impl TestPrint for Record<'_, Car> {
    fn print(&self) {
        msg!("-- Car: {}", self.key());
        msg!("      ID:             {}", self.data.id);
        msg!("      Make:           {}", self.data.make);
        msg!("      Model:          {}", self.data.model);
        msg!("      Purchase Auth:  {}", self.data.purchase_authority);
        msg!("      Operating Auth: {}", self.data.operating_authority);
    }
}

fn print_mint_data(data: &MintState) {
    msg!("-- Mint Data:");
    msg!("  Mint Authority:         {:#?}", data.mint_authority);
    msg!("  Supply:                 {}", data.supply);
    msg!("  Decimals:               {}", data.decimals);
    msg!("  Is Initialized:         {}", data.is_initialized);
    msg!("  Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState) {
    msg!("-- Metadata Data:");
    msg!("  Mint:                   {:#?}", data.mint);
    msg!("  Primary Sale Happened:  {}", data.primary_sale_happened);
    msg!("  Is Mutable:             {}", data.is_mutable);
    msg!("  Edition Nonce:          {:#?}", data.edition_nonce);
    msg!("  Title:                  {}", data.data.name);
    msg!("  Symbol:                 {}", data.data.symbol);
    msg!("  URI:                    {}", data.data.uri);
}
