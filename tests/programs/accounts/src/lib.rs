//! Testing non-record data accounts (PDAs).
use nautilus::splogger::{info, Splog};
use nautilus::*;

#[nautilus]
mod program_nautilus {
    fn create_person<'a>(
        mut new_person: Create<'a, Account<'a, Person>>,
        name: String,
        authority: Pubkey,
    ) -> ProgramResult {
        info!(" * New Person:        {}", &new_person.key());
        info!(" * Authority:         {}", &authority);
        //
        // /* Business Logic */
        //
        new_person.create(name, authority)?;
        //
        new_person.self_account.print();
        Ok(())
    }

    fn read_person<'a>(person: Account<'a, Person>) -> ProgramResult {
        person.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn create_home<'a>(
        mut new_home: Create<'a, Account<'a, Home>>,
        house_number: u8,
        street: String,
        some_pubkey: Pubkey,
    ) -> ProgramResult {
        info!(" * New Home: {}", &new_home.key());
        //
        // /* Business Logic */
        //
        new_home.create(house_number, street, (some_pubkey,))?; // Seed parameter required
                                                                //
        new_home.self_account.print();
        Ok(())
    }

    fn read_home<'a>(home: Account<'a, Home>) -> ProgramResult {
        home.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }

    fn create_car<'a>(
        mut new_car: Create<'a, Account<'a, Car>>,
        make: String,
        model: String,
        purchase_authority: Pubkey,
        operating_authority: Pubkey,
    ) -> ProgramResult {
        info!(" * New Car: {}", &new_car.key());
        //
        // /* Business Logic */
        //
        new_car.create(make, model, purchase_authority, operating_authority)?;
        //
        new_car.self_account.print();
        Ok(())
    }

    fn read_car<'a>(car: Account<'a, Car>) -> ProgramResult {
        car.print();
        //
        // /* Business Logic */
        //
        Ok(())
    }
}

#[derive(State)]
#[seeds(
    "person",               // Literal seed
    authority,              // Self-referencing seed
)]
struct Person {
    name: String,
    #[authority]
    authority: Pubkey,
}

#[derive(State)]
#[seeds(
    "home",                 // Literal seed
    some_pubkey: Pubkey,    // Parameter seed
)]
struct Home {
    house_number: u8,
    street: String,
}

#[derive(State)]
#[seeds(
    "car",                  // Literal seed
    purchase_authority,     // Self-referencing seed
    operating_authority,    // Self-referencing seed
)]
struct Car {
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

impl TestPrint for Account<'_, Person> {
    fn print(&self) {
        info!(" * Person: {}", self.key());
        info!("      Name:           {}", self.data.name);
        info!("      Authority:      {}", self.data.authority);
    }
}

impl TestPrint for Account<'_, Home> {
    fn print(&self) {
        info!(" * Home: {}", self.key());
        info!("      House Number:   {}", self.data.house_number);
        info!("      Street:         {}", self.data.street);
    }
}

impl TestPrint for Account<'_, Car> {
    fn print(&self) {
        info!(" * Car: {}", self.key());
        info!("      Make:           {}", self.data.make);
        info!("      Model:          {}", self.data.model);
        info!("      Purchase Auth:  {}", self.data.purchase_authority);
        info!("      Operating Auth: {}", self.data.operating_authority);
    }
}
