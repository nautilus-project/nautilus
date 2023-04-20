use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use self::mutable::NautilusMut;

pub mod create;
pub mod mutable;
pub mod signer;

pub trait NautilusAccountInfo: Clone {
    fn key(&self) -> &Pubkey;
    fn is_signer(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn lamports(&self) -> u64;
    fn owner(&self) -> &Pubkey;
    fn span(&self) -> usize;
    fn size(&self) -> u64 {
        self.span().try_into().unwrap()
    }
    fn required_rent(&self) -> Result<u64, solana_program::program_error::ProgramError> {
        use solana_program::sysvar::Sysvar;
        Ok((solana_program::sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()))
    }
}

pub trait NautilusTransferLamports: NautilusAccountInfo {
    fn transfer_lamports(self, to: impl NautilusMut, amount: u64) -> ProgramResult;
}

pub trait NautilusData: Clone + borsh::BorshDeserialize + borsh::BorshSerialize {
    const TABLE_NAME: &'static str;
    const AUTO_INCREMENT: bool;
    // fn default() -> Self;
    // fn load_data(account_info: Box<AccountInfo<'_>>) -> Result<Self, ProgramError>;
    // fn write_data(account_info: &mut Box<AccountInfo<'_>>) -> ProgramResult;
    fn primary_key<'a>(&self) -> &'a [u8];
    fn seeds<'a>(&self) -> [&'a [u8]; 2] {
        [Self::TABLE_NAME.as_bytes(), self.primary_key()]
    }
    fn pda<'a>(&self, program_id: &'a Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), program_id)
    }
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}

pub trait NautilusRecord: NautilusAccountInfo {
    fn primary_key(&self) -> &[u8];
    fn seeds(&self) -> [&[u8]; 2];
    fn pda(&self) -> (Pubkey, u8);
    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError>;
    fn count_authorities(&self) -> u8;
}
