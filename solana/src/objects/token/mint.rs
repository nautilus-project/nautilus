use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey,
};
use spl_token::state::Mint as MintState;

use crate::{
    cpi, error::NautilusError, Create, NautilusAccountInfo, NautilusCreateMint, NautilusSigner,
    Signer, Wallet,
};

#[derive(Clone)]
pub struct Mint<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub data: MintState,
}

impl<'a> Mint<'a> {
    pub fn new(account_info: Box<AccountInfo<'a>>, token_program: Box<AccountInfo<'a>>) -> Self {
        Self {
            account_info,
            token_program,
            data: MintState::default(),
        }
    }

    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data = match MintState::unpack(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    String::from("token_mint"),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => state_data,
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    String::from("token_mint"),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            account_info,
            token_program,
            data,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for Mint<'a> {
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
        spl_token::state::Mint::LEN
    }
}

impl<'a> NautilusCreateMint<'a> for Create<'a, Mint<'a>> {
    fn create(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        cpi::create::create_mint(
            self.clone(),
            decimals,
            mint_authority,
            freeze_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
        )?;
        self.self_account = Mint::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
        )?;
        Ok(())
    }

    fn create_with_payer(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        cpi::create::create_mint(
            self.clone(),
            decimals,
            mint_authority,
            freeze_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
        )?;
        self.self_account = Mint::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
        )?;
        Ok(())
    }
}
