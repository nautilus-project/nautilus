use borsh::{ BorshDeserialize, BorshSerialize };
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

pub trait NautilusProgram {
    fn entrypoint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult;
}

pub trait NautilusAccount: borsh::ser::BorshSerialize {

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

    fn accounts(&self) -> &[AccountInfo];
    fn address(&self) -> (Pubkey, u8);
    fn seeds(&self) -> &[&[&[u8]]];
}

pub trait NautilusAllocate: borsh::ser::BorshSerialize + NautilusAccount {

    fn allocate<'a>(
        &self, 
        self_account: AccountInfo<'a>,
        payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {
        
        invoke_signed(
            &system_instruction::allocate(
                &self_account.key,
                self.size()?,
            ),
            &[
                self_account.clone(),
                payer.clone(),
                system_program.clone(),
            ],
            self.seeds(),
        )
    }
}

pub trait NautilusCreate: borsh::ser::BorshSerialize + NautilusAccount {

    fn create<'a>(
        &self, 
        program_id: &'a Pubkey,
        self_account: AccountInfo<'a>,
        payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {

        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &self_account.key,
                self.lamports_required()?,
                self.size()?,
                program_id,
            ),
            &[
                self_account.clone(),
                payer.clone(),
                system_program.clone(),
            ],
            self.seeds(),
        )?;
        self.serialize(
            &mut &mut self_account.data.borrow_mut()[..]
        )?;
        Ok(())
    }
}

pub trait NautilusUpdate: borsh::ser::BorshSerialize + NautilusAccount {

    fn update<'a>(
        &self, 
        self_account: AccountInfo<'a>,
        payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {

        self.serialize(
            &mut &mut self_account.data.borrow_mut()[..]
        )?;
        Ok(())
    }
}

pub trait NautilusDelete: borsh::ser::BorshSerialize + NautilusAccount {

    fn delete<'a>(
        &self, 
        self_account: AccountInfo<'a>,
        payer: AccountInfo<'a>,
        system_program: AccountInfo<'a>,
    ) -> ProgramResult {
        
        let dest_starting_lamports = payer.lamports();
        **payer.lamports.borrow_mut() =
            dest_starting_lamports.checked_add(self_account.lamports()).unwrap();
        **self_account.lamports.borrow_mut() = 0;
        self_account.assign(&system_program::ID);
        self_account.realloc(0, false).map_err(Into::into)
    }
}