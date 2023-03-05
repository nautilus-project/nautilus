//
//
// ----------------------------------------------------------------
//
//                      Nautilus Objects
//
// ----------------------------------------------------------------
//
//
pub mod token;
pub mod wallet;

pub use token::Token;

pub trait NautilusAccountInfo<'a>:
    solana_program::account_info::IntoAccountInfo<'a> + Sized
{
    fn key(&self) -> &'a solana_program::pubkey::Pubkey;
    fn is_signer(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn lamports(&self) -> u64;
    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>;
    fn owner(&self) -> &'a solana_program::pubkey::Pubkey;
    //
    fn span(&self) -> usize;
    fn size(&self) -> u64 {
        self.span().try_into().unwrap()
    }
    fn required_rent(&self) -> Result<u64, solana_program::program_error::ProgramError> {
        use solana_program::sysvar::Sysvar;
        Ok((solana_program::sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()))
    }
    //
    fn transfer_lamports<T: NautilusAccountInfo<'a>>(
        self,
        to: T,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(self.key(), to.key(), amount),
            &[self.into_account_info(), to.into_account_info()], // TODO: Needs system program !
        )
    }
}

pub trait NautilusObject<'a>:
    NautilusAccountInfo<'a> + From<solana_program::account_info::AccountInfo<'a>>
{
    fn seeds(&self) -> [&'a [u8]; 2];
    fn seeds_with_bump(&self, bump: &'a [u8]) -> [&'a [u8]; 3];
    fn pda(
        &self,
        program_id: &'a solana_program::pubkey::Pubkey,
    ) -> (solana_program::pubkey::Pubkey, u8);
    //
    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError>;
    fn count_authorities() -> u8;
}

pub trait NautilusTable<'a>: NautilusObject<'a> {
    fn primary_key(&self) -> &'a [u8];
}
