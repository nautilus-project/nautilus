use nautilus::*;

#[nautilus]
mod program_nautilus {
    // Right now, the Nautilus Index must be initialized ahead of time.
    // Perhaps we can do this with the CLI.
    fn initialize<'a>(mut nautilus_index: Create<'a, NautilusIndex<'a>>) -> ProgramResult {
        splog_info!("Index size: {}", nautilus_index.span()?);
        nautilus_index.create()
    }

    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        splog_info!("-- New Person:        {}", &new_person.key());
        splog_info!("-- Authority:         {}", &authority);
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
        splog_info!("-- New Home: {}", &new_home.key());
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
        splog_info!("-- New Car: {}", &new_car.key());
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
        splog_info!("-- Token: {}", self.key());
        print_mint_data(&self.mint.data);
        print_metadata_data(&self.metadata.data);
    }
}

impl TestPrint for Record<'_, Person> {
    fn print(&self) {
        splog_info!("-- Person: {}", self.key());
        splog_info!("      ID:             {}", self.data.id);
        splog_info!("      Name:           {}", self.data.name);
        splog_info!("      Authority:      {}", self.data.authority);
    }
}

impl TestPrint for Record<'_, Home> {
    fn print(&self) {
        splog_info!("-- Home: {}", self.key());
        splog_info!("      ID:             {}", self.data.id);
        splog_info!("      House Number:   {}", self.data.house_number);
        splog_info!("      Street:         {}", self.data.street);
    }
}

impl TestPrint for Record<'_, Car> {
    fn print(&self) {
        splog_info!("-- Car: {}", self.key());
        splog_info!("      ID:             {}", self.data.id);
        splog_info!("      Make:           {}", self.data.make);
        splog_info!("      Model:          {}", self.data.model);
        splog_info!("      Purchase Auth:  {}", self.data.purchase_authority);
        splog_info!("      Operating Auth: {}", self.data.operating_authority);
    }
}

fn print_mint_data(data: &MintState) {
    splog_info!("-- Mint Data:");
    splog_info!("  Mint Authority:         {:#?}", data.mint_authority);
    splog_info!("  Supply:                 {}", data.supply);
    splog_info!("  Decimals:               {}", data.decimals);
    splog_info!("  Is Initialized:         {}", data.is_initialized);
    splog_info!("  Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState) {
    splog_info!("-- Metadata Data:");
    splog_info!("  Mint:                   {:#?}", data.mint);
    splog_info!("  Primary Sale Happened:  {}", data.primary_sale_happened);
    splog_info!("  Is Mutable:             {}", data.is_mutable);
    splog_info!("  Edition Nonce:          {:#?}", data.edition_nonce);
    splog_info!("  Title:                  {}", data.data.name);
    splog_info!("  Symbol:                 {}", data.data.symbol);
    splog_info!("  URI:                    {}", data.data.uri);
}
