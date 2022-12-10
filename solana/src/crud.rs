use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult, 
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    system_program,
    sysvar::{
        rent::Rent,
        Sysvar,
    },
};
use std::io::Result;


pub trait NautilusAccount: borsh::ser::BorshSerialize {

    type NautilusCrudObject;

    const TABLE_NAME: &'static str;
    const PRIMARY_KEY: &'static str;
    const AUTO_INCREMENT: bool;

    fn span(&self) -> Result<usize> {
        Ok((self.try_to_vec()?).len())
    }

    fn lamports_required(&self) -> Result<u64> {
        Ok((Rent::get().unwrap()).minimum_balance(self.span()?))
    }
    
    fn size(&self) -> Result<u64> {
        Ok(self.span()?.try_into().unwrap())
    }

    fn address(&self, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[ Self::TABLE_NAME.as_bytes().as_ref() ],
            program_id
        )
    }

    // fn new_inner() -> Self;
    // fn update_inner(&mut self) -> Self;
}

pub trait NautilusAllocate: borsh::ser::BorshSerialize + NautilusAccount {

    fn allocate<'a>(
        &self, 
        new_account: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {
        invoke_signed(
            &system_instruction::allocate(
                &new_account.key,
                self.size()?,
            ),
            &[ 
                new_account.clone(), 
                system_program.clone()
            ],
            &[&[
                Self::TABLE_NAME.as_bytes().as_ref(),
                // self.id.to_le_bytes().as_ref(),
            ]]
        )
    }
}

pub trait NautilusCreate: borsh::ser::BorshSerialize + NautilusAccount {

    fn create<'a>(
        &self, 
        program_id: &Pubkey,
        new_account: AccountInfo<'a>,
        payer_account: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {
        invoke_signed(
            &system_instruction::create_account(
                &payer_account.key,
                &new_account.key,
                self.lamports_required()?,
                self.size()?,
                program_id,
            ),
            &[
                payer_account.clone(), 
                new_account.clone(), 
                system_program.clone()
            ],
            &[&[
                Self::TABLE_NAME.as_bytes().as_ref(),
                // self.id.to_le_bytes().as_ref(),
            ]]
        )?;
        self.serialize(
            &mut &mut new_account.data.borrow_mut()[..]
        )?;
        Ok(())
    }
}

pub trait NautilusUpdate: borsh::ser::BorshSerialize + NautilusAccount {

    fn update(&self, account: &AccountInfo) -> ProgramResult {
        self.serialize(
            &mut &mut account.data.borrow_mut()[..]
        )?;
        Ok(())
    }
}

pub trait NautilusDelete: borsh::ser::BorshSerialize + NautilusAccount {

    fn close(&self, account: &AccountInfo, recipient: &AccountInfo) -> ProgramResult {
        let dest_starting_lamports = recipient.lamports();
        **recipient.lamports.borrow_mut() =
            dest_starting_lamports.checked_add(account.lamports()).unwrap();
        **account.lamports.borrow_mut() = 0;
        account.assign(&system_program::ID);
        account.realloc(0, false).map_err(Into::into)
    }
}
