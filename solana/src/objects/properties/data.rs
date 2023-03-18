pub trait NautilusData<'a>: super::NautilusAccountInfo<'a> {
    fn seeds(&self) -> [&'a [u8]; 2];
    fn seeds_with_bump(&self, bump: &'a [u8]) -> [&'a [u8]; 3];
    fn pda(
        &self,
        program_id: &'a solana_program::pubkey::Pubkey,
    ) -> (solana_program::pubkey::Pubkey, u8);
    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError>;
    fn count_authorities() -> u8;
}

pub trait NautilusTable<'a>: NautilusData<'a> {
    fn primary_key(&self) -> &'a [u8];
}
