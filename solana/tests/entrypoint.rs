#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(NautilusEntrypoint, BorshDeserialize, BorshSerialize)]
    enum MyInstructions {
        CreatePerson(CreatePersonArgs),
        UpdatePerson(DeletePersonArgs),
        DeletePerson(UpdatePersonArgs),
    }
}
