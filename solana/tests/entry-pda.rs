use nautilus::*;
use solana_program::vote::authorized_voters;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: Darryl, to: Darryl) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(from: Darryl, to: Darryl, amount: u64) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[derive(Nautilus)]
pub struct Darryl {
    #[primary_key(autoincrement = true)]
    id: u8,
    name: String,
    #[authority]
    authority: Pubkey,
}

#[test]
fn entry_pda() {}
