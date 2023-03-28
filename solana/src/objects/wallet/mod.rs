#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Wallet<'a> {
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub system_program: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Wallet<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::objects::properties::NautilusAccountInfo<'a> for Wallet<'a> {
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

impl<'a> crate::objects::properties::NautilusTransferLamports<'a> for Wallet<'a> {
    fn transfer_lamports<T: crate::objects::properties::NautilusAccountInfo<'a> + 'a>(
        self,
        to: T,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info;
        let system_program = self.system_program;
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(from.key, to.key(), amount),
            &[from.into(), to.into(), system_program.into()],
        )
    }
}

impl<'a> crate::objects::properties::create::NautilusCreate<'a>
    for crate::objects::properties::create::Create<'a, Wallet<'a>>
{
    fn create(&self) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;

        let payer = self.fee_payer.clone();
        let system_program = self.system_program.clone();
        solana_program::program::invoke(
            &solana_program::system_instruction::create_account(
                payer.key,
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer,
                self.self_account.account_info.clone(),
                system_program,
            ],
        )
    }

    fn create_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::NautilusAccountInfo;

        let system_program = self.system_program.clone();
        solana_program::program::invoke(
            &solana_program::system_instruction::create_account(
                payer.key(),
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer.into(),
                self.self_account.account_info.clone(),
                system_program,
            ],
        )
    }
}
