use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Token, to: Token) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(from: Token, to: Token, amount: u64) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[test]
fn entry_token() {}
