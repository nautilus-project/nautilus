use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Wallet, to: Wallet) {
        println!("Something");
    }
    fn print_something_else(from: Wallet, to: Wallet, amount: u64) {
        println!("Something else");
    }
}

#[test]
fn entry() {}
