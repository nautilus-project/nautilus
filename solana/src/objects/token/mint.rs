#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Mint<'a> {
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub token_program: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Mint<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::objects::properties::NautilusAccountInfo<'a> for Mint<'a> {
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
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

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a> crate::objects::properties::tokens::NautilusCreateMint<'a>
    for crate::objects::properties::create::Create<'a, Mint<'a>>
{
    fn create_mint<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;
        use solana_program::{program_pack::Pack, sysvar::Sysvar};

        let payer = self.fee_payer.clone();
        let rent = self.rent.clone();
        let system_program = self.system_program.clone();
        let token_program = self.self_account.token_program.clone();
        solana_program::program::invoke(
            &solana_program::system_instruction::create_account(
                &self.fee_payer.key,
                &self.self_account.key(),
                (solana_program::rent::Rent::get()?).minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &token_program.key,
            ),
            &[
                self.self_account.account_info.clone(),
                payer,
                system_program,
                token_program.clone(),
            ],
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_mint(
                &token_program.key,
                &self.self_account.key(),
                &mint_authority.key(),
                freeze_authority.map(|f| f.key()),
                decimals,
            )?,
            &[
                self.self_account.account_info.clone(),
                mint_authority.into(),
                token_program,
                rent,
            ],
        )?;
        Ok(())
    }

    fn create_mint_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;
        use solana_program::{program_pack::Pack, sysvar::Sysvar};

        let rent = self.rent.clone();
        let system_program = self.system_program.clone();
        let token_program = self.self_account.token_program.clone();
        solana_program::program::invoke(
            &solana_program::system_instruction::create_account(
                &self.fee_payer.key,
                &self.self_account.key(),
                (solana_program::rent::Rent::get()?).minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &token_program.key,
            ),
            &[
                self.self_account.account_info.clone(),
                payer.into(),
                system_program,
                token_program.clone(),
            ],
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_mint(
                &token_program.key,
                &self.self_account.key(),
                &mint_authority.key(),
                freeze_authority.map(|f| f.key()),
                decimals,
            )?,
            &[
                self.self_account.account_info.clone(),
                mint_authority.into(),
                token_program,
                rent,
            ],
        )?;
        Ok(())
    }
}
