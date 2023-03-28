use nautilus::*;

#[nautilus]
pub mod my_mod {
    use super::*;
    fn print_something(from: AssociatedTokenAccount, to: AssociatedTokenAccount) -> ProgramResult {
        println!("Something");
        Ok(())
    }
    fn print_something_else(
        from: AssociatedTokenAccount,
        to: AssociatedTokenAccount,
        amount: u64,
    ) -> ProgramResult {
        println!("Something else");
        Ok(())
    }
}

#[test]
fn entry_associated_token() {}
