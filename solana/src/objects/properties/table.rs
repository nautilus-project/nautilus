pub trait NautilusData: Clone + borsh::BorshDeserialize + borsh::BorshSerialize {
    fn primary_key<'a>(&self) -> &'a [u8];
    fn seeds<'a>(&self) -> [&'a [u8]; 2];
    fn pda<'a>(
        &self,
        program_id: &'a solana_program::pubkey::Pubkey,
    ) -> (solana_program::pubkey::Pubkey, u8);
    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError>;
    fn count_authorities(&self) -> u8;
}

pub trait NautilusTable<'a>: super::NautilusAccountInfo<'a> {
    fn primary_key(&self) -> &'a [u8];
    fn seeds(&self) -> [&'a [u8]; 2];
    fn pda(&self) -> (solana_program::pubkey::Pubkey, u8);
    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError>;
    fn count_authorities(&self) -> u8;
}
