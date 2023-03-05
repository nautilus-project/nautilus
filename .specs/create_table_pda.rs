// Normal

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}

pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let address_info_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_span = (address_info.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke(
        &system_instruction::create_account(
            &payer.key,
            &address_info_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;
    Ok(())
}

// Spec

#[derive(NautilusTable)]
pub struct AddressInfo {
    #[primary_key(autoincrement = true)]
    pub id: u8,
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
    #[authority(sign_to_create = true)]
    pub authority: Pubkey,
}

fn create_address_info(
    record: Create<AddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
    authority: Pubkey,
) -> ProgramResult {
    record.create(name, house_number, street, city, authority)?;
    Ok(())
}
