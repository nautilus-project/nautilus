#[derive(Clone, Debug, PartialEq)]
pub struct RequiredAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
}

impl RequiredAccount {
    pub fn get_default_nautilus_structs() -> Vec<(String, Vec<RequiredAccount>)> {
        vec![
            (
                String::from("Wallet"),
                vec![RequiredAccount {
                    name: String::from("self_account"),
                    is_mut: false,
                    is_signer: false,
                    desc: String::from("An account representing a Wallet"),
                }],
            ),
            (
                String::from("Token"),
                vec![
                    RequiredAccount {
                        name: String::from("self_account"),
                        is_mut: false,
                        is_signer: false,
                        desc: String::from("An account representing a Mint"),
                    },
                    RequiredAccount {
                        name: String::from("self_account"),
                        is_mut: false,
                        is_signer: false,
                        desc: String::from("An account representing a Metadata"),
                    },
                ],
            ),
            (
                String::from("Mint"),
                vec![RequiredAccount {
                    name: String::from("self_account"),
                    is_mut: false,
                    is_signer: false,
                    desc: String::from("An account representing a Mint"),
                }],
            ),
            (
                String::from("Metadata"),
                vec![RequiredAccount {
                    name: String::from("self_account"),
                    is_mut: false,
                    is_signer: false,
                    desc: String::from("An account representing a Metadata"),
                }],
            ),
            (
                String::from("AssociatedTokenAccount"),
                vec![RequiredAccount {
                    name: String::from("self_account"),
                    is_mut: false,
                    is_signer: false,
                    desc: String::from("An account representing an Associated Token Account"),
                }],
            ),
            (
                String::from("Darryl"),
                vec![RequiredAccount {
                    name: String::from("self_account"),
                    is_mut: false,
                    is_signer: false,
                    desc: String::from("An account representing a Darrylbeta"),
                }],
            ),
        ]
    }

    pub fn get_default_create_required_accounts() -> Vec<RequiredAccount> {
        vec![
            RequiredAccount {
                name: String::from("index"),
                is_mut: true,
                is_signer: false,
                desc: String::from("The Nautilus Index account for your program"),
            },
            RequiredAccount {
                name: String::from("system_program"),
                is_mut: false,
                is_signer: false,
                desc: String::from("The System Program"),
            },
        ]
    }

    pub fn condense_required_accounts(
        all_required_accounts: Vec<Vec<RequiredAccount>>,
    ) -> Vec<RequiredAccount> {
        all_required_accounts.concat()
        // .into_iter()
        // .fold(vec![], |mut res, r| {
        //     if let Some(added_account) = res.iter_mut().find(|x| x == &&r) {
        //         added_account.name = added_account.name.eq(r.name);
        //     } else {
        //         res.push(r);
        //     }
        //     res
        // })
    }
}
