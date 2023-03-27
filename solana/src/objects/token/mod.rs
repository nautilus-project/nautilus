pub mod associated_token;
pub mod metadata;
pub mod mint;

pub use associated_token::*;
pub use metadata::*;
pub use mint::*;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Token<'a> {
    mint: Mint<'a>,
    metadata: Option<Metadata<'a>>,
}

impl<'a> solana_program::account_info::IntoAccountInfo<'a> for Token<'a> {
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.mint.into_account_info()
    }
}

impl<'a> crate::properties::NautilusAccountInfo<'a> for Token<'a> {
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
        self.mint.key()
    }

    fn is_signer(&self) -> bool {
        self.mint.is_signer()
    }

    fn is_writable(&self) -> bool {
        self.mint.is_writable()
    }

    fn lamports(&self) -> u64 {
        self.mint.lamports()
    }

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.mint.mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.mint.owner()
    }

    fn span(&self) -> usize {
        self.mint.span()
    }
}

impl<'a> crate::properties::NautilusCreateMint<'a> for crate::properties::Create<'a, Token<'a>> {
    fn create_mint<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult {
        let create_mint: crate::properties::Create<Mint> = crate::properties::Create {
            fee_payer: self.fee_payer.to_owned(),
            owner: self.owner.to_owned(),
            system_program: self.system_program.to_owned(),
            rent: self.rent.to_owned(),
            self_account: self.self_account.mint.to_owned(),
        };
        create_mint.create_mint(decimals, mint_authority, freeze_authority)
    }

    fn create_mint_with_payer<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let create_mint: crate::properties::Create<Mint> = crate::properties::Create {
            fee_payer: self.fee_payer.to_owned(),
            owner: self.owner.to_owned(),
            system_program: self.system_program.to_owned(),
            rent: self.rent.to_owned(),
            self_account: self.self_account.mint.to_owned(),
        };
        create_mint.create_mint_with_payer(decimals, mint_authority, freeze_authority, payer)
    }
}

impl<'a> crate::properties::NautilusCreateMetadata<'a>
    for crate::properties::Create<'a, Token<'a>>
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
        let create_metadata: crate::properties::Create<Metadata> = crate::properties::Create {
            fee_payer: self.fee_payer.to_owned(),
            owner: self.owner.to_owned(),
            system_program: self.system_program.to_owned(),
            rent: self.rent.to_owned(),
            self_account: self.self_account.metadata.to_owned().unwrap(),
        };
        create_metadata.create_metadata(title, symbol, uri, mint, mint_authority, update_authority)
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
        let create_metadata: crate::properties::Create<Metadata> = crate::properties::Create {
            fee_payer: self.fee_payer.to_owned(),
            owner: self.owner.to_owned(),
            system_program: self.system_program.to_owned(),
            rent: self.rent.to_owned(),
            self_account: self.self_account.metadata.to_owned().unwrap(),
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

impl<'a> crate::properties::NautilusCreateToken<'a> for crate::properties::Create<'a, Token<'a>> {
    fn create<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
    ) -> solana_program::entrypoint::ProgramResult {
        crate::NautilusCreateMint::create_mint(self, decimals, mint_authority, freeze_authority)
    }

    fn create_with_payer<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        payer: T,
    ) -> solana_program::entrypoint::ProgramResult {
        crate::NautilusCreateMint::create_mint_with_payer(
            self,
            decimals,
            mint_authority,
            freeze_authority,
            payer,
        )
    }

    fn create_with_metadata<T: crate::properties::NautilusAccountInfo<'a>>(
        &self,
        decimals: u8,
        mint_authority: T,
        freeze_authority: Option<T>,
        title: String,
        symbol: String,
        uri: String,
        update_authority: T,
    ) -> solana_program::entrypoint::ProgramResult {
        let mint = self.self_account.mint.to_owned();
        crate::NautilusCreateMint::create_mint(
            self,
            decimals,
            mint_authority.clone(),
            freeze_authority,
        )?;
        crate::NautilusCreateMetadata::create_metadata(
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

    fn create_with_metadata_with_payer<T: crate::properties::NautilusAccountInfo<'a>>(
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
        let mint = self.self_account.mint.to_owned();
        crate::NautilusCreateMint::create_mint_with_payer(
            self,
            decimals,
            mint_authority.clone(),
            freeze_authority,
            payer.clone(),
        )?;
        crate::NautilusCreateMetadata::create_metadata_with_payer(
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
