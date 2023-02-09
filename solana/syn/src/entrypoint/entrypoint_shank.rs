use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;

pub fn shank_impl(enum_name: &Ident) -> TokenStream {
    quote! {
        impl #enum_name {

            fn test_shank() {
                println!("Test");
            }

        }
    }
}

// --------------------------------------------------------------------
// Sample code to be written:
//
// pub enum VaultInstruction {
//     /// Initialize a token vault, starts inactivate. Add tokens in subsequent instructions, then activate.
//     #[account(0, writable, name="fraction_mint",
//               desc="Initialized fractional share mint with 0 tokens in supply, authority on mint must be pda of program with seed [prefix, programid]")]
//     #[account(1, writable, name="redeem_treasury",
//             desc = "Initialized redeem treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
//     #[account(2, writable, name="fraction_treasury",
//             desc = "Initialized fraction treasury token account with 0 tokens in supply, owner of account must be pda of program like above")]
//     #[account(3, writable, name="vault",
//             desc = "Uninitialized vault account")]
//     #[account(4, name="authority",
//             desc = "Authority on the vault")]
//     #[account(5, name="pricing_lookup_address",
//             desc = "Pricing Lookup Address")]
//     #[account(6, name="token_program",
//             desc = "Token program")]
//     #[account(7, name="rent",
//             desc = "Rent sysvar")]
//     InitVault(InitVaultArgs),
// 
//     /// Activates the vault, distributing initial shares into the fraction treasury.
//     /// Tokens can no longer be removed in this state until Combination.
//     #[account(0, writable, name="vault", desc = "Initialized inactivated fractionalized token vault")]
//     #[account(1, writable, name="fraction_mint", desc = "Fraction mint")]
//     #[account(2, writable, name="fraction_treasury", desc = "Fraction treasury")]
//     #[account(3, name="fraction_mint_authority", desc = "Fraction mint authority for the program - seed of [PREFIX, program_id]")]
//     #[account(4, signer, name="vault_authority", desc = "Authority on the vault")]
//     #[account(5, name="token_program", desc = "Token program")]
//     ActivateVault(NumberOfShareArgs)
// }
