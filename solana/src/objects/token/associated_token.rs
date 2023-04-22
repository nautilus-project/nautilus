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
    cpi::create::create_associated_token_account, objects::DATA_NOT_SET_MSG, Create, Mint,
    NautilusAccountInfo, NautilusCreateAssociatedTokenAccount, NautilusSigner, Signer, Wallet,
};

#[derive(Clone)]
pub struct AssociatedTokenAccount<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub associated_token_program: Box<AccountInfo<'a>>,
    pub data: Option<AssociatedTokenAccountState>,
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
            Ok(state) => self.data = Some(state),
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

    pub fn data(&self) -> AssociatedTokenAccountState {
        match self.data {
            Some(data) => data,
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a> IntoAccountInfo<'a> for AssociatedTokenAccount<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for AssociatedTokenAccount<'a> {
    fn account_info(&self) -> Box<AccountInfo<'a>> {
        self.account_info.clone()
    }

    fn key(&self) -> &'a Pubkey {
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

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a> NautilusCreateAssociatedTokenAccount<'a> for Create<'a, AssociatedTokenAccount<'a>> {
    fn create(&mut self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_associated_token_account(
            self.self_account.clone(),
            mint,
            owner,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        mint: Mint<'a>,
        owner: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        create_associated_token_account(
            self.self_account.clone(),
            mint,
            owner,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )?;
        self.self_account.load_data();
        Ok(())
    }
}
