// use solana_program::{msg, program_error::ProgramError};

// pub enum NautilusError {
//     AccountDataFailedToDeserialize(String, String),
//     AccountDataFailedToLoad(String, String),
// }

// impl NautilusError {
//     fn throw(&self) -> u32 {
//         match &self {
//             Self::AccountDataFailedToDeserialize(state_type, pubkey) => {
//                 msg!(
//                     "Failed to deserialize {} data from account {}",
//                     state_type,
//                     pubkey
//                 );
//                 msg!("Are you sure this is of type {}?", state_type);
//                 3001
//             }
//             Self::AccountDataFailedToLoad(state_type, pubkey) => {
//                 msg!(
//                     "Could not read data from account when attempting to load {} data: {}",
//                     state_type,
//                     pubkey
//                 );
//                 msg!("Is it empty?");
//                 3002
//             }
//         }
//     }
// }

// impl From<NautilusError> for ProgramError {
//     fn from(value: NautilusError) -> Self {
//         ProgramError::Custom(value.throw())
//     }
// }
