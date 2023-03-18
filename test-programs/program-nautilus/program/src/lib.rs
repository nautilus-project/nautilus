use nautilus::*;

#[nautilus]
mod program_nautilus {

    fn wallet_transfer_test<'a>(from: Wallet<'a>, to: JoeIsADarryl, amount: u64) -> ProgramResult {
        from.transfer_lamports(to, amount)
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Nautilus)]
pub struct JoeIsADarryl {}
