use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi::{create::create_account, transfer::transfer_lamports},
    Create, NautilusAccountInfo, NautilusCreate, NautilusMut, NautilusSigner,
    NautilusTransferLamports, Signer,
};

/// The Nautilus object representing a Solana system account.
///
/// The underlying account - designated in field `account_info` - is the system account
/// this Wallet represents.
///
/// We also include the read-only System Program for any CPI operations necessary, since we do not
/// own this account.
#[derive(Clone)]
pub struct Wallet<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub system_program: Box<AccountInfo<'a>>,
}

impl<'a> Wallet<'a> {
    /// Instantiate a new `Wallet` without loading the account inner data from on-chain.
    ///
    /// This function actually does nothing with on-chain data anyway, since system accounts have no inner data.
    pub fn new(account_info: Box<AccountInfo<'a>>, system_program: Box<AccountInfo<'a>>) -> Self {
        Self {
            account_info,
            system_program,
        }
    }

    /// Instantiate a new `Wallet` and load the account inner data from on-chain.
    ///
    /// This function actually does nothing with on-chain data anyway, since system accounts have no inner data.
    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        system_program: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        Ok(Self {
            account_info,
            system_program,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for Wallet<'a> {
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
        Ok(0)
    }
}

impl<'a> NautilusTransferLamports<'a> for Signer<Wallet<'a>> {
    fn transfer_lamports(self, to: impl NautilusMut<'a>, amount: u64) -> ProgramResult {
        let system_program = self.self_account.system_program.clone();
        transfer_lamports(self, to, amount, system_program)
    }
}

impl<'a> NautilusCreate<'a> for Create<'a, Wallet<'a>> {
    fn create(&mut self) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.clone(),
            system_program: self.system_program.clone(),
        });
        create_account(
            self.clone(),
            self.system_program.key,
            payer,
            self.system_program.clone(),
        )
    }

    fn create_with_payer(&mut self, payer: impl NautilusSigner<'a>) -> ProgramResult {
        create_account(
            self.clone(),
            self.system_program.key,
            payer,
            self.system_program.clone(),
        )
    }
}
