use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account as AssociatedTokenAccountState;

use crate::{
    cpi::create::create_associated_token_account, objects::DATA_NOT_SET_MSG, Create, Mint, Mut,
    NautilusAccountInfo, NautilusCreateAssociatedTokenAccount, NautilusMut, NautilusSigner, Signer,
};

#[derive(Clone)]
pub struct AssociatedTokenAccount<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub associated_token_program: Box<AccountInfo<'a>>,
    pub data: Option<Box<AssociatedTokenAccountState>>,
}

impl<'a> AssociatedTokenAccount<'a> {
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        associated_token_program: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        let mut obj = Self {
            account_info,
            token_program,
            associated_token_program,
            data: None,
        };
        if load_data {
            obj.load_data();
        };
        obj
    }

    fn load_data(&mut self) {
        match AssociatedTokenAccountState::unpack(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(Box::new(state)),
            Err(_) => {
                msg!(
                    "Error parsing AssociatedTokenAccount state from {}",
                    &self.account_info.key
                );
                msg!("Are you sure this is an AssociatedTokenAccount?");
                self.data = None
            }
        }
    }

    pub fn data(&self) -> Box<AssociatedTokenAccountState> {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a> IntoAccountInfo<'a> for AssociatedTokenAccount<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl NautilusAccountInfo for AssociatedTokenAccount<'_> {
    fn key(&self) -> &Pubkey {
        self.account_info.key
    }

    fn is_signer(&self) -> bool {
        self.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.account_info.lamports()
    }

    fn owner(&self) -> &Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl NautilusCreateAssociatedTokenAccount for Create<'_, AssociatedTokenAccount<'_>> {
    fn create(
        &mut self,
        mint: impl NautilusAccountInfo,
        owner: impl NautilusAccountInfo,
    ) -> ProgramResult {
        create_associated_token_account(
            self.fee_payer.key,
            owner.key(),
            mint.key(),
            self.self_account.token_program.key,
            &[
                // mint.clone().into(),
                // *self.self_account.into(),
                // owner.into(),
                *self.fee_payer.clone(),
                *self.system_program.clone(),
                // *self.self_account.token_program.clone(),
                // *self.self_account.associated_token_program.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        mint: impl NautilusAccountInfo,
        owner: impl NautilusAccountInfo,
        payer: impl NautilusSigner,
    ) -> ProgramResult {
        create_associated_token_account(
            payer.key(),
            owner.key(),
            mint.key(),
            self.self_account.token_program.key,
            &[
                // mint.clone().into(),
                // *self.self_account.into(),
                // owner.into(),
                // payer.into(),
                *self.system_program.clone(),
                // *self.self_account.token_program.clone(),
                // *self.self_account.associated_token_program.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }
}

impl<'a> IntoAccountInfo<'a> for Create<'a, AssociatedTokenAccount<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a> NautilusMut for Mut<AssociatedTokenAccount<'a>> {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError> {
        // self.self_account.account_info.try_borrow_mut_lamports()
        todo!()
    }
}

impl<'a> IntoAccountInfo<'a> for Mut<AssociatedTokenAccount<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl NautilusSigner for Signer<AssociatedTokenAccount<'_>> {}

impl<'a> IntoAccountInfo<'a> for Signer<AssociatedTokenAccount<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}
