#[derive(Clone, Debug, PartialEq)]
pub struct RequiredAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    Pda(Vec<Defaults>),
    Wallet,
    Token(bool),
    Mint(bool),
    Metadata(bool),
    AssociatedTokenAccount(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Defaults {
    Index(bool),
    SelfAccount(String, String, bool, bool),
    Metadata(String, String, bool, bool),
    MintAuthority(String, String, bool, bool),
    FeePayer,
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
}

impl Defaults {
    pub fn resolve(self) -> RequiredAccount {
        match self {
            Defaults::Index(is_mut) => RequiredAccount {
                name: "index".to_string(),
                is_mut,
                is_signer: false,
                desc: "The Nautilus Index for this program".to_string(),
            },
            Defaults::SelfAccount(name, desc, is_mut, is_pda) => RequiredAccount {
                name,
                is_mut,
                is_signer: (is_mut && !is_pda),
                desc,
            },
            Defaults::Metadata(name, desc, is_mut, is_pda) => RequiredAccount {
                name,
                is_mut,
                is_signer: (is_mut && !is_pda),
                desc,
            },
            Defaults::MintAuthority(name, desc, is_mut, is_pda) => RequiredAccount {
                name,
                is_mut,
                is_signer: (is_mut && !is_pda),
                desc,
            },
            Defaults::FeePayer => RequiredAccount {
                name: "feePayer".to_string(),
                is_mut: true,
                is_signer: true,
                desc: "The transaction fee payer".to_string(),
            },
            Defaults::SystemProgram => RequiredAccount {
                name: "systemProgram".to_string(),
                is_mut: false,
                is_signer: false,
                desc: "The System Program".to_string(),
            },
            Defaults::TokenProgram => RequiredAccount {
                name: "tokenProgram".to_string(),
                is_mut: false,
                is_signer: false,
                desc: "The Token Program".to_string(),
            },
            Defaults::AssociatedTokenProgram => RequiredAccount {
                name: "associatedTokenProgram".to_string(),
                is_mut: false,
                is_signer: false,
                desc: "The Associated Token Program".to_string(),
            },
            Defaults::TokenMetadataProgram => RequiredAccount {
                name: "tokenMetadataProgram".to_string(),
                is_mut: false,
                is_signer: false,
                desc: "The Token Metadata Program".to_string(),
            },
        }
    }
}

// TODO: Add support for custom descriptions
//
impl RequiredAccount {
    pub fn get_source_nautilus_structs() -> Vec<String> {
        vec![
            "Wallet".to_string(),
            "Token".to_string(),
            "Mint".to_string(),
            "Metadata".to_string(),
            "AssociatedTokenAccount".to_string(),
        ]
    }

    pub fn derive_object_type(ty_name: &str) -> ObjectType {
        if ty_name.eq("Wallet") {
            ObjectType::Wallet
        } else if ty_name.eq("Token") {
            ObjectType::Token(false) // TODO: PDA Tokens not supported yet
        } else if ty_name.eq("Mint") {
            ObjectType::Mint(false) // TODO: PDA Tokens not supported yet
        } else if ty_name.eq("Metadata") {
            ObjectType::Metadata(false) // TODO: PDA Tokens not supported yet
        } else if ty_name.eq("AssociatedTokenAccount") {
            ObjectType::AssociatedTokenAccount(false) // TODO: PDA Tokens not supported yet
        } else {
            ObjectType::Pda(vec![]) // TODO: PDA authorities not supported yet
        }
    }

    pub fn resolve_for_read(obj_name: String, obj_type: ObjectType) -> Vec<Self> {
        let is_mut = false;
        match obj_type {
            ObjectType::Pda(_) => {
                vec![Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, true).resolve()]
            }
            ObjectType::Wallet => {
                vec![Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, false).resolve()]
            }
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "Metadata";
                let mint_authority_name = obj_name.clone() + "MintAuthority";
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                    Defaults::Metadata(metadata_name.clone(), metadata_name, is_mut, is_pda)
                        .resolve(),
                    Defaults::MintAuthority(
                        mint_authority_name.clone(),
                        mint_authority_name,
                        is_mut,
                        is_pda,
                    )
                    .resolve(),
                ]
            }
            ObjectType::Mint(is_pda) => {
                vec![Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve()]
            }
            ObjectType::Metadata(is_pda) => {
                vec![Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve()]
            }
            ObjectType::AssociatedTokenAccount(is_pda) => {
                vec![Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve()]
            }
        }
    }

    pub fn resolve_for_create(obj_name: String, obj_type: ObjectType) -> Vec<Self> {
        let is_mut = true;
        match obj_type {
            ObjectType::Pda(authorities) => {
                let mut accounts = vec![
                    Defaults::Index(is_mut).resolve(),
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, true).resolve(),
                ];
                authorities
                    .into_iter()
                    .for_each(|authority_type| accounts.push(authority_type.resolve()));
                accounts.extend(vec![
                    Defaults::FeePayer.resolve(),
                    Defaults::SystemProgram.resolve(),
                ]);
                accounts
            }
            ObjectType::Wallet => vec![
                Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, false).resolve(),
                Defaults::FeePayer.resolve(),
                Defaults::SystemProgram.resolve(),
            ],
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "Metadata";
                let mint_authority_name = obj_name.clone() + "MintAuthority";
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                    Defaults::Metadata(metadata_name.clone(), metadata_name, is_mut, is_pda)
                        .resolve(),
                    Defaults::MintAuthority(
                        mint_authority_name.clone(),
                        mint_authority_name,
                        is_mut,
                        is_pda,
                    )
                    .resolve(),
                    Defaults::FeePayer.resolve(),
                    Defaults::SystemProgram.resolve(),
                    Defaults::TokenProgram.resolve(),
                    Defaults::TokenMetadataProgram.resolve(),
                ]
            }
            ObjectType::Mint(is_pda) => vec![
                Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                Defaults::FeePayer.resolve(),
                Defaults::SystemProgram.resolve(),
                Defaults::TokenProgram.resolve(),
            ],
            ObjectType::Metadata(is_pda) => vec![
                Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                Defaults::FeePayer.resolve(),
                Defaults::SystemProgram.resolve(),
                Defaults::TokenMetadataProgram.resolve(),
            ],
            ObjectType::AssociatedTokenAccount(is_pda) => vec![
                Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                Defaults::FeePayer.resolve(),
                Defaults::SystemProgram.resolve(),
                Defaults::AssociatedTokenProgram.resolve(),
            ],
        }
    }

    pub fn condense(all_required_accounts: Vec<Vec<Self>>) -> Vec<Self> {
        let flattened = all_required_accounts
            .into_iter()
            .flat_map(|v| v.into_iter());
        let mut map = std::collections::HashMap::new();
        for account in flattened {
            let entry = map.entry(account.name.clone()).or_insert(account.clone());
            entry.is_mut |= account.is_mut;
            entry.is_signer |= account.is_signer;
            entry.desc = account.desc;
        }
        map.into_iter().map(|(_, v)| v).collect()
    }

    pub fn to_idl_instruction_account(&self) -> nautilus_idl::IdlInstructionAccount {
        nautilus_idl::IdlInstructionAccount {
            name: self.name.clone(),
            is_mut: self.is_mut,
            is_signer: self.is_signer,
            desc: self.desc.clone(),
        }
    }
}
