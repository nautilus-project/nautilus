use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Person, to: Person) -> ProgramResult {
        println!("Something");
        println!("From: {}\nTo:   {}", from.key(), to.key());
        println!(
            "-- From: \nID:   {}\nName: {}\nAuth: {}",
            from.id, from.name, from.authority
        );
        println!(
            "-- To: \nID:   {}\nName: {}\nAuth: {}",
            to.id, to.name, to.authority
        );
        Ok(())
    }
    fn print_something_else(from: Person, to: Person, amount: u64) -> ProgramResult {
        println!("Something else");
        println!("From: {}\nTo:   {}", from.key(), to.key());
        println!(
            "-- From: \nID:   {}\nName: {}\nAuth: {}",
            from.id, from.name, from.authority
        );
        println!(
            "-- To: \nID:   {}\nName: {}\nAuth: {}",
            to.id, to.name, to.authority
        );
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

#[test]
fn entry_wallet() {}
