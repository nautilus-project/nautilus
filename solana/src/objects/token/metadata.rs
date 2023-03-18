#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Metadata<'a> {
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub token_metadata_program: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Metadata<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a> crate::properties::NautilusAccountInfo<'a> for Metadata<'a> {
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

impl<'a> crate::properties::NautilusCreateMetadata<'a>
    for crate::properties::Create<'a, Metadata<'a>>
{
    fn create_metadata<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: crate::token::Mint<'a>,
        mint_authority: T,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let metadata = self.self_account.clone();
        let payer = self.fee_payer.clone();
        let rent = self.rent.clone();
        let token_metadata_program = metadata.token_metadata_program.clone();
        solana_program::program::invoke(
            &mpl_token_metadata::instruction::create_metadata_accounts_v3(
                *token_metadata_program.key,
                *crate::properties::NautilusAccountInfo::key(&metadata),
                *crate::properties::NautilusAccountInfo::key(&mint),
                *mint_authority.key(),
                *payer.key,
                *update_authority.key(),
                title,
                symbol,
                uri,
                None,
                0,
                true,
                false,
                None,
                None,
                None,
            ),
            &[
                metadata.into(),
                mint.account_info.clone(),
                mint_authority.into(),
                payer,
                token_metadata_program,
                rent,
            ],
        )
    }

    fn create_metadata_with_payer<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: crate::token::Mint<'a>,
        mint_authority: T,
        update_authority: T,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let metadata = self.self_account.clone();
        let rent = self.rent.clone();
        let token_metadata_program = metadata.token_metadata_program.clone();
        solana_program::program::invoke(
            &mpl_token_metadata::instruction::create_metadata_accounts_v3(
                *token_metadata_program.key,
                *crate::properties::NautilusAccountInfo::key(&metadata),
                *crate::properties::NautilusAccountInfo::key(&mint),
                *mint_authority.key(),
                *payer.key(),
                *update_authority.key(),
                title,
                symbol,
                uri,
                None,
                0,
                true,
                false,
                None,
                None,
                None,
            ),
            &[
                metadata.into(),
                mint.account_info.clone(),
                mint_authority.into(),
                payer.into(),
                token_metadata_program,
                rent,
            ],
        )
    }
}
