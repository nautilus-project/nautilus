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
    
        #[account(
                0, 
                writable, 
                name="autoinc_account",
                desc="The account for autoincrementing this table")
        ]
        #[account(
                1, 
                writable, 
                name="new_account",
                desc="The account that will represent the Person being created")
        ]
        #[account(
                2, 
                signer, 
                name="authority",
                desc = "Record authority")
        ]
        #[account(
                3, 
                writable,
                signer, 
                name="fee_payer",
                desc = "Fee payer")
        ]
        #[account(
                4, 
                name="system_program",
                desc = "The System Program")
        ]
        CreatePerson(CreatePersonArgs),

        #[account(
                0, 
                writable, 
                name="target_account",
                desc="The account that will represent the Person being deleted")
        ]
        #[account(
                1, 
                signer, 
                name="authority",
                desc = "Record authority")
        ]
        #[account(
                2, 
                writable,
                signer, 
                name="fee_payer",
                desc = "Fee payer")
        ]
        DeletePerson,

        #[account(
                0, 
                writable, 
                name="new_account",
                desc="The account that will represent the Person being updated")
        ]
        #[account(
                1, 
                signer, 
                name="authority",
                desc = "Record authority")
        ]
        #[account(
                2, 
                writable,
                signer, 
                name="fee_payer",
                desc = "Fee payer")
        ]
        #[account(
                3, 
                name="system_program",
                desc = "The System Program")
        ]
        UpdatePerson(UpdatePersonArgs),
}