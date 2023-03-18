use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Darryl, to: JoeC) {
        println!("Something");
    }
    #[derive(Nautilus, BorshDeserialize, BorshSerialize)]
    pub struct JoeC {}
}

#[test]
fn entry() {}
