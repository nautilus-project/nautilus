pub use mpl_token_metadata::state::{
    Edition as EditionState, MasterEditionV2 as MasterEditionState, TokenMetadataAccount,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::NautilusError, NautilusAccountInfo};

/// The Nautilus object representing an Edition of an NFT.
///
/// The underlying account - designated in field `account_info` - is the edition account.
///
/// We also include the read-only Token Metadata Program for any CPI operations necessary, since we do not
/// own this account.
#[derive(Clone)]
pub struct Edition<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_metadata_program: Box<AccountInfo<'a>>,
    pub rent: Box<AccountInfo<'a>>,
    pub data: EditionState,
}

impl<'a> Edition<'a> {
    /// Instantiate a new `Edition` without loading the account inner data from on-chain.
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
    ) -> Self {
        Self {
            account_info,
            token_metadata_program,
            rent,
            data: EditionState::default(),
        }
    }

    /// Instantiate a new `Edition` and load the account inner data from on-chain.
    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data = match EditionState::safe_deserialize(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    String::from("token_metadata"),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => state_data,
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    String::from("token_metadata"),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            account_info,
            token_metadata_program,
            rent,
            data,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for Edition<'a> {
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
        Ok(EditionState::size())
    }
}

/// The Nautilus object representing an MasterEdition of an NFT.
///
/// The underlying account - designated in field `account_info` - is the edition account.
///
/// We also include the read-only Token Metadata Program for any CPI operations necessary, since we do not
/// own this account.
#[derive(Clone)]
pub struct MasterEdition<'a> {
    pub account_info: Box<AccountInfo<'a>>,
    pub token_metadata_program: Box<AccountInfo<'a>>,
    pub rent: Box<AccountInfo<'a>>,
    pub data: MasterEditionState,
}

impl<'a> MasterEdition<'a> {
    /// Instantiate a new `MasterEdition` without loading the account inner data from on-chain.
    pub fn new(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
    ) -> Self {
        Self {
            account_info,
            token_metadata_program,
            rent,
            data: MasterEditionState::default(),
        }
    }

    /// Instantiate a new `MasterEdition` and load the account inner data from on-chain.
    pub fn load(
        account_info: Box<AccountInfo<'a>>,
        token_metadata_program: Box<AccountInfo<'a>>,
        rent: Box<AccountInfo<'a>>,
    ) -> Result<Self, ProgramError> {
        let data =
            match MasterEditionState::safe_deserialize(match &account_info.try_borrow_data() {
                Ok(acct_data) => acct_data,
                Err(_) => {
                    return Err(NautilusError::LoadDataFailed(
                        String::from("token_metadata"),
                        account_info.key.to_string(),
                    )
                    .into())
                }
            }) {
                Ok(state_data) => state_data,
                Err(_) => {
                    return Err(NautilusError::DeserializeDataFailed(
                        String::from("token_metadata"),
                        account_info.key.to_string(),
                    )
                    .into())
                }
            };
        Ok(Self {
            account_info,
            token_metadata_program,
            rent,
            data,
        })
    }
}

impl<'a> NautilusAccountInfo<'a> for MasterEdition<'a> {
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
        Ok(MasterEditionState::size())
    }
}
