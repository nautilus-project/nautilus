#[derive(Clone)]
pub struct Create<'a, T: super::NautilusAccountInfo<'a>> {
    pub fee_payer: solana_program::account_info::AccountInfo<'a>,
    pub owner: solana_program::account_info::AccountInfo<'a>,
    pub system_program: solana_program::account_info::AccountInfo<'a>,
    pub rent: solana_program::account_info::AccountInfo<'a>,
    pub self_account: T,
}

pub trait NautilusCreate<'a> {
    fn create(&self) -> solana_program::entrypoint::ProgramResult;
    fn create_with_payer<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult;
}

impl<'a, T: super::NautilusAccountInfo<'a>> solana_program::account_info::IntoAccountInfo<'a>
    for Create<'a, T>
{
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.self_account.into_account_info()
    }
}

impl<'a, T: super::NautilusAccountInfo<'a>> super::NautilusAccountInfo<'a> for Create<'a, T> {
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
        self.self_account.key()
    }

    fn is_signer(&self) -> bool {
        self.self_account.is_signer()
    }

    fn is_writable(&self) -> bool {
        self.self_account.is_writable()
    }

    fn lamports(&self) -> u64 {
        self.self_account.lamports()
    }

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.self_account.mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.self_account.owner()
    }

    fn span(&self) -> usize {
        self.self_account.span()
    }
}

impl<'a, T: super::NautilusTransferLamports<'a>> crate::properties::NautilusTransferLamports<'a>
    for Create<'a, T>
{
    fn transfer_lamports<U: super::NautilusAccountInfo<'a>>(
        self,
        to: U,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        self.self_account.transfer_lamports(to, amount)
    }
}
