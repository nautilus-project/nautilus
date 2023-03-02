#[derive(Debug)]
pub struct EnforceStructsError();

impl std::fmt::Display for EnforceStructsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Only structs are allowed for the #[derive(NautilusAccount)] macro."
        )
    }
}

impl std::error::Error for EnforceStructsError {}

#[derive(Debug)]
pub struct EnforcePrimaryKeyType();

impl std::fmt::Display for EnforcePrimaryKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Primary key fields can only be of type 'u8', 'String', or 'Pubkey'."
        )
    }
}

impl std::error::Error for EnforcePrimaryKeyType {}
