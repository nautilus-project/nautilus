#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(Nautilus, BorshDeserialize, BorshSerialize)]
    enum MyInstructions {
        CreatePerson,
        UpdatePerson,
        DeletePerson,
    }
}
