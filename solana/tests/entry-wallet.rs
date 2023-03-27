use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Wallet, to: Wallet) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(from: Wallet, to: Wallet, amount: u64) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[test]
fn entry_wallet() {}
