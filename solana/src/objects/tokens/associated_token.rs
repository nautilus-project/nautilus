//! The `AssociatedTokenAccount<T>` Nautilus object and all associated trait implementations.
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey,
};
pub use spl_token::state::Account as AssociatedTokenAccountState;

use crate::{
    cpi, error::NautilusError, Create, Mint, Mut, NautilusAccountInfo, NautilusMut, NautilusSigner,
    Signer, Wallet,
};

/// The Nautilus object representing an associated token account.
///
/// The underlying account - designated in field `account_info` - is the associated token account.
///
/// We also include the read-only Token Program and Associated Token Program for any CPI operations necessary, since we do not
/// own this account.
#[derive(Clone)]
pub struct AssociatedTokenAccount<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_program: Box<AccountInfo<'a>>,
    pub associated_token_program: Box<AccountInfo<'a>>,
    pub data: AssociatedTokenAccountState,
}

impl<'a> AssociatedTokenAccount<'a> {
    /// Instantiate a new `AssociatedTokenAccount` without loading the account inner data from on-chain.
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        associated_token_program: Box<AccountInfo<'a>>,
    ) -> Self {
        Self {
            account_info,
            token_program,
            associated_token_program,
            data: AssociatedTokenAccountState::default(),
        }
    }

    /// Instantiate a new `AssociatedTokenAccount` and load the account inner data from on-chain.
    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        token_program: Box<AccountInfo<'a>>,
        associated_token_program: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data =
            match AssociatedTokenAccountState::unpack(match &account_info.try_borrow_data() {
                Ok(acct_data) => acct_data,
                Err(_) => {
                    return Err(NautilusError::LoadDataFailed(
                        String::from("associated_token_account"),
                        account_info.key.to_string(),
                    )
                    .into())
                }
            }) {
                Ok(state_data) => state_data,
                Err(_) => {
                    return Err(NautilusError::DeserializeDataFailed(
                        String::from("associated_token_account"),
                        account_info.key.to_string(),
                    )
                    .into())
                }
            };
        Ok(Self {
            account_info,
            token_program,
            associated_token_program,
            data,
        })
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

    fn span(&self) -> Result<usize, ProgramError> {
        Ok(AssociatedTokenAccountState::LEN)
    }
}

impl<'a> Mut<AssociatedTokenAccount<'a>> {
    /// Burn tokens from this associated token account.
    pub fn burn(
        &self,
        mint: impl NautilusAccountInfo<'a>,
        authority: impl NautilusSigner<'a>,
        amount: u64,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::burn(
            self.self_account.token_program.key,
            self.clone(),
            mint,
            authority,
            multisigs,
            amount,
        )
    }

    /// Freeze token movement from this associated token account.
    pub fn freeze(
        &self,
        mint: impl NautilusAccountInfo<'a>,
        freeze_authority: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::freeze_account(
            self.self_account.token_program.key,
            self.clone(),
            mint,
            freeze_authority,
            multisigs,
        )
    }

    /// Thaw this associated token account. It should be already frozen.
    pub fn thaw(
        &self,
        mint: impl NautilusAccountInfo<'a>,
        freeze_authority: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::thaw_account(
            self.self_account.token_program.key,
            self.clone(),
            mint,
            freeze_authority,
            multisigs,
        )
    }

    /// Transfer tokens from this associated token account to another.
    pub fn transfer(
        &self,
        to: impl NautilusMut<'a>,
        authority: impl NautilusSigner<'a>,
        amount: u64,
    ) -> ProgramResult {
        let multisigs: Option<Vec<Signer<Wallet>>> = None; // TODO: Multisig support
        cpi::token::transfer(
            self.self_account.token_program.key,
            self.clone(),
            to,
            authority,
            multisigs,
            amount,
        )
    }
}

impl<'a> Create<'a, AssociatedTokenAccount<'a>> {
    /// Create a new Associated Token Account using the Associated Token Program.
    pub fn create(&mut self, mint: Mint<'a>, owner: impl NautilusAccountInfo<'a>) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        })?;
        cpi::associated_token::create_associated_token_account(
            self.self_account.clone(),
            owner,
            mint,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )?;
        self.self_account = AssociatedTokenAccount::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
            self.self_account.associated_token_program.clone(),
        )?;
        Ok(())
    }

    /// This function is the same as `create(&mut self, ..)` but allows you to specify a rent payer.
    pub fn create_with_payer(
        &mut self,
        mint: Mint<'a>,
        owner: impl NautilusAccountInfo<'a>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        cpi::associated_token::create_associated_token_account(
            self.self_account.clone(),
            owner,
            mint,
            payer,
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
            self.self_account.associated_token_program.to_owned(),
        )?;
        self.self_account = AssociatedTokenAccount::load(
            self.self_account.account_info.clone(),
            self.self_account.token_program.clone(),
            self.self_account.associated_token_program.clone(),
        )?;
        Ok(())
    }
}
