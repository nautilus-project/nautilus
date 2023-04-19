use nautilus::*;

// #[nautilus]
// pub mod my_mod {
//     fn create_wallet(new_wallet: Create<Wallet>) -> ProgramResult {
//         new_wallet.create()
//     }
//     fn create_wallet_with_payer(
//         new_wallet: Create<Wallet>,
//         rent_payer: Signer<Wallet>,
//     ) -> ProgramResult {
//         new_wallet.create_with_payer(rent_payer)
//     }
// }

pub enum NautilusEntrypoint {
    CreateWallet(),
    CreateWalletWithPayer(),
}
impl borsh::de::BorshDeserialize for NautilusEntrypoint {
    fn deserialize(buf: &mut &[u8]) -> core::result::Result<Self, borsh::maybestd::io::Error> {
        let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
        let return_value = match variant_idx {
            0u8 => NautilusEntrypoint::CreateWallet(),
            1u8 => NautilusEntrypoint::CreateWalletWithPayer(),
            _ => {
                let msg = {
                    let res = ::alloc::fmt::format(format_args!(
                        "Unexpected variant index: {0:?}",
                        variant_idx
                    ));
                    res
                };
                return Err(borsh::maybestd::io::Error::new(
                    borsh::maybestd::io::ErrorKind::InvalidInput,
                    msg,
                ));
            }
        };
        Ok(return_value)
    }
}
impl borsh::ser::BorshSerialize for NautilusEntrypoint {
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> core::result::Result<(), borsh::maybestd::io::Error> {
        let variant_idx: u8 = match self {
            NautilusEntrypoint::CreateWallet(..) => 0u8,
            NautilusEntrypoint::CreateWalletWithPayer(..) => 1u8,
        };
        writer.write_all(&variant_idx.to_le_bytes())?;
        match self {
            NautilusEntrypoint::CreateWallet() => {}
            NautilusEntrypoint::CreateWalletWithPayer() => {}
        }
        Ok(())
    }
}
fn create_wallet<'a>(mut new_wallet: Create<Wallet>) -> ProgramResult {
    new_wallet.create()
}
fn create_wallet_with_payer<'a>(
    mut new_wallet: Create<Wallet>,
    rent_payer: Signer<Wallet>,
) -> ProgramResult {
    new_wallet.create_with_payer(rent_payer)
}
pub fn process_instruction<'a>(
    program_id: &'static Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = NautilusEntrypoint::try_from_slice(input)?;
    match instruction {
        NautilusEntrypoint::CreateWallet() => {
            // ::solana_program::log::sol_log(&{
            //     let res = ::alloc::fmt::format(format_args!("Instruction: {0}", "CreateWallet"));
            //     res
            // });
            let accounts_iter = &mut accounts.iter();
            let new_wallet_self_account = next_account_info(accounts_iter)?;
            let fee_payer = next_account_info(accounts_iter)?;
            let rent = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let mut new_wallet = Create::new(
                Box::new(fee_payer.to_owned()),
                Box::new(system_program.to_owned()),
                Box::new(rent.to_owned()),
                Wallet::new(
                    Box::new(new_wallet_self_account.to_owned()),
                    Box::new(system_program.to_owned()),
                    false,
                ),
            );
            create_wallet(new_wallet)
        }
        NautilusEntrypoint::CreateWalletWithPayer() => {
            // ::solana_program::log::sol_log(&{
            //     let res =
            //         ::alloc::fmt::format(format_args!("Instruction: {0}", "CreateWalletWithPayer"));
            //     res
            // });
            let accounts_iter = &mut accounts.iter();
            let new_wallet_self_account = next_account_info(accounts_iter)?;
            let rent_payer_self_account = next_account_info(accounts_iter)?;
            let fee_payer = next_account_info(accounts_iter)?;
            let rent = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let mut new_wallet = Create::new(
                Box::new(fee_payer.to_owned()),
                Box::new(system_program.to_owned()),
                Box::new(rent.to_owned()),
                Wallet::new(
                    Box::new(new_wallet_self_account.to_owned()),
                    Box::new(system_program.to_owned()),
                    false,
                ),
            );
            let rent_payer = Signer::new(Wallet::new(
                Box::new(rent_payer_self_account.to_owned()),
                Box::new(system_program.to_owned()),
                true,
            ));
            create_wallet_with_payer(new_wallet, rent_payer)
        }
    }
}

#[test]
fn entry_create_wallet() {}
