mod entrypoint_borsh;
mod entrypoint_processor;
mod entrypoint_shank;

use quote::{
    ToTokens,
    quote,
};
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{
    Attribute,
    Data,
    DataEnum,
    DeriveInput,
    Ident,
    Generics,
    Result,
    Token,
    Variant,
    Visibility,
    WhereClause, 
    braced,
    parse::{ Parse, ParseStream }, 
    punctuated::Punctuated,
    token,
};

// The NautilusEntrypointEnum is the syn struct we'll use to implement parsing & token stream methods
//
// Basically, it's used to facilitate parsing of the struct tokens read in by the derive macro,
//      and turn them into the necessary tokens to implement the program's entrypoint
//
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusEntrypointEnum {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}

// Parses the struct tokens and creates the NautilusEntrypointEnum struct
//
impl Parse for NautilusEntrypointEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![enum]) {
            let enum_token = input.parse::<Token![enum]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, brace, variants) = data_enum(input)?;
            Ok(DeriveInput {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: Data::Enum(DataEnum {
                    enum_token,
                    brace_token: brace,
                    variants,
                }),
            }.into())
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn data_enum(
    input: ParseStream,
) -> Result<(
    Option<WhereClause>,
    token::Brace,
    Punctuated<Variant, Token![,]>,
)> {
    let where_clause = input.parse()?;

    let content;
    let brace = braced!(content in input);
    let variants = content.parse_terminated(Variant::parse)?;

    Ok((where_clause, brace, variants))
}

// Allows us to, in the above parsing method, convert the parsed tokens into the NautilusEntrypointEnum
//
impl From<DeriveInput> for NautilusEntrypointEnum {
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

// Allows us to convert the input into a token stream from the parsed NautilusEntrypointEnum
//      using the below business logic
//
impl ToTokens for NautilusEntrypointEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

// Allows us to convert the built-out NautilusEntrypointEnum struct into the tokens we need
//
// Basically, this is where all the magic happens
//
impl From<&NautilusEntrypointEnum> for TokenStream {
    fn from(ast: &NautilusEntrypointEnum) -> Self {
        
        let name = &ast.ident;
        let bname = format!("{}NautilusEntrypointEnum", name);
        let bident = syn::Ident::new(&bname, name.span());
        
        println!("  -- Name: {}", name);
        println!("  -- bName: {}", bname);
        println!("  -- bIdent: {}", bident);

        let borsh_impl = entrypoint_borsh::borsh_impl(name);
        let processor = entrypoint_processor::processor(name);
        let shank_impl = entrypoint_shank::shank_impl(name);

        quote! {
            #borsh_impl
            #processor
            #shank_impl
        }.into()
    }
}