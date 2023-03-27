#[derive(Clone, Debug, PartialEq)]
pub struct RequiredAccount {
    pub ident: syn::Ident,
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
    pub account_type: RequiredAccountType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequiredAccountType {
    IndexAccount,
    Account,
    SystemAccount,
    Sysvar,
    SystemProgram,
    Program,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
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
            Defaults::Index(is_mut) => {
                let name = "index".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut,
                    is_signer: false,
                    desc: "The Nautilus Index for this program".to_string(),
                    account_type: RequiredAccountType::IndexAccount,
                }
            }
            Defaults::SelfAccount(name, desc, is_mut, is_pda) => {
                let ident = crate::util::name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                }
            }
            Defaults::Metadata(name, desc, is_mut, is_pda) => {
                let ident = crate::util::name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                }
            }
            Defaults::MintAuthority(name, desc, is_mut, is_pda) => {
                let ident = crate::util::name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                }
            }
            Defaults::FeePayer => {
                let name = "feePayer".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut: true,
                    is_signer: true,
                    desc: "The transaction fee payer".to_string(),
                    account_type: RequiredAccountType::SystemAccount,
                }
            }
            Defaults::SystemProgram => {
                let name = "systemProgram".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The System Program".to_string(),
                    account_type: RequiredAccountType::SystemProgram,
                }
            }
            Defaults::TokenProgram => {
                let name = "tokenProgram".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Program".to_string(),
                    account_type: RequiredAccountType::TokenProgram,
                }
            }
            Defaults::AssociatedTokenProgram => {
                let name = "associatedTokenProgram".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Associated Token Program".to_string(),
                    account_type: RequiredAccountType::AssociatedTokenProgram,
                }
            }
            Defaults::TokenMetadataProgram => {
                let name = "tokenMetadataProgram".to_string();
                RequiredAccount {
                    ident: crate::util::name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Metadata Program".to_string(),
                    account_type: RequiredAccountType::TokenMetadataProgram,
                }
            }
        }
    }
}

// TODO: Add support for custom descriptions
//
impl RequiredAccount {
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
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, false).resolve(),
                    Defaults::SystemProgram.resolve(),
                ]
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
                    Defaults::TokenProgram.resolve(),
                    Defaults::TokenMetadataProgram.resolve(),
                ]
            }
            ObjectType::Mint(is_pda) => {
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                    Defaults::TokenProgram.resolve(),
                ]
            }
            ObjectType::Metadata(is_pda) => {
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                    Defaults::TokenMetadataProgram.resolve(),
                ]
            }
            ObjectType::AssociatedTokenAccount(is_pda) => {
                vec![
                    Defaults::SelfAccount(obj_name.clone(), obj_name, is_mut, is_pda).resolve(),
                    Defaults::AssociatedTokenProgram.resolve(),
                ]
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
        let mut res: Vec<RequiredAccount> = map.into_iter().map(|(_, v)| v).collect();
        res.sort_by(|a, b| {
            let account_type_cmp = a.account_type.cmp(&b.account_type);
            if account_type_cmp == std::cmp::Ordering::Equal {
                a.name.cmp(&b.name)
            } else {
                account_type_cmp
            }
        });
        res
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

impl From<&RequiredAccount> for proc_macro2::TokenStream {
    fn from(ast: &RequiredAccount) -> Self {
        match ast.account_type {
            RequiredAccountType::Account
            | RequiredAccountType::Program
            | RequiredAccountType::SystemAccount
            | RequiredAccountType::Sysvar => {
                let ident = crate::util::self_account_ident(&ast.ident);
                quote::quote! { account_info: #ident.to_owned() }
            }
            RequiredAccountType::IndexAccount => quote::quote! { index: index.to_owned() },
            RequiredAccountType::SystemProgram => {
                quote::quote! { system_program: system_program.to_owned() }
            }
            RequiredAccountType::TokenProgram => {
                quote::quote! { token_program: token_program.to_owned() }
            }
            RequiredAccountType::AssociatedTokenProgram => {
                quote::quote! { associated_token_program: associated_token_program.to_owned() }
            }
            RequiredAccountType::TokenMetadataProgram => {
                quote::quote! { token_metadata_program: token_metadata_program.to_owned() }
            }
        }
    }
}
