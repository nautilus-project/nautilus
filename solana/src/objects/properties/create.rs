use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// use crate::{Metadata, Mint, NautilusData};

use super::{signer::NautilusSigner, NautilusAccountInfo};

#[derive(Clone)]
pub struct Create<'a, T>
where
    T: 'a + NautilusAccountInfo,
{
    pub fee_payer: Box<AccountInfo<'a>>,
    pub system_program: Box<AccountInfo<'a>>,
    pub rent: Box<AccountInfo<'a>>,
    pub self_account: Box<T>,
}

// impl<'a, T> Create<'a, T>
// where
//     T: NautilusAccountInfo<'a> + 'a,
// {
//     pub fn new(
//         fee_payer: Box<AccountInfo<'a>>,
//         system_program: Box<AccountInfo<'a>>,
//         rent: Box<AccountInfo<'a>>,
//         self_account: Box<T>,
//     ) -> Self {
//         Self {
//             fee_payer,
//             system_program,
//             rent,
//             self_account,
//         }
//     }
// }

// impl<'a, T> IntoAccountInfo<'a> for Create<'a, T>
// where
//     T: NautilusAccountInfo<'a> + 'a,
// {
//     fn into_account_info(self) -> AccountInfo<'a> {
//         self.self_account.into_account_info()
//     }
// }

// impl<'a, T> NautilusAccountInfo<'a> for Create<'a, T>
// where
//     T: NautilusAccountInfo<'a> + 'a,
// {
//     fn key(&self) -> &'a Pubkey {
//         self.self_account.key()
//     }

//     fn is_signer(&self) -> bool {
//         self.self_account.is_signer()
//     }

//     fn is_writable(&self) -> bool {
//         self.self_account.is_writable()
//     }

//     fn lamports(&self) -> u64 {
//         self.self_account.lamports()
//     }

//     fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
//         self.self_account.mut_lamports()
//     }

//     fn owner(&self) -> &'a Pubkey {
//         self.self_account.owner()
//     }

//     fn span(&self) -> usize {
//         self.self_account.span()
//     }
// }

// impl<'a, T> NautilusSigner<'a> for Create<'a, T> where T: NautilusAccountInfo<'a> + 'a {}

pub trait NautilusCreate {
    fn create(&mut self) -> ProgramResult;
    fn create_with_payer(&mut self, payer: impl NautilusSigner) -> ProgramResult;
}

// pub trait NautilusCreateMint<'a> {
//     fn create(
//         &mut self,
//         decimals: u8,
//         mint_authority: impl NautilusSigner<'a>,
//         freeze_authority: Option<impl NautilusAccountInfo<'a>>,
//     ) -> ProgramResult;

//     fn create_with_payer(
//         &mut self,
//         decimals: u8,
//         mint_authority: impl NautilusSigner<'a>,
//         freeze_authority: Option<impl NautilusAccountInfo<'a>>,
//         payer: impl NautilusSigner<'a>,
//     ) -> ProgramResult;
// }

// pub trait NautilusCreateMetadata<'a> {
//     fn create(
//         &mut self,
//         title: String,
//         symbol: String,
//         uri: String,
//         mint: Mint<'a>,
//         mint_authority: impl NautilusSigner<'a>,
//         update_authority: impl NautilusAccountInfo<'a>,
//     ) -> ProgramResult;

//     fn create_with_payer(
//         &mut self,
//         title: String,
//         symbol: String,
//         uri: String,
//         mint: Mint<'a>,
//         mint_authority: impl NautilusSigner<'a>,
//         update_authority: impl NautilusAccountInfo<'a>,
//         payer: impl NautilusSigner<'a>,
//     ) -> ProgramResult;
// }

// pub trait NautilusCreateToken<'a> {
//     fn create(
//         &mut self,
//         decimals: u8,
//         title: String,
//         symbol: String,
//         uri: String,
//         mint_authority: impl NautilusSigner<'a>,
//         update_authority: impl NautilusAccountInfo<'a>,
//         freeze_authority: Option<impl NautilusAccountInfo<'a>>,
//     ) -> ProgramResult;

//     fn create_with_payer(
//         &mut self,
//         decimals: u8,
//         title: String,
//         symbol: String,
//         uri: String,
//         mint_authority: impl NautilusSigner<'a>,
//         update_authority: impl NautilusAccountInfo<'a>,
//         freeze_authority: Option<impl NautilusAccountInfo<'a>>,
//         payer: impl NautilusSigner<'a>,
//     ) -> ProgramResult;

//     fn metadata(&self) -> Metadata<'a>;
// }

// pub trait NautilusCreateAssociatedTokenAccount<'a> {
//     fn create(&mut self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult;

//     fn create_with_payer(
//         &mut self,
//         mint: Mint<'a>,
//         owner: impl NautilusAccountInfo<'a>,
//         payer: impl NautilusSigner<'a>,
//     ) -> ProgramResult;
// }

// pub trait NautilusCreateRecord<'a, T: NautilusData> {
//     fn create_record(&mut self, data: T) -> ProgramResult;
//     fn create_record_with_payer(
//         &mut self,
//         data: T,
//         payer: impl NautilusSigner<'a>,
//     ) -> ProgramResult;
// }
