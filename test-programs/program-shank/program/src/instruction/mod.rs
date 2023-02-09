pub mod create;
pub mod delete;
pub mod update;

pub use create::*;
pub use delete::*;
pub use update::*;

use {
    borsh::{
        BorshDeserialize, 
        BorshSerialize,
    },
    shank::ShankInstruction,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankInstruction)]
pub enum MyInstruction {
    
    #[account(0, writable, name="person_account",
              desc="The account that will represent the Person being created")]
    #[account(1, writable, name="payer",
            desc = "Fee payer")]
    #[account(2, name="system_program",
            desc = "The System Program")]
    CreatePerson(CreatePersonArgs),

    #[account(0, name="person_account",
              desc="The account representing the Person being deleted")]
    #[account(1, writable, name="payer",
            desc = "Fee payer & one who will receive the claimed rent")]
    DeletePerson,

    #[account(0, name="person_account",
              desc="The account representing the Person being updated")]
    #[account(1, writable, name="payer",
            desc = "Fee payer")]
    UpdatePerson(UpdatePersonArgs),
}