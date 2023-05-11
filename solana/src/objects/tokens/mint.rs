//! The `Mint<T>` Nautilus object and all associated trait implementations.
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey,
};
use spl_token::instruction::AuthorityType;
pub use spl_token::state::Mint as MintState;

use crate::{
    cpi, error::NautilusError, Create, Mut, NautilusAccountInfo, NautilusMut, NautilusSigner,
    Signer, Wallet,
};

/// The Nautilus object representing a mint account.
///
/// The underlying account - designated in field `account_info` - is the mint
/// account.
///
/// We also include the read-only Token Program for any CPI operations
/// necessary, since we do not own this account.
#[derive(Clone)]
pub struct Mint<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub data: MintState,
}

impl<'a> Mint<'a> {
    // Inner data state associated functions

    /// Instantiate a new `Mint` without loading the account inner data from
    /// on-chain.
    pub fn new(account_info: Box<AccountInfo<'a>>, token_program: Box<AccountInfo<'a>>) -> Self {
        Self {
            account_info,
            token_program,
            data: MintState::default(),
        }
    }

    /// Instantiate a new `Mint` and load the account inner data from on-chain.
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

    // Token program capabilities
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

    fn span(&self) -> Result<usize, ProgramError> {
        Ok(spl_token::state::Mint::LEN)
    }
}

impl<'a> Mut<Mint<'a>> {
    /// Mint new tokens to an associated token account.
    pub fn mint_to(
        &self,
        recipient: impl NautilusMut<'a>,
        mint_authority: impl NautilusSigner<'a>,
        amount: u64,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::mint_to(
            self.self_account.token_program.key,
            self.clone(),
            recipient,
            mint_authority,
            multisigs,
            amount,
        )
    }

    /// Change the mint's authority.
    pub fn set_authority(
        &self,
        new_authority: Option<&Pubkey>,
        authority_type: AuthorityType,
        current_authority: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::set_authority(
            self.self_account.token_program.key,
            self.clone(),
            new_authority,
            authority_type,
            current_authority,
            multisigs,
        )
    }
}

impl<'a> Create<'a, Mint<'a>> {
    /// Create a new SPL mint with a Token Program.
    pub fn create(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        })?;
        cpi::system::create_account(self.clone(), self.self_account.token_program.key, payer)?;
        cpi::token::initialize_mint(
            self.self_account.token_program.key,
            self.clone(),
            mint_authority.key(),
            freeze_authority.map(|f| f.key()),
            decimals,
            self.rent.to_owned(),
        )?;
        self.self_account = Mint::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
        )?;
        Ok(())
    }

    /// This function is the same as `create(&mut self, ..)` but allows you to
    /// specify a rent payer.
    pub fn create_with_payer(
        &mut self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        cpi::system::create_account(self.clone(), self.self_account.token_program.key, payer)?;
        cpi::token::initialize_mint(
            self.self_account.token_program.key,
            self.clone(),
            mint_authority.key(),
            freeze_authority.map(|f| f.key()),
            decimals,
            self.rent.to_owned(),
        )?;
        self.self_account = Mint::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
        )?;
        Ok(())
    }
}
