// Normal

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateTokenArgs {
    pub token_title: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub token_decimals: u8,
}

fn create_token(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let args = CreateTokenArgs::try_from_slice(instruction_data)?;

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;

    invoke(
        &system_instruction::create_account(
            &payer.key,
            &mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    invoke(
        &token_instruction::initialize_mint(
            &token_program.key,
            &mint_account.key,
            &mint_authority.key,
            Some(&mint_authority.key),
            args.token_decimals,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    invoke(
        &mpl_instruction::create_metadata_accounts_v3(
            *token_metadata_program.key,
            *metadata_account.key,
            *mint_account.key,
            *mint_authority.key,
            *payer.key,
            *mint_authority.key,
            args.token_title,
            args.token_symbol,
            args.token_uri,
            None,
            0,
            true,
            false,
            None,
            None,
            None,
        ),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
    )?;

    Ok(())
}

// Spec

fn create_token(
    new_token: Create<Token>,
    decimals: u64,
    title: String,
    symbol: String,
    uri: String,
) -> ProgramResult {
    new_token.create(decimals)?;
    new_token.set_metadata(title, symbol, uri)?;
    // or
    new_token.create_with_metadata(decimals, title, symbol, uri)?;
    Ok(())
}
