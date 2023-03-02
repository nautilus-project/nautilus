use nautilus::*;
use shank::ShankInstruction;

use crate::custom_instruction::CustomArgs;
use crate::state::{Hero, Villain};

#[derive(BorshDeserialize, BorshSerialize, ShankInstruction)]
pub enum MyInstructions {
    #[account(
        0,
        writable,
        name = "autoinc_account",
        desc = "The autoincrement account."
    )]
    #[account(1, writable, name = "new_account", desc = "The account to be created.")]
    #[account(
        2,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(3, writable, signer, name = "fee_payer", desc = "Fee payer")]
    #[account(4, name = "system_program", desc = "The System Program")]
    CreateHero(Hero),

    #[account(
        0,
        writable,
        name = "target_account",
        desc = "The account to be deleted."
    )]
    #[account(
        1,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
    DeleteHero,

    #[account(
        0,
        writable,
        name = "target_account",
        desc = "The account to be updated."
    )]
    #[account(
        1,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
    #[account(3, name = "system_program", desc = "The System Program")]
    UpdateHero(Hero),

    #[account(
        0,
        writable,
        name = "autoinc_account",
        desc = "The autoincrement account."
    )]
    #[account(1, writable, name = "new_account", desc = "The account to be created.")]
    #[account(
        2,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(3, writable, signer, name = "fee_payer", desc = "Fee payer")]
    #[account(4, name = "system_program", desc = "The System Program")]
    CreateVillain(Villain),

    #[account(
        0,
        writable,
        name = "target_account",
        desc = "The account to be deleted."
    )]
    #[account(
        1,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
    DeleteVillain,

    #[account(
        0,
        writable,
        name = "target_account",
        desc = "The account to be updated."
    )]
    #[account(
        1,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
    #[account(3, name = "system_program", desc = "The System Program")]
    UpdateVillain(Villain),

    #[account(
        0,
        writable,
        name = "target_account",
        desc = "The account to be used as a test."
    )]
    #[account(
        1,
        writable,
        signer,
        name = "authority",
        desc = "One of the authorities specified for this account."
    )]
    #[account(2, writable, signer, name = "fee_payer", desc = "Fee payer")]
    #[account(3, name = "system_program", desc = "The System Program")]
    CustomInstruction(CustomArgs),
}
