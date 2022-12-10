use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};


#[derive(BorshDeserialize, BorshSerialize)]
pub enum PrestigeProtocolInstruction {
    CreateChallenge,
}


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = PrestigeProtocolInstruction::try_from_slice(&instruction_data)?;
    
    match instruction {

        PrestigeProtocolInstruction::CreateChallenge => {
            msg!("Prestige Protocol Instruction: CreateChallenge");
            return Ok(())
        }
    }
}