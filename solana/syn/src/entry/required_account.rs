//! The module for determining accounts required for a Nautilus object.
use case::CaseExt;
use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

/// The details of a required account for a Nautilus object.
#[derive(Clone, Debug, PartialEq)]
pub struct RequiredAccount {
    pub ident: Ident,
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
    pub account_type: RequiredAccountType,
}

/// The type of account required.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequiredAccountType {
    ProgramId,
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

/// The type of Nautilus object
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectType {
    NautilusIndex,
    Wallet,
    Token(bool),
    Mint(bool),
    Metadata,
    AssociatedTokenAccount,
    Record(bool, Vec<Construct>),
}

/// A construct shell enum used to map variants with provided args into required accounts.
#[derive(Clone, Debug, PartialEq)]
pub enum Construct {
    ProgramId,
    Index(bool),
    SelfAccount(String, String, bool, bool),
    Metadata(String, String, bool),
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
    /// Converts a Construct into the proper RequiredAccount using the provided variant arguments.
    fn from(value: Construct) -> Self {
        match value {
            Construct::ProgramId => {
                let name = "program_id".to_string();
                RequiredAccount {
                    ident: Ident::new(&name, Span::call_site()),
                    name: name.clone(),
                    is_mut: false,
                    is_signer: false,
                    desc: name,
                    account_type: RequiredAccountType::ProgramId,
                }
            }
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
            Construct::SelfAccount(name, desc, is_mut, is_signer) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer,
                    desc,
                    account_type: RequiredAccountType::Account(RequiredAccountSubtype::SelfAccount),
                }
            }
            Construct::Metadata(name, desc, is_mut) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: false,
                    desc,
                    account_type: RequiredAccountType::Account(RequiredAccountSubtype::Metadata),
                }
            }
            Construct::MintAuthority(name, desc, is_mut, is_signer) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer,
                    desc,
                    account_type: RequiredAccountType::Account(
                        RequiredAccountSubtype::MintAuthority,
                    ),
                }
            }
            Construct::FeePayer => {
                let account_type = RequiredAccountType::FeePayer;
                let name = account_type.to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: true,
                    is_signer: true,
                    desc: "The transaction fee payer".to_string(),
                    account_type,
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
                let account_type = RequiredAccountType::SystemProgram;
                let name = account_type.to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The System Program".to_string(),
                    account_type,
                }
            }
            Construct::TokenProgram => {
                let account_type = RequiredAccountType::TokenProgram;
                let name = account_type.to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Program".to_string(),
                    account_type,
                }
            }
            Construct::AssociatedTokenProgram => {
                let account_type = RequiredAccountType::AssociatedTokenProgram;
                let name = account_type.to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Associated Token Program".to_string(),
                    account_type,
                }
            }
            Construct::TokenMetadataProgram => {
                let account_type = RequiredAccountType::TokenMetadataProgram;
                let name = account_type.to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Metadata Program".to_string(),
                    account_type,
                }
            }
        }
    }
}

// TODO: Add support for custom descriptions
//
impl RequiredAccount {
    pub fn derive_object_type(ty_name: &str, is_mut: bool) -> ObjectType {
        if ty_name.eq("NautilusIndex") {
            ObjectType::NautilusIndex
        } else if ty_name.eq("Wallet") {
            ObjectType::Wallet
        } else if ty_name.eq("Token") {
            ObjectType::Token(false) // TODO: PDA Tokens not supported yet
        } else if ty_name.eq("Mint") {
            ObjectType::Mint(false) // TODO: PDA Tokens not supported yet
        } else if ty_name.eq("Metadata") {
            ObjectType::Metadata
        } else if ty_name.eq("AssociatedTokenAccount") {
            ObjectType::AssociatedTokenAccount
        } else {
            ObjectType::Record(is_mut, vec![]) // TODO: PDA authorities not supported yet
        }
    }

    /// Resolves the required accounts for an object name and ObjectType.
    /// The object name, as declared in the user's function signature, is used to append as a
    /// prefix to certain accounts where necessary.
    pub fn resolve_accounts(
        obj_name: String,
        object_type: ObjectType,
        is_create: bool,
        is_signer: bool,
        is_mut: bool,
    ) -> (Vec<Self>, Option<Vec<Self>>) {
        let read = match object_type {
            ObjectType::NautilusIndex => {
                vec![
                    Construct::ProgramId.into(),
                    Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, false).into(),
                ]
            }
            ObjectType::Wallet => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, is_signer).into(),
                Construct::SystemProgram.into(),
            ],
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "_metadata";
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name.clone(),
                        is_mut,
                        is_signer && !is_pda,
                    )
                    .into(),
                    Construct::Metadata(
                        metadata_name.clone(),
                        format!("Metadata account for: {}", obj_name),
                        is_mut,
                    )
                    .into(),
                    Construct::TokenProgram.into(),
                    Construct::TokenMetadataProgram.into(),
                ]
            }
            ObjectType::Mint(is_pda) => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, is_signer && !is_pda)
                    .into(),
                Construct::TokenProgram.into(),
            ],
            ObjectType::Metadata => vec![
                Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, false).into(),
                Construct::TokenMetadataProgram.into(),
            ],
            ObjectType::AssociatedTokenAccount => {
                vec![
                    Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, false).into(),
                    Construct::TokenProgram.into(),
                    Construct::AssociatedTokenProgram.into(),
                ]
            }
            ObjectType::Record(is_mut, _) => {
                vec![
                    Construct::ProgramId.into(),
                    Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, false).into(),
                    Construct::Index(is_mut).into(),
                ]
            }
        };
        (
            read,
            match is_create {
                true => Some(vec![
                    Construct::FeePayer.into(),
                    Construct::SystemProgram.into(),
                    Construct::Sysvar(SysvarType::Rent).into(),
                ]),
                false => None,
            },
        )
    }

    /// De-duplication of required accounts. Used to aggregate all accounts required for an instruction.
    pub fn condense(all_required_accounts: Vec<Vec<Self>>) -> Vec<Self> {
        let flattened = all_required_accounts
            .into_iter()
            .flat_map(|v| v.into_iter())
            .filter(|r| match r.account_type {
                RequiredAccountType::ProgramId => false,
                _ => true,
            });
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
}

impl From<&RequiredAccount> for proc_macro2::TokenStream {
    /// Converts a required account into the tokens used to instantiate a Nautilus object.
    /// Each required account for a Nautilus object can use `Into<TokenStream>` to generate the
    /// cloning of the `Box` pointers in the processor match arm, to be passed into the object's initializer.
    fn from(ast: &RequiredAccount) -> Self {
        match &ast.account_type {
            RequiredAccountType::ProgramId => quote! { program_id },
            RequiredAccountType::IndexAccount => quote! { nautilus_index.clone() },
            RequiredAccountType::Account(subtype) => match subtype {
                RequiredAccountSubtype::SelfAccount => {
                    let ident_pointer = self_account_ident_pointer(&ast.ident);
                    quote! { #ident_pointer.clone() }
                }
                RequiredAccountSubtype::Metadata => {
                    let ident_pointer = metadata_ident_pointer(&ast.ident);
                    quote! { #ident_pointer.clone() }
                }
                RequiredAccountSubtype::MintAuthority => {
                    let ident_pointer = mint_authority_ident_pointer(&ast.ident);
                    quote! { #ident_pointer.clone() }
                }
            },
            _ => {
                let ident_pointer = to_ident_pointer(&ast.ident);
                quote! { #ident_pointer.clone() }
            }
        }
    }
}

impl ToString for RequiredAccountType {
    fn to_string(&self) -> String {
        match self {
            RequiredAccountType::IndexAccount => "index".to_string(),
            RequiredAccountType::FeePayer => "feePayer".to_string(),
            RequiredAccountType::Sysvar => "sysvar".to_string(),
            RequiredAccountType::SystemProgram => "systemProgram".to_string(),
            RequiredAccountType::Program => "program".to_string(),
            RequiredAccountType::TokenProgram => "tokenProgram".to_string(),
            RequiredAccountType::AssociatedTokenProgram => "associatedTokenProgram".to_string(),
            RequiredAccountType::TokenMetadataProgram => "tokenMetadataProgram".to_string(),
            _ => "account".to_string(),
        }
    }
}

pub fn appended_ident(ident: &Ident, to_append: &str) -> Ident {
    Ident::new(&(ident.to_string() + to_append), Span::call_site())
}

pub fn self_account_ident(ident: &Ident) -> Ident {
    appended_ident(ident, "_self_account")
}

pub fn metadata_ident(ident: &Ident) -> Ident {
    appended_ident(ident, "_metadata_account")
}

pub fn mint_authority_ident(ident: &Ident) -> Ident {
    appended_ident(ident, "_mint_authority")
}

pub fn to_ident_pointer(ident: &Ident) -> Ident {
    appended_ident(ident, "_pointer")
}

pub fn self_account_ident_pointer(ident: &Ident) -> Ident {
    appended_ident(ident, "_self_account_pointer")
}

pub fn metadata_ident_pointer(ident: &Ident) -> Ident {
    appended_ident(ident, "_metadata_account_pointer")
}

pub fn mint_authority_ident_pointer(ident: &Ident) -> Ident {
    appended_ident(ident, "_mint_authority_pointer")
}

pub fn name_to_ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

pub fn name_to_ident_snake(name: &str) -> Ident {
    Ident::new(&(name.to_string().to_snake()), Span::call_site())
}
