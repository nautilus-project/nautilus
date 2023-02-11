#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(NautilusAccount)]
    struct Person {
        #[primary_key(autoincrement = true)]
        id: u32,
        name: String,
        #[authority]
        authority: Pubkey,
    }

    let person = Person {
        id: 30,
        name: String::from("Joe"),
        authority: Pubkey::new_unique(),
    };

    let _ = person;
}