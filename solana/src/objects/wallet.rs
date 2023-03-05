pub struct Wallet<'a> {
    account_info: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> From<solana_program::account_info::AccountInfo<'a>> for Wallet<'a> {
    fn from(value: solana_program::account_info::AccountInfo<'a>) -> Self {
        Self {
            account_info: value,
        }
    }
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Wallet<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::NautilusAccountInfo<'a> for Wallet<'a> {
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

impl<'a> Wallet<'a> {
    pub fn create() -> Self {
        todo!()
    }

    pub fn delete(self) -> solana_program::entrypoint::ProgramResult {
        todo!()
    }

    pub fn update() -> Self {
        todo!()
    }
}
