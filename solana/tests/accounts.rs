#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(NautilusAccount)]
    struct Person {
        #[primary_key(autoincrement = false)]
        id: u8,
        name: String,
        #[authority]
        multisig1: Pubkey,
        #[authority]
        multisig2: Pubkey,
        #[authority]
        multisig3: Pubkey,
    }

    let person = Person {
        id: 30,
        name: String::from("Joe"),
        multisig1: Pubkey::new_unique(),
        multisig2: Pubkey::new_unique(),
        multisig3: Pubkey::new_unique(),
    };

    println!("\n    [Test Output]: Self");
    println!("id:           {}", person.id);
    println!("name:         {}", person.name);
    println!("multisig1:    {}", person.multisig1);
    println!("multisig2:    {}", person.multisig2);
    println!("multisig3:    {}", person.multisig3);

    println!("\n    [Test Output]: Self: NautilusAccountData");
    println!("Span:         {:?}", person.span());
    println!("Size:         {:?}", person.size());
    // println!("Rent:         {:?}", person.lamports_required());
    println!("Primary Key:  {:?}", person.primary_key());
    println!("Seeds:        {:?}", person.seeds());
    println!("Signer Seeds: {:?}", person.seeds_with_bump(&[1]));
    println!("PDA:          {:?}", person.pda(&Pubkey::new_unique()).0);

    let _ = person;
}
