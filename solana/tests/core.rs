#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(NautilusEntrypoint)]
    enum MyInstructions<'a> {
        CreatePerson(NautilusCreateArgs<'a, Person>),
        UpdatePerson(NautilusDeleteArgs<'a>),
        DeletePerson(NautilusUpdateArgs<'a, PersonOptionized>),
    }

    #[derive(NautilusAccount)]
    struct Person {
        #[primary_key(autoincrement = false)]
        id: u8,
        name: String,
        #[authority]
        authority: Pubkey,
        #[authority]
        signer: Pubkey,
    }
}
