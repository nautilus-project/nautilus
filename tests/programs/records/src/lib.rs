//! Testing data records (PDAs).
use nautilus::splogger::{info, Splog};
use nautilus::*;

#[nautilus]
mod program_nautilus {
    // Right now, the Nautilus Index must be initialized ahead of time.
    // Perhaps we can do this with the CLI.
    fn initialize<'a>(mut nautilus_index: Create<'a, NautilusIndex<'a>>) -> ProgramResult {
        info!("Index size: {}", nautilus_index.span()?);
        //
        // /* Business Logic */
        //
        nautilus_index.create()?;
        //
        Ok(())
    }

    fn create_person<'a>(
        mut new_person: Create<'a, Record<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        info!("-- New Person:        {}", &new_person.key());
        info!("-- Authority:         {}", &authority);
        //
        // /* Business Logic */
        //
        new_person.create(name, authority)?;
        //
        new_person.self_account.print();
        Ok(())
    }

    fn read_person<'a>(person: Record<'a, Person>) -> ProgramResult {
        person.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn create_home<'a>(
        mut new_home: Create<'a, Record<'a, Home>>,
        id: u8,
        house_number: u8,
        street: String,
    ) -> ProgramResult {
        info!("-- New Home: {}", &new_home.key());
        //
        // /* Business Logic */
        //
        new_home.create(id, house_number, street)?;
        //
        new_home.self_account.print();
        Ok(())
    }

    fn read_home<'a>(home: Record<'a, Home>) -> ProgramResult {
        home.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn create_car<'a>(
        mut new_car: Create<'a, Record<'a, Car>>,
        make: String,
        model: String,
        purchase_authority: Pubkey,
        operating_authority: Pubkey,
    ) -> ProgramResult {
        info!("-- New Car: {}", &new_car.key());
        //
        // /* Business Logic */
        //
        new_car.create(make, model, purchase_authority, operating_authority)?;
        //
        new_car.self_account.print();
        Ok(())
    }

    fn read_car<'a>(car: Record<'a, Car>) -> ProgramResult {
        car.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn fund_person<'a>(
        person: Mut<Record<'a, Person>>,
        payer: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        payer.transfer_lamports(person, amount)
    }

    fn transfer_from_person<'a>(
        person: Mut<Record<'a, Person>>,
        recipient: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        person.transfer_lamports(recipient, amount)
    }

    fn fund_home<'a>(
        home: Mut<Record<'a, Home>>,
        payer: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        payer.transfer_lamports(home, amount)
    }

    fn transfer_from_home<'a>(
        home: Mut<Record<'a, Home>>,
        recipient: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        home.transfer_lamports(recipient, amount)
    }

    fn fund_car<'a>(
        car: Mut<Record<'a, Car>>,
        payer: Signer<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        payer.transfer_lamports(car, amount)
    }

    fn transfer_from_car<'a>(
        car: Mut<Record<'a, Car>>,
        recipient: Mut<Wallet<'a>>,
        amount: u64,
    ) -> ProgramResult {
        car.transfer_lamports(recipient, amount)
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
        info!("-- Token: {}", self.key());
        print_mint_data(&self.mint.data);
        print_metadata_data(&self.metadata.data);
    }
}

impl TestPrint for Record<'_, Person> {
    fn print(&self) {
        info!("-- Person: {}", self.key());
        info!("      ID:             {}", self.data.id);
        info!("      Name:           {}", self.data.name);
        info!("      Authority:      {}", self.data.authority);
    }
}

impl TestPrint for Record<'_, Home> {
    fn print(&self) {
        info!("-- Home: {}", self.key());
        info!("      ID:             {}", self.data.id);
        info!("      House Number:   {}", self.data.house_number);
        info!("      Street:         {}", self.data.street);
    }
}

impl TestPrint for Record<'_, Car> {
    fn print(&self) {
        info!("-- Car: {}", self.key());
        info!("      ID:             {}", self.data.id);
        info!("      Make:           {}", self.data.make);
        info!("      Model:          {}", self.data.model);
        info!("      Purchase Auth:  {}", self.data.purchase_authority);
        info!("      Operating Auth: {}", self.data.operating_authority);
    }
}

fn print_mint_data(data: &MintState) {
    info!("-- Mint Data:");
    info!("  Mint Authority:         {:#?}", data.mint_authority);
    info!("  Supply:                 {}", data.supply);
    info!("  Decimals:               {}", data.decimals);
    info!("  Is Initialized:         {}", data.is_initialized);
    info!("  Freeze Authority:       {:#?}", data.freeze_authority);
}

fn print_metadata_data(data: &MetadataState) {
    info!("-- Metadata Data:");
    info!("  Mint:                   {:#?}", data.mint);
    info!("  Primary Sale Happened:  {}", data.primary_sale_happened);
    info!("  Is Mutable:             {}", data.is_mutable);
    info!("  Edition Nonce:          {:#?}", data.edition_nonce);
    info!("  Title:                  {}", data.data.name);
    info!("  Symbol:                 {}", data.data.symbol);
    info!("  URI:                    {}", data.data.uri);
}
