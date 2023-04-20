// use nautilus::*;

// #[nautilus]
// pub mod my_mod {
//     fn print_something(from: Record<Person>, to: Mut<Wallet>) -> ProgramResult {
//         println!("Something");
//         println!("From: {}", from.key());
//         println!("To:   {}", to.key());
//         Ok(())
//     }
//     fn print_something_else(
//         from: Record<Person>,
//         to: Record<Person>,
//         amount: u64,
//     ) -> ProgramResult {
//         println!("Something else");
//         println!("From: {}", from.key());
//         println!("To:   {}", to.key());
//         println!("Amount: {}", amount);
//         Ok(())
//     }
// }

// #[derive(Nautilus)]
// struct Person {
//     #[primary_key(autoincrement = true)]
//     id: u8,
//     name: String,
//     #[authority]
//     authority: Pubkey,
// }

// #[test]
// fn entry_wallet() {}
