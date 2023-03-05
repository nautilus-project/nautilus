// Normal

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransferTokensArgs {
    pub quantity: u64,
}

pub fn transfer_tokens(accounts: &[AccountInfo], args: TransferTokensArgs) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let from_associated_token_account = next_account_info(accounts_iter)?;
    let to_associated_token_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    if to_associated_token_account.lamports() == 0 {
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                &payer.key,
                &recipient.key,
                &mint_account.key,
            ),
            &[
                mint_account.clone(),
                to_associated_token_account.clone(),
                recipient.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    };

    invoke(
        &token_instruction::transfer(
            &token_program.key,
            &from_associated_token_account.key,
            &to_associated_token_account.key,
            &owner.key,
            &[&owner.key, &recipient.key],
            args.quantity,
        )?,
        &[
            mint_account.clone(),
            from_associated_token_account.clone(),
            to_associated_token_account.clone(),
            owner.clone(),
            recipient.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}

// Spec

pub fn transfer_tokens(token: Token, to: Wallet, amount: u64) -> ProgramResult {
    token.transfer(to, amount)?;

    Ok(())
}
