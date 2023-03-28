use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Mint, to: Mint) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(from: Mint, to: Mint, amount: u64) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[test]
fn entry_associated_token() {}
