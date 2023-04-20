use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Mint as MintState;

use crate::{
    cpi::create::create_mint, objects::DATA_NOT_SET_MSG, Create, Mut, NautilusAccountInfo,
    NautilusCreateMint, NautilusMut, NautilusSigner, Signer,
};

#[derive(Clone)]
pub struct Mint<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub data: Option<Box<MintState>>,
}

impl<'a> Mint<'a> {
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        load_data: bool,
    ) -> Self {
        let mut obj = Self {
            account_info,
            token_program,
            data: None,
        };
        if load_data {
            obj.load_data();
        };
        obj
    }

    fn load_data(&mut self) {
        match MintState::unpack(match &self.account_info.try_borrow_data() {
            Ok(data) => data,
            Err(e) => {
                msg!("Could not read data from {}", &self.account_info.key);
                msg!("Is it empty?");
                panic!("{}", e);
            }
        }) {
            Ok(state) => self.data = Some(Box::new(state)),
            Err(_) => {
                msg!("Error parsing Mint state from {}", &self.account_info.key);
                msg!("Are you sure this is a Mint?");
                self.data = None
            }
        }
    }

    pub fn data(&self) -> Box<MintState> {
        match &self.data {
            Some(data) => data.clone(),
            None => panic!("{}", DATA_NOT_SET_MSG),
        }
    }
}

impl<'a> IntoAccountInfo<'a> for Mint<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        *self.account_info
    }
}

impl NautilusAccountInfo for Mint<'_> {
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
        spl_token::state::Mint::LEN
    }
}

impl NautilusCreateMint for Create<'_, Mint<'_>> {
    fn create(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner,
        freeze_authority: Option<impl NautilusAccountInfo>,
    ) -> ProgramResult {
        create_mint(
            self.fee_payer.key,
            self.self_account.key(),
            self.self_account.required_rent()?,
            self.self_account.size(),
            self.self_account.token_program.key,
            mint_authority.key(),
            freeze_authority.map(|_| self.key()),
            decimals,
            &[
                *self.fee_payer.clone(),
                // *self.into(),
                *self.system_program.clone(),
            ],
            &[
                // *self.into(),
                // mint_authority.into(),
                // *self.self_account.token_program.clone(),
                *self.rent.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner,
        freeze_authority: Option<impl NautilusAccountInfo>,
        payer: impl NautilusSigner,
    ) -> ProgramResult {
        create_mint(
            payer.key(),
            self.self_account.key(),
            self.self_account.required_rent()?,
            self.self_account.size(),
            self.self_account.token_program.key,
            mint_authority.key(),
            freeze_authority.map(|_| self.key()),
            decimals,
            &[
                // payer.into(),
                // *self.into(),
                *self.system_program.clone(),
            ],
            &[
                // *self.into(),
                // mint_authority.into(),
                // *self.self_account.token_program.clone(),
                *self.rent.clone(),
            ],
        )?;
        self.self_account.load_data();
        Ok(())
    }
}

impl<'a> IntoAccountInfo<'a> for Create<'a, Mint<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a> NautilusMut for Mut<Mint<'a>> {
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &mut u64>, ProgramError> {
        // self.self_account.account_info.try_borrow_mut_lamports()
        todo!()
    }
}

impl<'a> IntoAccountInfo<'a> for Mut<Mint<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl NautilusSigner for Signer<Mint<'_>> {}

impl<'a> IntoAccountInfo<'a> for Signer<Mint<'a>> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}
