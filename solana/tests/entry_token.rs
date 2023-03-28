use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn print_something(from: Token, to: Token) -> ProgramResult {
        println!("Something");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        Ok(())
    }
    fn print_something_else(from: Token, to: Token, amount: u64) -> ProgramResult {
        println!("Something else");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        println!("Amount: {}", amount);
        Ok(())
    }
}

#[test]
fn entry_token() {}
