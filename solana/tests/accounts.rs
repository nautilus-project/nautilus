#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(NautilusAccount)]
    struct Person {
        #[primary_key(autoincrement = false)]
        id: u8,
        name: String,
        #[authority]
        authority: Pubkey,
    }

    let person = Person {
        id: 30,
        name: String::from("Joe"),
        authority: Pubkey::new_unique(),
        // signer: Pubkey::new_unique(),
    };

    // impl NautilusOptionized for PersonOptionized
    // where
    //     PersonOptionized: NautilusAccountData,
    // {
    //     fn test_opt(&self) -> String {
    //         String::from("Optionized")
    //     }
    // }

    let person_optionized = PersonOptionized {
        id: 30,
        name: Some(String::from("Joe")),
        authority: Some(Pubkey::new_unique()),
        // signer: Some(Pubkey::new_unique()),
    };

    println!("\n    [Test Output]: Self");
    println!("id:           {}", person.id);
    println!("name:         {}", person.name);
    println!("auth:         {}", person.authority);

    println!("\n    [Test Output]: Self: NautilusAccountData");
    println!("Span:         {:?}", person.span());
    println!("Size:         {:?}", person.size());
    // println!("Rent:         {:?}", person.lamports_required());
    println!("Primary Key:  {:?}", person.primary_key());
    println!("Seeds:        {:?}", person.seeds());
    println!("Signer Seeds: {:?}", person.seeds_with_bump(&[1]));
    println!("PDA:          {:?}", person.pda(&Pubkey::new_unique()).0);

    println!("\n    [Test Output]: Optionized");
    println!("id:           {}", person_optionized.id);
    println!("name:         {:?}", person_optionized.name);
    println!("auth:         {:?}", person_optionized.authority);

    println!("\n    [Test Output]: Optionized: NautilusAccountData");
    println!("Span:         {:?}", person_optionized.span());
    println!("Size:         {:?}", person_optionized.size());
    // println!("Rent:         {:?}", person_optionized.lamports_required());
    println!("Primary Key:  {:?}", person_optionized.primary_key());
    println!("Seeds:        {:?}", person_optionized.seeds());
    println!(
        "Signer Seeds: {:?}",
        person_optionized.seeds_with_bump(&[1])
    );
    println!(
        "PDA:          {:?}",
        person_optionized.pda(&Pubkey::new_unique()).0
    );

    println!("\n    [Test Output]: NautilusAccountAuth");
    // println!("Check Auth:   {:?}", person.check_authorities(vec![]));

    println!("\n    [Test Output]: NautilusAccountCreate");
    // match Person::parse_nautilus_create_args(&Pubkey::new_unique(), &vec![], person) {
    //     Ok(_) => println!("CreateArgs:   Ok"),
    //     Err(_) => panic!("CreateArgs:   FAILED"),
    // };

    println!("\n    [Test Output]: NautilusAccountDelete");
    // match Person::parse_nautilus_delete_args(&Pubkey::new_unique(), &vec![]) {
    //     Ok(_) => println!("DeleteArgs:   Ok"),
    //     Err(_) => panic!("DeleteArgs:   FAILED"),
    // };

    println!("\n    [Test Output]: NautilusAccountUpdate");
    // match Person::parse_nautilus_update_args(&Pubkey::new_unique(), &vec![], person) {
    //     Ok(_) => println!("UpdateArgs:   Ok"),
    //     Err(_) => panic!("UpdateArgs:   FAILED"),
    // };

    let _ = person;
}
