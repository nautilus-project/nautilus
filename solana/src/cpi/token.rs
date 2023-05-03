//! Cross-Program invocations to the Token Program (legacy).
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey,
};
use spl_token::instruction::AuthorityType;

use crate::{NautilusAccountInfo, NautilusMut, NautilusSigner};

/// Approves a delegate.  A delegate is given the authority over tokens on
/// behalf of the source account's owner.
pub fn approve<'a>(
    token_program_id: &Pubkey,
    source_account: impl NautilusMut<'a>,
    delegate: impl NautilusAccountInfo<'a>,
    source_owner: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
) -> ProgramResult {
    let mut accounts = vec![
        *source_account.account_info(),
        *delegate.account_info(),
        *source_owner.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::approve(
            token_program_id,
            source_account.key(),
            delegate.key(),
            source_owner.key(),
            signer_pubkeys.as_slice(),
            amount,
        )?,
        &accounts,
    )
}

/// Approves a delegate.  A delegate is given the authority over tokens on
/// behalf of the source account's owner.
///
/// This instruction differs from Approve in that the token mint and
/// decimals value is checked by the caller.  This may be useful when
/// creating transactions offline or within a hardware wallet.
#[allow(clippy::too_many_arguments)]
pub fn approve_checked<'a>(
    token_program_id: &Pubkey,
    source_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    delegate: impl NautilusAccountInfo<'a>,
    source_owner: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let mut accounts = vec![
        *source_account.account_info(),
        *delegate.account_info(),
        *source_owner.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::approve_checked(
            token_program_id,
            source_account.key(),
            mint.key(),
            delegate.key(),
            source_owner.key(),
            signer_pubkeys.as_slice(),
            amount,
            decimals,
        )?,
        &accounts,
    )
}

/// Burns tokens by removing them from an account.
pub fn burn<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
) -> ProgramResult {
    let mut accounts = vec![
        *token_account.account_info(),
        *mint.account_info(),
        *authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::burn(
            token_program_id,
            token_account.key(),
            mint.key(),
            authority.key(),
            signer_pubkeys.as_slice(),
            amount,
        )?,
        &accounts,
    )
}

/// Burns tokens by removing them from an account.  `BurnChecked` does not
/// support accounts associated with the native mint, use `CloseAccount`
/// instead.
///
/// This instruction differs from Burn in that the decimals value is checked
/// by the caller. This may be useful when creating transactions offline or
/// within a hardware wallet.
pub fn burn_checked<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let mut accounts = vec![
        *token_account.account_info(),
        *mint.account_info(),
        *authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::burn_checked(
            token_program_id,
            token_account.key(),
            mint.key(),
            authority.key(),
            signer_pubkeys.as_slice(),
            amount,
            decimals,
        )?,
        &accounts,
    )
}

/// Close an account by transferring all its SOL to the destination account.
/// Non-native accounts may only be closed if its token amount is zero.
pub fn close_account<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
    destination: impl NautilusMut<'a>,
    authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
) -> ProgramResult {
    let mut accounts = vec![
        *token_account.account_info(),
        *destination.account_info(),
        *authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::close_account(
            token_program_id,
            token_account.key(),
            destination.key(),
            authority.key(),
            signer_pubkeys.as_slice(),
        )?,
        &accounts,
    )
}

/// Freeze an Initialized account using the Mint's freeze_authority (if
/// set).
pub fn freeze_account<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    freeze_authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
) -> ProgramResult {
    let mut accounts = vec![
        *token_account.account_info(),
        *mint.account_info(),
        *freeze_authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::freeze_account(
            token_program_id,
            token_account.key(),
            mint.key(),
            freeze_authority.key(),
            signer_pubkeys.as_slice(),
        )?,
        &accounts,
    )
}

/// Initializes a new account to hold tokens.  If this account is associated
/// with the native mint then the token balance of the initialized account
/// will be equal to the amount of SOL in the account. If this account is
/// associated with another mint, that mint must be initialized before this
/// command can succeed.
#[allow(clippy::boxed_local)]
pub fn initialize_account<'a>(
    token_program_id: &Pubkey,
    new_token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    authority: impl NautilusAccountInfo<'a>,
    rent: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_account(
            token_program_id,
            new_token_account.key(),
            mint.key(),
            authority.key(),
        )?,
        &[
            *new_token_account.account_info(),
            *mint.account_info(),
            *authority.account_info(),
            *rent,
        ],
    )
}

/// Like InitializeAccount, but the owner pubkey is passed via instruction data
/// rather than the accounts list. This variant may be preferable when using
/// Cross Program Invocation from an instruction that does not need the owner's
/// `AccountInfo` otherwise.
#[allow(clippy::boxed_local)]
pub fn initialize_account2<'a>(
    token_program_id: &Pubkey,
    new_token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    authority: &Pubkey,
    rent: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_account2(
            token_program_id,
            new_token_account.key(),
            mint.key(),
            authority,
        )?,
        &[
            *new_token_account.account_info(),
            *mint.account_info(),
            *rent,
        ],
    )
}

/// Like InitializeAccount2, but does not require the Rent sysvar to be provided
pub fn initialize_account3<'a>(
    token_program_id: &Pubkey,
    new_token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    authority: &Pubkey,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_account3(
            token_program_id,
            new_token_account.key(),
            mint.key(),
            authority,
        )?,
        &[*new_token_account.account_info(), *mint.account_info()],
    )
}

/// Initialize the Immutable Owner extension for the given token account
///
/// Fails if the account has already been initialized, so must be called before
/// `InitializeAccount`.
pub fn initialize_immutable_owner<'a>(
    token_program_id: &Pubkey,
    new_token_account: impl NautilusMut<'a>,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_immutable_owner(
            token_program_id,
            new_token_account.key(),
        )?,
        &[*new_token_account.account_info()],
    )
}

/// Initializes a new mint and optionally deposits all the newly minted
/// tokens in an account.
#[allow(clippy::boxed_local)]
pub fn initialize_mint<'a>(
    token_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    rent: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_mint(
            token_program_id,
            mint.key(),
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        &[*mint.account_info(), *rent],
    )
}

/// Like InitializeMint, but does not require the Rent sysvar to be provided
pub fn initialize_mint2<'a>(
    token_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::initialize_mint2(
            token_program_id,
            mint.key(),
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        &[*mint.account_info()],
    )
}

/// Initializes a multisignature account with N provided signers.
///
/// Multisignature accounts can used in place of any single owner/delegate
/// accounts in any token instruction that require an owner/delegate to be
/// present.  The variant field represents the number of signers (M)
/// required to validate this multisignature account.
#[allow(clippy::boxed_local)]
pub fn initialize_multisig<'a>(
    token_program_id: &Pubkey,
    multisig_account: impl NautilusMut<'a>,
    multisigs: Vec<impl NautilusAccountInfo<'a>>,
    m: u8,
    rent: Box<AccountInfo<'a>>,
) -> ProgramResult {
    let mut accounts = vec![*multisig_account.account_info(), *rent];
    let signer_pubkeys = append_multisig_accounts_and_return_keys(&mut accounts, multisigs);
    invoke(
        &spl_token::instruction::initialize_multisig(
            token_program_id,
            multisig_account.key(),
            signer_pubkeys.as_slice(),
            m,
        )?,
        &accounts,
    )
}

/// Initializes a multisignature account with N provided signers.
///
/// Multisignature accounts can used in place of any single owner/delegate
/// accounts in any token instruction that require an owner/delegate to be
/// present.  The variant field represents the number of signers (M)
/// required to validate this multisignature account.
pub fn initialize_multisig2<'a>(
    token_program_id: &Pubkey,
    multisig_account: impl NautilusMut<'a>,
    multisigs: Vec<impl NautilusAccountInfo<'a>>,
    m: u8,
) -> ProgramResult {
    let mut accounts = vec![*multisig_account.account_info()];
    let signer_pubkeys = append_multisig_accounts_and_return_keys(&mut accounts, multisigs);
    invoke(
        &spl_token::instruction::initialize_multisig2(
            token_program_id,
            multisig_account.key(),
            signer_pubkeys.as_slice(),
            m,
        )?,
        &accounts,
    )
}

/// Mints new tokens to an account.  The native mint does not support
/// minting.
pub fn mint_to<'a>(
    token_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    recipient: impl NautilusMut<'a>,
    mint_authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
) -> ProgramResult {
    let mut accounts = vec![
        *mint.account_info(),
        *recipient.account_info(),
        *mint_authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::mint_to(
            token_program_id,
            mint.key(),
            recipient.key(),
            mint_authority.key(),
            signer_pubkeys.as_slice(),
            amount,
        )?,
        &accounts,
    )
}

/// Mints new tokens to an account.  The native mint does not support
/// minting.
///
/// This instruction differs from MintTo in that the decimals value is
/// checked by the caller.  This may be useful when creating transactions
/// offline or within a hardware wallet.
pub fn mint_to_checked<'a>(
    token_program_id: &Pubkey,
    mint: impl NautilusMut<'a>,
    recipient: impl NautilusMut<'a>,
    mint_authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let mut accounts = vec![
        *mint.account_info(),
        *recipient.account_info(),
        *mint_authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::mint_to_checked(
            token_program_id,
            mint.key(),
            recipient.key(),
            mint_authority.key(),
            signer_pubkeys.as_slice(),
            amount,
            decimals,
        )?,
        &accounts,
    )
}

/// Revokes the delegate's authority.
pub fn revoke<'a>(
    token_program_id: &Pubkey,
    source_account: impl NautilusMut<'a>,
    source_owner: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
) -> ProgramResult {
    let mut accounts = vec![*source_account.account_info(), *source_owner.account_info()];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::revoke(
            token_program_id,
            source_account.key(),
            source_owner.key(),
            signer_pubkeys.as_slice(),
        )?,
        &accounts,
    )
}

/// Sets a new authority of a mint or account.
pub fn set_authority<'a>(
    token_program_id: &Pubkey,
    mint_or_account: impl NautilusMut<'a>,
    new_authority: Option<&Pubkey>,
    authority_type: AuthorityType,
    current_authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
) -> ProgramResult {
    let mut accounts = vec![
        *mint_or_account.account_info(),
        *current_authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::set_authority(
            token_program_id,
            mint_or_account.key(),
            new_authority,
            authority_type,
            current_authority.key(),
            signer_pubkeys.as_slice(),
        )?,
        &accounts,
    )
}

/// Given a wrapped / native token account (a token account containing SOL)
/// updates its amount field based on the account's underlying `lamports`.
/// This is useful if a non-wrapped SOL account uses `system_instruction::transfer`
/// to move lamports to a wrapped token account, and needs to have its token
/// `amount` field updated.
pub fn sync_native<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::sync_native(token_program_id, token_account.key())?,
        &[*token_account.account_info()],
    )
}

/// Thaw a Frozen account using the Mint's freeze_authority (if set).
pub fn thaw_account<'a>(
    token_program_id: &Pubkey,
    token_account: impl NautilusMut<'a>,
    mint: impl NautilusAccountInfo<'a>,
    freeze_authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
) -> ProgramResult {
    let mut accounts = vec![
        *token_account.account_info(),
        *mint.account_info(),
        *freeze_authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::thaw_account(
            token_program_id,
            token_account.key(),
            mint.key(),
            freeze_authority.key(),
            signer_pubkeys.as_slice(),
        )?,
        &accounts,
    )
}

/// Mints new tokens to an account.  The native mint does not support
/// minting.
pub fn transfer<'a>(
    token_program_id: &Pubkey,
    from: impl NautilusMut<'a>,
    to: impl NautilusMut<'a>,
    authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
) -> ProgramResult {
    let mut accounts = vec![
        *from.account_info(),
        *to.account_info(),
        *authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::transfer(
            token_program_id,
            from.key(),
            to.key(),
            authority.key(),
            signer_pubkeys.as_slice(),
            amount,
        )?,
        &accounts,
    )
}

/// Transfers tokens from one account to another either directly or via a
/// delegate.  If this account is associated with the native mint then equal
/// amounts of SOL and Tokens will be transferred to the destination
/// account.
///
/// This instruction differs from Transfer in that the token mint and
/// decimals value is checked by the caller.  This may be useful when
/// creating transactions offline or within a hardware wallet.
#[allow(clippy::too_many_arguments)]
pub fn transfer_checked<'a>(
    token_program_id: &Pubkey,
    mint: impl NautilusAccountInfo<'a>,
    from: impl NautilusMut<'a>,
    to: impl NautilusMut<'a>,
    authority: impl NautilusSigner<'a>,
    multisigs: Option<Vec<impl NautilusSigner<'a>>>,
    amount: u64,
    decimals: u8,
) -> ProgramResult {
    let mut accounts = vec![
        *mint.account_info(),
        *from.account_info(),
        *to.account_info(),
        *authority.account_info(),
    ];
    let signer_pubkeys = match multisigs {
        Some(sigs) => append_multisig_accounts_and_return_keys(&mut accounts, sigs),
        None => vec![],
    };
    invoke(
        &spl_token::instruction::transfer_checked(
            token_program_id,
            from.key(),
            mint.key(),
            to.key(),
            authority.key(),
            signer_pubkeys.as_slice(),
            amount,
            decimals,
        )?,
        &accounts,
    )
}

/// Helper function to build lists of pubkeys and accounts from multisig option.
fn append_multisig_accounts_and_return_keys<'a>(
    accounts: &mut Vec<AccountInfo<'a>>,
    multisigs: Vec<impl NautilusAccountInfo<'a>>,
) -> Vec<&'a Pubkey> {
    multisigs
        .iter()
        .map(|m| {
            accounts.push(*m.account_info());
            m.key()
        })
        .collect()
}
