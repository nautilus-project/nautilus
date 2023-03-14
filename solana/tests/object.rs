#[test]
fn object() {
    use nautilus::*;

    #[derive(Nautilus)]
    pub struct Hero {
        #[primary_key(autoincrement = true)]
        id: u8,
        name: String,
        #[authority]
        authority: Pubkey,
    }
}
