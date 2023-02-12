use quote::{
    ToTokens,
    quote,
};
use proc_macro2::{
    TokenStream,
};
use std::fmt::Debug;
use syn::{
    Attribute,
    Data,
    DataStruct,
    DeriveInput,
    Field,
    Fields,
    FieldsNamed,
    Ident,
    Generics,
    Lit,
    Meta,
    Result,
    Token,
    Visibility,
    WhereClause, 
    parse::{ Parse, ParseStream }, 
    token,
};

// The NautilusAccountStruct is the syn struct we'll use to implement parsing & token stream methods
//
// Basically, it's used to facilitate parsing of the struct tokens read in by the derive macro,
//      and turn them into the necessary tokens to implement all of the NautilusCrud traits
//
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusAccountStruct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}

// TODO: Is there a better way to do this impl?
//
// Parses the struct tokens and creates the NautilusAccountStruct struct
//
impl Parse for NautilusAccountStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            let struct_token = input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields, semi) = data_struct(input)?;
            Ok(DeriveInput {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: Data::Struct(DataStruct {
                    struct_token,
                    fields,
                    semi_token: semi,
                }),
            }.into())
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn data_struct(
    input: ParseStream,
) -> Result<(Option<WhereClause>, Fields, Option<Token![;]>)> {
    let mut lookahead = input.lookahead1();
    let mut where_clause = None;
    if lookahead.peek(Token![where]) {
        where_clause = Some(input.parse()?);
        lookahead = input.lookahead1();
    }

    if where_clause.is_none() && lookahead.peek(token::Paren) {
        let fields = input.parse()?;

        lookahead = input.lookahead1();
        if lookahead.peek(Token![where]) {
            where_clause = Some(input.parse()?);
            lookahead = input.lookahead1();
        }

        if lookahead.peek(Token![;]) {
            let semi = input.parse()?;
            Ok((where_clause, Fields::Unnamed(fields), Some(semi)))
        } else {
            Err(lookahead.error())
        }
    } else if lookahead.peek(token::Brace) {
        let fields = input.parse()?;
        Ok((where_clause, Fields::Named(fields), None))
    } else if lookahead.peek(Token![;]) {
        let semi = input.parse()?;
        Ok((where_clause, Fields::Unit, Some(semi)))
    } else {
        Err(lookahead.error()) // TODO: Can only be for structs
    }
}

// Allows us to, in the above parsing method, convert the parsed tokens into the NautilusAccountStruct
//
impl From<DeriveInput> for NautilusAccountStruct {
    fn from(value: DeriveInput) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            ident: value.ident,
            generics: value.generics,
            data: value.data,
        }
    }
}

// Allows us to convert the input into a token stream from the parsed NautilusAccountStruct
//      using the below business logic
//
impl ToTokens for NautilusAccountStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

// Allows us to convert the built-out NautilusAccountStruct struct into the tokens we need
//
// Basically, this is where all the magic happens
//
impl From<&NautilusAccountStruct> for TokenStream {
    fn from(ast: &NautilusAccountStruct) -> Self {

        let mut primary_key_ident_opt: Option<Ident> = None;
        let mut autoincrement_enabled: bool = true;
        let mut authority_idents: Vec<Ident> = vec![];
        
        let name = &ast.ident;

        let fields = if let Data::Struct(
            DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }
        ) = &ast.data {
            named
        } else {
            unimplemented!() // TODO: Can only be for structs
        };

        for field in fields {
            let parsed_attributes = parse_attributes(field);
            autoincrement_enabled = parsed_attributes.autoincrement;
            if parsed_attributes.primary_key {
                primary_key_ident_opt = Some(field.ident.clone().unwrap());
            }
            if parsed_attributes.authority {
                authority_idents.push(field.ident.clone().unwrap());
            }
        }
        
        // let optionized = fields.iter().map(|f| {
        //     let original_ty = f.ty.clone();
        //     let ty = Path
        //     Field {
        //         attrs: Vec::new(),
        //         vis: Visibility::Inherited,
        //         ident: f.ident,
        //         colon_token: f.colon_token,
        //         ty,
        //     }
        // });

        let primary_key_ident = match primary_key_ident_opt {
            Some(ident) => ident,
            None => todo!("Throw an error on None value"),
        };

        let seed_prefix_binding = name.to_string().to_lowercase();
        let seed_prefix = seed_prefix_binding.as_str();
        let primary_key_seed = build_primary_key_seed(&primary_key_ident);

        let authority_idents_len = authority_idents.len();
        let check_authorities_syntax = match authority_idents.len() == 0 {
            true => quote! { Ok(()) },
            false => {
                let mut authority_checks = TokenStream::new();
                for authority_ident in authority_idents {
                    authority_checks.extend(quote! {
                        if account.key.eq(&self.#authority_ident) { assert!(account.is_signer) } // TODO: Not a valid check
                    })
                }
                quote! {
                    for account in accounts {
                        #authority_checks
                    }
                    Ok(())
                }
            }
        };

        let gather_authorities_syntax = match authority_idents_len == 0 {
            true => quote! { vec![]; },
            false => {
                let mut authorities = TokenStream::new();
                for _ in 1..authority_idents_len {
                    authorities.extend(quote! { 
                        next_account_info(accounts_iter)?.to_owned(), 
                    })
                }
                quote! {
                    vec![
                        #authorities
                    ];
                }
            }
        };
 
        quote! {
            
            impl BorshDeserialize for #name {
                fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                    BorshDeserialize::deserialize(&mut &buf[..])
                }
            }
            impl BorshSerialize for #name {
                fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                    BorshSerialize::serialize(self, writer)
                }
            }

            impl NautilusAccountBase for #name {

                const AUTO_INCREMENT: bool = #autoincrement_enabled;

                fn seeds<'a>(&self) -> &[&'a [u8]] {
                    &[
                        #seed_prefix.as_bytes().as_ref(),
                        #primary_key_seed,
                    ]
                }

                fn seeds_with_bump<'a>(&self, program_id: &Pubkey) -> &[&'a [u8]] {
                    let bump = self.pda(program_id).1;
                    &[
                        #seed_prefix.as_bytes().as_ref(),
                        #primary_key_seed,
                        bump.to_le_bytes().as_ref(),
                    ]
                }

                fn pda(&self, program_id: &Pubkey) -> (Pubkey, u8) {
                    Pubkey::find_program_address(
                        self.seeds(),
                        program_id, 
                    )
                }

                fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
                    #check_authorities_syntax
                }

                fn parse_nautilus_create_args<'a>(
                    program_id: &'a Pubkey, 
                    accounts: &'a [AccountInfo<'a>], 
                    create_instruction_args: Self,
                ) -> Result<NautilusCreateArgs<'a, Self>, ProgramError> {

                    let accounts_iter = &mut accounts.iter();
                    let autoinc_account = match Self::AUTO_INCREMENT {
                        true => Some(next_account_info(accounts_iter)?.to_owned()),
                        false => None,
                    };
                    let new_account = next_account_info(accounts_iter)?.to_owned();
                    let authorities = #gather_authorities_syntax
                    let fee_payer = next_account_info(accounts_iter)?.to_owned();
                    let system_program = next_account_info(accounts_iter)?.to_owned();

                    Ok(NautilusCreateArgs { 
                        program_id, 
                        autoinc_account, 
                        new_account, 
                        authorities,
                        fee_payer, 
                        system_program, 
                        data: create_instruction_args, 
                    })
                }

                fn parse_nautilus_delete_args<'a>(
                    program_id: &'a Pubkey, 
                    accounts: &'a [AccountInfo<'a>], 
                ) -> Result<NautilusDeleteArgs<'a>, ProgramError> {

                    let accounts_iter = &mut accounts.iter();
                    let target_account = next_account_info(accounts_iter)?.to_owned();
                    let authorities = #gather_authorities_syntax
                    let fee_payer = next_account_info(accounts_iter)?.to_owned();
            
                    Ok(NautilusDeleteArgs { 
                        program_id, 
                        target_account, 
                        authorities, 
                        fee_payer, 
                    })
                }

                fn parse_nautilus_update_args<'a>(
                    program_id: &'a Pubkey, 
                    accounts: &'a [AccountInfo<'a>], 
                    update_data: Self,
                ) -> Result<NautilusUpdateArgs<'a, Self>, ProgramError> {

                    let accounts_iter = &mut accounts.iter();
                    let target_account = next_account_info(accounts_iter)?.to_owned();
                    let authorities = #gather_authorities_syntax
                    let fee_payer = next_account_info(accounts_iter)?.to_owned();
                    let system_program = next_account_info(accounts_iter)?.to_owned();
            
                    Ok(NautilusUpdateArgs { 
                        program_id, 
                        target_account, 
                        authorities,
                        fee_payer, 
                        system_program, 
                        update_data, 
                    })
                }
            }

        }.into()
    }
}

struct NautilusAccountFieldAttributes {
    primary_key: bool,
    autoincrement: bool,
    authority: bool,
}

fn parse_attributes(field: &Field) -> NautilusAccountFieldAttributes {
    let mut primary_key = false;
    let mut autoincrement = true;
    let mut authority = false;
    for attr in field.attrs.iter() {
        if attr.path.is_ident("primary_key") { primary_key = true } // TODO: Add type check on Primary Key
        if attr.path.is_ident("authority") { authority = true }
        if primary_key {
            match attr.parse_meta() {
                Ok(meta) => {
                    if let Meta::NameValue(m) = meta {
                        if m.path.is_ident("autoincrement") {
                            if let Lit::Bool(lit_bool) = &m.lit {
                                autoincrement = lit_bool.value;
                            }                    
                        }          
                    }
                },
                Err(_) => (),
            }
        }
    };
    NautilusAccountFieldAttributes {
        primary_key,
        autoincrement,
        authority,
    }
}

// TODO: Build seed derivation based on type
//
fn build_primary_key_seed(key: &Ident) -> TokenStream {
    quote! {
        self.#key.to_le_bytes().as_ref()
    }
}