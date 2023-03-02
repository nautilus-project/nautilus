use crate::{sysvar, BorshDeserialize, BorshSerialize, ProgramError, Pubkey};

// The trait that will enable operations to a PDA's inner data and seeds.
pub trait NautilusAccountData: BorshDeserialize + BorshSerialize + Sized {
    /// The "table name" for the PDA - a string literal of the struct's name lower case.
    const TABLE_NAME: &'static str;
    /// Whether or not this PDA has autoincrement enabled.
    const AUTO_INCREMENT: bool;

    /// Returns the span of the account's data as usize.
    fn span(&self) -> Result<usize, ProgramError> {
        Ok((self.clone().try_to_vec()?).len())
    }

    /// Returns the size of the accounts data as u64.
    fn size(&self) -> Result<u64, ProgramError> {
        Ok(self.span()?.try_into().unwrap())
    }

    /// Calculates the rent required for this account's allocated data size.
    fn lamports_required(&self) -> Result<u64, ProgramError> {
        use sysvar::Sysvar;
        Ok((sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()?))
    }

    /// Returns the primary key serialized into a buffer to be used in seeds.
    ///
    /// Implemented when the #[derive(NautilusAccount)] macro is added.
    fn primary_key<'a>(&self) -> &'a [u8];

    /// The seeds for this PDA as an array of buffers.
    ///
    /// These seeds will always be <table-name> + <primary-key>.
    ///
    /// For autoincrement PDAs, the primary key is always 1.
    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key()]
    }

    /// Allows you to pass in a bump and get the seeds with the bump included.
    ///
    /// For use in signing with `invoke_signed`.
    fn seeds_with_bump<'a>(&self, bump: &'a [u8]) -> [&'a [u8]; 3] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key(), bump]
    }

    /// Returns the address and bump of the PDA.
    ///
    /// Simply returns the result of `Pubkey::find_program_address(..)`.
    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
}
