#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct AssociatedTokenAccount<'a> {
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub token_program: solana_program::account_info::AccountInfo<'a>,
    pub associated_token_program: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for AssociatedTokenAccount<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::objects::properties::NautilusAccountInfo<'a> for AssociatedTokenAccount<'a> {
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

impl<'a> crate::objects::properties::tokens::NautilusCreateAssociatedTokenAccount<'a>
    for crate::objects::properties::create::Create<'a, AssociatedTokenAccount<'a>>
{
    fn create(&self, mint: super::mint::Mint<'a>) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;
        let payer = self.fee_payer.clone();
        let system_program = self.system_program.clone();
        solana_program::program::invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                payer.key,
                self.key(),
                mint.key(),
                self.self_account.token_program.key,
            ),
            &[
                mint.into(),
                self.self_account.account_info.clone(),
                payer,
                system_program,
                self.self_account.token_program.clone(),
                self.self_account.associated_token_program.clone(),
            ],
        )
    }

    fn create_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        mint: super::mint::Mint<'a>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;
        let system_program = self.system_program.clone();
        solana_program::program::invoke(
            &spl_associated_token_account::instruction::create_associated_token_account(
                payer.key(),
                self.key(),
                mint.key(),
                self.self_account.token_program.key,
            ),
            &[
                mint.into(),
                self.self_account.account_info.clone(),
                payer.into(),
                system_program,
                self.self_account.token_program.clone(),
                self.self_account.associated_token_program.clone(),
            ],
        )
    }
}
