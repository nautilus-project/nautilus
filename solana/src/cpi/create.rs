use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
};

use crate::{Create, NautilusAccountInfo, NautilusSigner};

pub fn create_account<'a>(
    new_account: impl NautilusSigner,
    owner: &Pubkey,
    payer: impl NautilusSigner,
    system_program: Box<AccountInfo<'a>>,
) -> ProgramResult {
    invoke(
        &system_instruction::create_account(
            payer.key(),
            new_account.key(),
            new_account.required_rent()?,
            new_account.size(),
            owner,
        ),
        &[payer.into(), new_account.into(), *system_program],
    )
}

// pub fn create_pda<'a>(
//     new_account: impl NautilusRecord<'a>,
//     owner: &Pubkey,
//     payer: impl NautilusSigner<'a>,
//     system_program: Box<AccountInfo<'a>>,
//     data: impl NautilusData,
// ) -> ProgramResult {
//     let (_, bump) = new_account.pda();
//     let seeds = new_account.seeds();
//     invoke_signed(
//         &system_instruction::create_account(
//             payer.key(),
//             new_account.key(),
//             new_account.required_rent()?,
//             new_account.size(),
//             owner,
//         ),
//         &[payer.into(), new_account.clone().into(), *system_program],
//         &[&seeds, &[&[bump]]],
//     )?;
//     data.serialize(&mut &mut new_account.into_account_info().data.borrow_mut()[..])?;
//     Ok(())
// }

// pub fn create_mint<'a>(
//     mint: Create<'a, Mint<'a>>,
//     decimals: u8,
//     mint_authority: impl NautilusAccountInfo<'a>,
//     freeze_authority: Option<impl NautilusAccountInfo<'a>>,
//     payer: impl NautilusSigner<'a>,
//     rent: Box<AccountInfo<'a>>,
//     system_program: Box<AccountInfo<'a>>,
//     token_program: Box<AccountInfo<'a>>,
// ) -> ProgramResult {
//     create_account(mint.clone(), &token_program.key, payer, system_program)?;
//     invoke(
//         &spl_token::instruction::initialize_mint(
//             &token_program.key,
//             &mint.key(),
//             &mint_authority.key(),
//             freeze_authority.map(|f| f.key()),
//             decimals,
//         )?,
//         &[mint.into(), mint_authority.into(), *token_program, *rent],
//     )
// }

// pub fn create_metadata<'a>(
//     metadata: Create<'a, Metadata<'a>>,
//     title: String,
//     symbol: String,
//     uri: String,
//     mint: impl NautilusAccountInfo<'a>,
//     mint_authority: impl NautilusSigner<'a>,
//     update_authority: impl NautilusAccountInfo<'a>,
//     payer: impl NautilusSigner<'a>,
//     rent: Box<AccountInfo<'a>>,
//     token_metadata_program: Box<AccountInfo<'a>>,
// ) -> ProgramResult {
//     invoke(
//         &mpl_token_metadata::instruction::create_metadata_accounts_v3(
//             *token_metadata_program.key,
//             *metadata.key(),
//             *mint.key(),
//             *mint_authority.key(),
//             *payer.key(),
//             *update_authority.key(),
//             title,
//             symbol,
//             uri,
//             None,
//             0,
//             true,
//             false,
//             None,
//             None,
//             None,
//         ),
//         &[
//             metadata.into(),
//             mint.into(),
//             mint_authority.into(),
//             payer.into(),
//             *token_metadata_program,
//             *rent,
//         ],
//     )
// }

// pub fn create_associated_token_account<'a>(
//     new_account: AssociatedTokenAccount<'a>,
//     mint: impl NautilusAccountInfo<'a>,
//     owner: impl NautilusAccountInfo<'a>,
//     payer: impl NautilusSigner<'a>,
//     system_program: Box<AccountInfo<'a>>,
//     token_program: Box<AccountInfo<'a>>,
//     associated_token_program: Box<AccountInfo<'a>>,
// ) -> ProgramResult {
//     invoke(
//         &spl_associated_token_account::instruction::create_associated_token_account(
//             payer.key(),
//             owner.key(),
//             mint.key(),
//             token_program.key,
//         ),
//         &[
//             mint.into(),
//             new_account.into(),
//             owner.into(),
//             payer.into(),
//             *system_program,
//             *token_program,
//             *associated_token_program,
//         ],
//     )
// }
