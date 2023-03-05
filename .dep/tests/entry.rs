#[test]
fn can_parse() {
    use nautilus::*;

    #[derive(Nautilus)]
    enum MyInstructions {
        CreatePerson,
        UpdatePerson,
        DeletePerson,
    }
}
