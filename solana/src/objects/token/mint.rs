use solana_program::{
    account_info::{AccountInfo, IntoAccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::{
    create_mint, Create, NautilusAccountInfo, NautilusCreateMint, NautilusSigner, Signer, Wallet,
};

#[derive(Clone)]
pub struct Mint<'a> {
    pub account_info: AccountInfo<'a>,
    pub token_program: AccountInfo<'a>,
}

impl<'a> IntoAccountInfo<'a> for Mint<'a> {
    fn into_account_info(self) -> AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> NautilusAccountInfo<'a> for Mint<'a> {
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
        &self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
    ) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        create_mint(
            self.clone(),
            decimals,
            mint_authority,
            freeze_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
        )
    }

    fn create_with_payer(
        &self,
        decimals: u8,
        mint_authority: impl NautilusSigner<'a>,
        freeze_authority: Option<impl NautilusAccountInfo<'a>>,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        create_mint(
            self.clone(),
            decimals,
            mint_authority,
            freeze_authority,
            payer,
            self.rent.to_owned(),
            self.system_program.to_owned(),
            self.self_account.token_program.to_owned(),
        )
    }
}
