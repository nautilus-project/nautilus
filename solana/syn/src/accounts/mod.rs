mod accounts_borsh;
mod accounts_crud;
mod accounts_shank;

use quote::{
    ToTokens,
    quote,
};
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{
    Attribute,
    Data,
    DataStruct,
    DeriveInput,
    Fields,
    Ident,
    Generics,
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
        Err(lookahead.error())
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
        
        let name = &ast.ident;
        let attrs = &ast.attrs;
        
        println!("  -- Name: {}", name);
        println!("  -- Attrs: {:?}", attrs);
        println!("  -- Attrs len: {}", attrs.len());

        let borsh_impl = accounts_borsh::borsh_impl(name);
        let crud_impl = accounts_crud::crud_impl(name);
        let shank_impl = accounts_shank::shank_impl(name);
        
        quote! {
            #borsh_impl
            #crud_impl
            #shank_impl
        }.into()
    }
}