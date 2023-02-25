#[test]
fn can_parse() {
    use nautilus::*;

    nautilus! {
        MyInstructions {
            CreatePerson,
            UpdatePerson,
            DeletePerson,
            #[instruction(custom_instruction)]
            CustomInstruction(CustomInstructionArgs),
        }
    }

    #[derive(NautilusAccount)]
    struct Person {
        #[primary_key(autoincrement = false)]
        id: u8,
        name: String,
        #[authority]
        authority: Pubkey,
        #[authority]
        signer: Pubkey,
    }

    struct CustomInstructionAccounts<'a> {
        payer: AccountInfo<'a>,
    }

    struct CustomInstructionArgs {
        name: String,
    }

    fn custom_instruction(accounts: CustomInstructionAccounts, args: CustomInstructionArgs) {}
}
