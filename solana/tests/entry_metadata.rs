use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Metadata, to: Metadata) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(from: Metadata, to: Metadata, amount: u64) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[test]
fn entry_associated_token() {}
