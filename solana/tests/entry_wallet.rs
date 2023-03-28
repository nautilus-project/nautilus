use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn print_something(from: Wallet, to: Wallet) -> ProgramResult {
        println!("Something");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        Ok(())
    }
    fn print_something_else(from: Wallet, to: Wallet, amount: u64) -> ProgramResult {
        println!("Something else");
        println!("From: {}", from.key());
        println!("To:   {}", to.key());
        println!("Amount: {}", amount);
        Ok(())
    }
}

#[test]
fn entry_wallet() {}
