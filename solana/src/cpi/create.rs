use mpl_token_metadata::state::{Collection, CollectionDetails, Creator, Uses};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

pub fn create_account(
    payer: &Pubkey,
    new_account: &Pubkey,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    invoke(
        &system_instruction::create_account(payer, new_account, lamports, space, owner),
        accounts,
    )
}

pub fn create_pda<'a>(
    payer: &Pubkey,
    new_account: &Pubkey,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
    accounts: &[AccountInfo],
    seeds: &[&[u8]],
    bump: u8,
) -> ProgramResult {
    invoke_signed(
        &system_instruction::create_account(payer, new_account, lamports, space, owner),
        accounts,
        &[seeds, &[&[bump]]],
    )
}

pub fn create_mint(
    payer: &Pubkey,
    mint: &Pubkey,
    lamports: u64,
    space: u64,
    token_program: &Pubkey,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    accounts_for_create: &[AccountInfo],
    accounts_for_init: &[AccountInfo],
) -> ProgramResult {
    create_account(
        payer,
        mint,
        lamports,
        space,
        token_program,
        accounts_for_create,
    )?;
    invoke(
        &spl_token::instruction::initialize_mint(
            token_program,
            mint,
            mint_authority,
            freeze_authority,
            decimals,
        )?,
        accounts_for_init,
    )
}

pub fn create_metadata(
    token_metadata_program_id: Pubkey,
    metadata: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    title: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    collection: Option<Collection>,
    uses: Option<Uses>,
    collection_details: Option<CollectionDetails>,
    accounts: &[AccountInfo],
) -> ProgramResult {
    invoke(
        &mpl_token_metadata::instruction::create_metadata_accounts_v3(
            token_metadata_program_id,
            metadata,
            mint,
            mint_authority,
            payer,
            update_authority,
            title,
            symbol,
            uri,
            creators,
            seller_fee_basis_points,
            update_authority_is_signer,
            is_mutable,
            collection,
            uses,
            collection_details,
        ),
        accounts,
    )
}

pub fn create_associated_token_account(
    payer: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer,
            owner,
            mint,
            token_program,
        ),
        accounts,
    )
}
