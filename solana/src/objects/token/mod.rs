pub mod associated_token;
pub mod metadata;
pub mod mint;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Token<'a> {
    pub mint: solana_program::account_info::AccountInfo<'a>,
    pub metadata: solana_program::account_info::AccountInfo<'a>,
    pub token_program: solana_program::account_info::AccountInfo<'a>,
    pub token_metadata_program: solana_program::account_info::AccountInfo<'a>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Token<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.mint
    }
}

impl<'a> crate::objects::properties::NautilusAccountInfo<'a> for Token<'a> {
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
        self.mint.key
    }

    fn is_signer(&self) -> bool {
        self.mint.is_signer
    }

    fn is_writable(&self) -> bool {
        self.mint.is_writable
    }

    fn lamports(&self) -> u64 {
        self.mint.lamports()
    }

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.mint.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.mint.owner
    }

    fn span(&self) -> usize {
        self.mint.data_len()
    }
}

impl<'a> crate::objects::properties::tokens::NautilusCreateMint<'a>
    for crate::objects::properties::create::Create<'a, Token<'a>>
{
    fn create_mint<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult {
        let mint = mint::Mint {
            account_info: self.self_account.mint.to_owned(),
            token_program: self.self_account.token_program.to_owned(),
        };
        let create_mint: crate::objects::properties::create::Create<mint::Mint> =
            crate::objects::properties::create::Create {
                fee_payer: self.fee_payer.to_owned(),
                owner: self.owner.to_owned(),
                system_program: self.system_program.to_owned(),
                rent: self.rent.to_owned(),
                self_account: mint.to_owned(),
            };
        create_mint.create_mint(decimals, mint_authority, freeze_authority)
    }

    fn create_mint_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let mint = mint::Mint {
            account_info: self.self_account.mint.to_owned(),
            token_program: self.self_account.token_program.to_owned(),
        };
        let create_mint: crate::objects::properties::create::Create<mint::Mint> =
            crate::objects::properties::create::Create {
                fee_payer: self.fee_payer.to_owned(),
                owner: self.owner.to_owned(),
                system_program: self.system_program.to_owned(),
                rent: self.rent.to_owned(),
                self_account: mint.to_owned(),
            };
        create_mint.create_mint_with_payer(decimals, mint_authority, freeze_authority, payer)
    }
}

impl<'a> crate::objects::properties::tokens::NautilusCreateMetadata<'a>
    for crate::objects::properties::create::Create<'a, Token<'a>>
{
    fn create_metadata<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: mint::Mint<'a>,
        mint_authority: T,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let metadata = metadata::Metadata {
            account_info: self.self_account.metadata.to_owned(),
            token_metadata_program: self.self_account.token_metadata_program.to_owned(),
        };
        let create_metadata: crate::objects::properties::create::Create<metadata::Metadata> =
            crate::objects::properties::create::Create {
                fee_payer: self.fee_payer.to_owned(),
                owner: self.owner.to_owned(),
                system_program: self.system_program.to_owned(),
                rent: self.rent.to_owned(),
                self_account: metadata.to_owned(),
            };
        create_metadata.create_metadata(title, symbol, uri, mint, mint_authority, update_authority)
    }

    fn create_metadata_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        title: String,
        symbol: String,
        uri: String,
        mint: mint::Mint<'a>,
        mint_authority: T,
        update_authority: T,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let metadata = metadata::Metadata {
            account_info: self.self_account.metadata.to_owned(),
            token_metadata_program: self.self_account.token_metadata_program.to_owned(),
        };
        let create_metadata: crate::objects::properties::create::Create<metadata::Metadata> =
            crate::objects::properties::create::Create {
                fee_payer: self.fee_payer.to_owned(),
                owner: self.owner.to_owned(),
                system_program: self.system_program.to_owned(),
                rent: self.rent.to_owned(),
                self_account: metadata.to_owned(),
            };
        create_metadata.create_metadata_with_payer(
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
        )
    }
}

impl<'a> crate::objects::properties::tokens::NautilusCreateToken<'a>
    for crate::objects::properties::create::Create<'a, Token<'a>>
{
    fn create<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult {
        crate::objects::properties::tokens::NautilusCreateMint::create_mint(
            self,
            decimals,
            mint_authority,
            freeze_authority,
        )
    }

    fn create_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        crate::objects::properties::tokens::NautilusCreateMint::create_mint_with_payer(
            self,
            decimals,
            mint_authority,
            freeze_authority,
            payer,
        )
    }

    fn create_with_metadata<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let mint = mint::Mint {
            account_info: self.self_account.mint.to_owned(),
            token_program: self.self_account.token_program.to_owned(),
        };
        crate::objects::properties::tokens::NautilusCreateMint::create_mint(
            self,
            decimals,
            mint_authority.clone(),
            freeze_authority,
        )?;
        crate::objects::properties::tokens::NautilusCreateMetadata::create_metadata(
            self,
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
        )?;
        Ok(())
    }

    fn create_with_metadata_with_payer<T: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let mint = mint::Mint {
            account_info: self.self_account.mint.to_owned(),
            token_program: self.self_account.token_program.to_owned(),
        };
        crate::objects::properties::tokens::NautilusCreateMint::create_mint_with_payer(
            self,
            decimals,
            mint_authority.clone(),
            freeze_authority,
            payer.clone(),
        )?;
        crate::objects::properties::tokens::NautilusCreateMetadata::create_metadata_with_payer(
            self,
            title,
            symbol,
            uri,
            mint,
            mint_authority,
            update_authority,
            payer,
        )?;
        Ok(())
    }
}
