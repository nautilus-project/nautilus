struct UpdateWeaponAccounts {
    Weapon: AccountInfo,
    payer: AccountInfo,
    mint: AccountInfo,
    token_account: AccountInfo,
    system_program: AccountInfo,
}

struct UpdateWeaponArgs {
    id: u32,
    name: String,
}

#[nautilus_instruction]
struct UpdateWeaponInstruction {
    accounts: UpdateWeaponAccounts,
    args: UpdateWeaponArgs,
}

fn update_weapon(instruction: UpdateWeaponInstruction) -> ProgramResult {

    let Weapon = Weapon::new(
        instruction.args.id,
        instruction.args.name,
    );

    assert!(instruction.accounts.Weapon.lamports() == 0);

    assert!(nautilus_util::spl::validate_token_account(
        instructions.accounts.mint,
        instructions.accounts.token_account,
    ));

    nautilus_util::spl::transfer(
        instruction.token_account,
        10,
    )?;

    Weapon.validate_pda(instruction.accounts.Weapon)?;
    Weapon.update(instructions.accounts.Weapon)?;

    Ok()
}
