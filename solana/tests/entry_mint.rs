use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Mint, to: Mint) -> ProgramResult {
        println!("Something");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        Ok(())
    }
    fn print_something_else(from: Mint, to: Mint, amount: u64) -> ProgramResult {
        println!("Something else");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        println!("Amount: {}", amount);
        Ok(())
    }
}

#[test]
fn entry_associated_token() {}
