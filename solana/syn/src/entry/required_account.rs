use convert_case::{Case, Casing};

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
    Account(RequiredAccountSubtype),
    FeePayer,
    Sysvar,
    SystemProgram,
    Program,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequiredAccountSubtype {
    SelfAccount,
    Metadata,
    MintAuthority,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    Table(Vec<Construct>),
    Wallet,
    Token(bool),
    Mint(bool),
    Metadata(bool),
    AssociatedTokenAccount(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Construct {
    Index(bool),
    SelfAccount(String, String, bool, bool),
    Metadata(String, String, bool, bool),
    MintAuthority(String, String, bool, bool),
    FeePayer,
    Sysvar(SysvarType),
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    TokenMetadataProgram,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SysvarType {
    Clock,
    EpochSchedule,
    Rent,
}

impl From<Construct> for RequiredAccount {
    fn from(value: Construct) -> Self {
        match value {
            Construct::Index(is_mut) => {
                let name = "index".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut,
                    is_signer: false,
                    desc: "The Nautilus Index for this program".to_string(),
                    account_type: RequiredAccountType::IndexAccount,
                }
            }
            Construct::SelfAccount(name, desc, is_mut, is_pda) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: is_mut && !is_pda,
                    desc,
                    account_type: RequiredAccountType::Account(RequiredAccountSubtype::SelfAccount),
                }
            }
            Construct::Metadata(name, desc, is_mut, is_pda) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: is_mut && !is_pda,
                    desc,
                    account_type: RequiredAccountType::Account(RequiredAccountSubtype::Metadata),
                }
            }
            Construct::MintAuthority(name, desc, is_mut, is_pda) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: is_mut && !is_pda,
                    desc,
                    account_type: RequiredAccountType::Account(
                        RequiredAccountSubtype::MintAuthority,
                    ),
                }
            }
            Construct::FeePayer => {
                let name = "feePayer".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: true,
                    is_signer: true,
                    desc: "The transaction fee payer".to_string(),
                    account_type: RequiredAccountType::FeePayer,
                }
            }
            Construct::Sysvar(sysvar_type) => {
                let name = match sysvar_type {
                    SysvarType::Clock => "clock".to_string(),
                    SysvarType::EpochSchedule => "epochSchedule".to_string(),
                    SysvarType::Rent => "rent".to_string(),
                };
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name: name.clone(),
                    is_mut: false,
                    is_signer: false,
                    desc: format!("The Sysvar: {}", &(name.to_case(Case::Title))).to_string(),
                    account_type: RequiredAccountType::Sysvar,
                }
            }
            Construct::SystemProgram => {
                let name = "systemProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The System Program".to_string(),
                    account_type: RequiredAccountType::SystemProgram,
                }
            }
            Construct::TokenProgram => {
                let name = "tokenProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Program".to_string(),
                    account_type: RequiredAccountType::TokenProgram,
                }
            }
            Construct::AssociatedTokenProgram => {
                let name = "associatedTokenProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Associated Token Program".to_string(),
                    account_type: RequiredAccountType::AssociatedTokenProgram,
                }
            }
            Construct::TokenMetadataProgram => {
                let name = "tokenMetadataProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
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
            ObjectType::AssociatedTokenAccount(true)
        } else {
            ObjectType::Table(vec![]) // TODO: PDA authorities not supported yet
        }
    }

    pub fn resolve_accounts(
        obj_name: String,
        object_type: ObjectType,
        is_create: bool,
        is_signer: bool,
    ) -> (Vec<Self>, Option<Vec<Self>>) {
        let read = match object_type {
            ObjectType::Table(_) => {
                vec![Construct::SelfAccount(
                    obj_name.clone(),
                    obj_name,
                    is_create || is_signer,
                    true,
                )
                .into()]
            }
            ObjectType::Wallet => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_create || is_signer, false)
                    .into(),
                Construct::SystemProgram.into(),
            ],
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "Metadata";
                // let mint_authority_name = obj_name.clone() + "MintAuthority";
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_create || is_signer,
                        is_pda,
                    )
                    .into(),
                    Construct::Metadata(
                        metadata_name.clone(),
                        metadata_name,
                        is_create || is_signer,
                        is_pda,
                    )
                    .into(),
                    // Construct::MintAuthority(
                    //     mint_authority_name.clone(),
                    //     mint_authority_name,
                    //     is_create || is_signer,
                    //     is_pda,
                    // )
                    // .into(),
                    Construct::TokenProgram.into(),
                    Construct::TokenMetadataProgram.into(),
                ]
            }
            ObjectType::Mint(is_pda) => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_create || is_signer, is_pda)
                    .into(),
                Construct::TokenProgram.into(),
            ],
            ObjectType::Metadata(is_pda) => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_create || is_signer, is_pda)
                    .into(),
                Construct::TokenMetadataProgram.into(),
            ],
            ObjectType::AssociatedTokenAccount(is_pda) => {
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_create || is_signer,
                        is_pda,
                    )
                    .into(),
                    Construct::TokenProgram.into(),
                    Construct::AssociatedTokenProgram.into(),
                ]
            }
        };
        (
            read,
            match is_create {
                true => Some(vec![
                    Construct::FeePayer.into(),
                    Construct::Sysvar(SysvarType::Rent).into(),
                    Construct::SystemProgram.into(),
                ]),
                false => None,
            },
        )
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
        match &ast.account_type {
            RequiredAccountType::IndexAccount => quote::quote! { index: index.to_owned() },
            RequiredAccountType::Account(subtype) => match subtype {
                RequiredAccountSubtype::SelfAccount => {
                    let ident = self_account_ident(&ast.ident);
                    quote::quote! { account_info: #ident.to_owned() }
                }
                RequiredAccountSubtype::Metadata => {
                    let ident = metadata_ident(&ast.ident);
                    quote::quote! { metadata: #ident.to_owned() }
                }
                RequiredAccountSubtype::MintAuthority => {
                    let ident = mint_authority_ident(&ast.ident);
                    quote::quote! { mint_authority: #ident.to_owned() }
                }
            },
            RequiredAccountType::FeePayer
            | RequiredAccountType::Sysvar
            | RequiredAccountType::SystemProgram
            | RequiredAccountType::Program
            | RequiredAccountType::TokenProgram
            | RequiredAccountType::AssociatedTokenProgram
            | RequiredAccountType::TokenMetadataProgram => {
                let ident = &ast.ident;
                quote::quote! { #ident: #ident.to_owned() }
            }
        }
    }
}

fn appended_ident(ident: &syn::Ident, to_append: &str) -> syn::Ident {
    syn::Ident::new(
        &(ident.to_string() + to_append),
        proc_macro2::Span::call_site(),
    )
}

pub fn self_account_ident(ident: &syn::Ident) -> syn::Ident {
    appended_ident(ident, "_self_account")
}

pub fn metadata_ident(ident: &syn::Ident) -> syn::Ident {
    appended_ident(ident, "_metadata_account")
}

pub fn mint_authority_ident(ident: &syn::Ident) -> syn::Ident {
    appended_ident(ident, "_mint_authority")
}

pub fn name_to_ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::call_site())
}

pub fn name_to_ident_snake(name: &str) -> syn::Ident {
    use case::CaseExt;

    syn::Ident::new(
        &(name.to_string().to_snake()),
        proc_macro2::Span::call_site(),
    )
}
