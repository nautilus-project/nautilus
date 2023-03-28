#[derive(Clone, Debug, PartialEq)]
pub struct RequiredAccount {
    pub ident: syn::Ident,
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: String,
    pub account_type: RequiredAccountType,
    pub object_type: ObjectType,
    pub construct: Construct,
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
    Pda(Vec<Construct>),
    Wallet,
    Token(bool),
    Mint(bool),
    Metadata(bool),
    AssociatedTokenAccount(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Construct {
    Index(bool, ObjectType),
    SelfAccount(String, String, bool, bool, ObjectType),
    Metadata(String, String, bool, bool, ObjectType),
    MintAuthority(String, String, bool, bool, ObjectType),
    FeePayer(ObjectType),
    SystemProgram(ObjectType),
    TokenProgram(ObjectType),
    AssociatedTokenProgram(ObjectType),
    TokenMetadataProgram(ObjectType),
}

impl Construct {
    pub fn resolve(self) -> RequiredAccount {
        let construct = self.clone();
        match self {
            Construct::Index(is_mut, object_type) => {
                let name = "index".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut,
                    is_signer: false,
                    desc: "The Nautilus Index for this program".to_string(),
                    account_type: RequiredAccountType::IndexAccount,
                    object_type,
                    construct,
                }
            }
            Construct::SelfAccount(name, desc, is_mut, is_pda, object_type) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                    object_type,
                    construct,
                }
            }
            Construct::Metadata(name, desc, is_mut, is_pda, object_type) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                    object_type,
                    construct,
                }
            }
            Construct::MintAuthority(name, desc, is_mut, is_pda, object_type) => {
                let ident = name_to_ident_snake(&name);
                RequiredAccount {
                    ident,
                    name,
                    is_mut,
                    is_signer: (is_mut && !is_pda),
                    desc,
                    account_type: RequiredAccountType::Account,
                    object_type,
                    construct,
                }
            }
            Construct::FeePayer(object_type) => {
                let name = "feePayer".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: true,
                    is_signer: true,
                    desc: "The transaction fee payer".to_string(),
                    account_type: RequiredAccountType::SystemAccount,
                    object_type,
                    construct,
                }
            }
            Construct::SystemProgram(object_type) => {
                let name = "systemProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The System Program".to_string(),
                    account_type: RequiredAccountType::SystemProgram,
                    object_type,
                    construct,
                }
            }
            Construct::TokenProgram(object_type) => {
                let name = "tokenProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Program".to_string(),
                    account_type: RequiredAccountType::TokenProgram,
                    object_type,
                    construct,
                }
            }
            Construct::AssociatedTokenProgram(object_type) => {
                let name = "associatedTokenProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Associated Token Program".to_string(),
                    account_type: RequiredAccountType::AssociatedTokenProgram,
                    object_type,
                    construct,
                }
            }
            Construct::TokenMetadataProgram(object_type) => {
                let name = "tokenMetadataProgram".to_string();
                RequiredAccount {
                    ident: name_to_ident_snake(&name),
                    name,
                    is_mut: false,
                    is_signer: false,
                    desc: "The Token Metadata Program".to_string(),
                    account_type: RequiredAccountType::TokenMetadataProgram,
                    object_type,
                    construct,
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

    pub fn resolve_for_read(obj_name: String, object_type: ObjectType) -> Vec<Self> {
        let is_mut = false;
        match object_type {
            ObjectType::Pda(_) => {
                vec![
                    Construct::SelfAccount(obj_name.clone(), obj_name, is_mut, true, object_type)
                        .resolve(),
                ]
            }
            ObjectType::Wallet => {
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        false,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::SystemProgram(object_type).resolve(),
                ]
            }
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "Metadata";
                // let mint_authority_name = obj_name.clone() + "MintAuthority";
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::Metadata(
                        metadata_name.clone(),
                        metadata_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    // Construct::MintAuthority(
                    //     mint_authority_name.clone(),
                    //     mint_authority_name,
                    //     is_mut,
                    //     is_pda,
                    //     object_type.clone(),
                    // )
                    // .resolve(),
                    Construct::TokenProgram(object_type.clone()).resolve(),
                    Construct::TokenMetadataProgram(object_type).resolve(),
                ]
            }
            ObjectType::Mint(is_pda) => {
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::TokenProgram(object_type).resolve(),
                ]
            }
            ObjectType::Metadata(is_pda) => {
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::TokenMetadataProgram(object_type).resolve(),
                ]
            }
            ObjectType::AssociatedTokenAccount(is_pda) => {
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::TokenProgram(object_type.clone()).resolve(),
                    Construct::AssociatedTokenProgram(object_type).resolve(),
                ]
            }
        }
    }

    pub fn resolve_for_create(obj_name: String, object_type: ObjectType) -> Vec<Self> {
        let is_mut = true;
        match object_type {
            ObjectType::Pda(authorities) => {
                let this_object_type = ObjectType::Pda(vec![]);
                let mut accounts = vec![
                    Construct::Index(is_mut, this_object_type.clone()).resolve(),
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        true,
                        this_object_type.clone(),
                    )
                    .resolve(),
                ];
                authorities
                    .into_iter()
                    .for_each(|authority_type| accounts.push(authority_type.resolve()));
                accounts.extend(vec![
                    Construct::FeePayer(this_object_type.clone()).resolve(),
                    Construct::SystemProgram(this_object_type).resolve(),
                ]);
                accounts
            }
            ObjectType::Wallet => vec![
                Construct::SelfAccount(
                    obj_name.clone(),
                    obj_name,
                    is_mut,
                    false,
                    object_type.clone(),
                )
                .resolve(),
                Construct::FeePayer(object_type.clone()).resolve(),
                Construct::SystemProgram(object_type).resolve(),
            ],
            ObjectType::Token(is_pda) => {
                let metadata_name = obj_name.clone() + "Metadata";
                let mint_authority_name = obj_name.clone() + "MintAuthority";
                vec![
                    Construct::SelfAccount(
                        obj_name.clone(),
                        obj_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::Metadata(
                        metadata_name.clone(),
                        metadata_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::MintAuthority(
                        mint_authority_name.clone(),
                        mint_authority_name,
                        is_mut,
                        is_pda,
                        object_type.clone(),
                    )
                    .resolve(),
                    Construct::FeePayer(object_type.clone()).resolve(),
                    Construct::SystemProgram(object_type.clone()).resolve(),
                    Construct::TokenProgram(object_type.clone()).resolve(),
                    Construct::TokenMetadataProgram(object_type).resolve(),
                ]
            }
            ObjectType::Mint(is_pda) => vec![
                Construct::SelfAccount(
                    obj_name.clone(),
                    obj_name,
                    is_mut,
                    is_pda,
                    object_type.clone(),
                )
                .resolve(),
                Construct::FeePayer(object_type.clone()).resolve(),
                Construct::SystemProgram(object_type.clone()).resolve(),
                Construct::TokenProgram(object_type).resolve(),
            ],
            ObjectType::Metadata(is_pda) => vec![
                Construct::SelfAccount(
                    obj_name.clone(),
                    obj_name,
                    is_mut,
                    is_pda,
                    object_type.clone(),
                )
                .resolve(),
                Construct::FeePayer(object_type.clone()).resolve(),
                Construct::SystemProgram(object_type.clone()).resolve(),
                Construct::TokenMetadataProgram(object_type).resolve(),
            ],
            ObjectType::AssociatedTokenAccount(is_pda) => vec![
                Construct::SelfAccount(
                    obj_name.clone(),
                    obj_name,
                    is_mut,
                    is_pda,
                    object_type.clone(),
                )
                .resolve(),
                Construct::FeePayer(object_type.clone()).resolve(),
                Construct::SystemProgram(object_type.clone()).resolve(),
                Construct::AssociatedTokenProgram(object_type).resolve(),
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
            RequiredAccountType::IndexAccount => quote::quote! { index: index.to_owned() },
            RequiredAccountType::Account => match ast.construct {
                Construct::SelfAccount(..) => {
                    let ident = self_account_ident(&ast.ident);
                    match ast.object_type {
                        ObjectType::Pda(_)
                        | ObjectType::Wallet
                        | ObjectType::Mint(_)
                        | ObjectType::Metadata(_)
                        | ObjectType::AssociatedTokenAccount(_) => {
                            quote::quote! { account_info: #ident.to_owned() }
                        }
                        ObjectType::Token(_) => {
                            quote::quote! { mint: #ident.to_owned() }
                        }
                    }
                }
                Construct::Metadata(..) => {
                    let ident = metadata_ident(&ast.ident);
                    quote::quote! { metadata: #ident.to_owned() }
                }
                Construct::MintAuthority(..) => {
                    let ident = mint_authority_ident(&ast.ident);
                    quote::quote! { mint_authority: #ident.to_owned() }
                }
                _ => {
                    panic!("An unexpected error occured while resolving entrypoint account tokens.")
                }
            },
            RequiredAccountType::SystemAccount => {
                let ident = self_account_ident(&ast.ident);
                quote::quote! { account_info: #ident.to_owned() }
            }
            RequiredAccountType::Sysvar => {
                let ident = self_account_ident(&ast.ident);
                quote::quote! { #ident: #ident.to_owned() }
            }
            RequiredAccountType::SystemProgram => {
                quote::quote! { system_program: system_program.to_owned() }
            }
            RequiredAccountType::Program => {
                let ident = self_account_ident(&ast.ident);
                quote::quote! { #ident: #ident.to_owned() }
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
