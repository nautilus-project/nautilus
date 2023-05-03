use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr, Type,
};

/// Possible account seeds.
#[derive(Clone, Debug)]
pub enum Seed {
    Lit { value: String },
    Field { ident: Ident },
    Param { ident: Ident, ty: Type },
}

impl Parse for Seed {
    /// Parses a parsed token stream into a `Seed`.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            Ok(Seed::Lit { value: lit.value() })
        } else if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            if input.peek(syn::Token![:]) {
                input.parse::<syn::Token![:]>()?;
                let ty: Type = input.parse()?;
                Ok(Seed::Param { ident, ty })
            } else {
                Ok(Seed::Field { ident })
            }
        } else {
            Err(lookahead.error())
        }
    }
}

/// Vehicle for parsing a parsed token stream into a `Vec<Seed>`.
pub struct SeedParser {
    pub seeds: Vec<Seed>,
}

impl Parse for SeedParser {
    /// Parses a parsed token stream into a `Vec<Seed>`.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let seeds: Punctuated<Seed, Comma> = content.parse_terminated(Seed::parse)?;
        Ok(SeedParser {
            seeds: seeds.into_iter().collect(),
        })
    }
}
