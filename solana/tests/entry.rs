use nautilus::*;

#[nautilus]
pub mod my_mod {
    fn print_something(from: Darryl, to: String) {
        println!("Something");
    }
    fn print_something_else(from: Darryl, to: Darryl, amount: u64) {
        println!("Something else");
    }
}

#[test]
fn entry() {}
