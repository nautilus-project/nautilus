use {
    borsh::{BorshDeserialize, BorshSerialize},
    shank::ShankAccount,
    solana_program::pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankAccount)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub authority: Pubkey,
}
