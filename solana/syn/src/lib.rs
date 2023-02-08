use quote::{
    ToTokens,
    quote,
};
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{*, parse::ParseStream, parse::Parse, punctuated::Punctuated};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusAccount {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: Data,
}

impl From<DeriveInput> for NautilusAccount {
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

//
// ----------------------------------------------------------------------------------------------------------------
//
//  TODO: Re-write Parse impl
//      * Likely drop Enum & Union, and serve only structs ?
//

impl Parse for NautilusAccount {
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
        } else if lookahead.peek(Token![enum]) {
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
        } else if lookahead.peek(Token![union]) {
            let union_token = input.parse::<Token![union]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields) = data_union(input)?;
            Ok(DeriveInput {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: Data::Union(DataUnion {
                    union_token,
                    fields,
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

pub fn data_union(input: ParseStream) -> Result<(Option<WhereClause>, FieldsNamed)> {
    let where_clause = input.parse()?;
    let fields = input.parse()?;
    Ok((where_clause, fields))
}

//
// ----------------------------------------------------------------------------------------------------------------
//

impl ToTokens for NautilusAccount {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(self.into());
    }
}

//
// ----------------------------------------------------------------------------------------------------------------
//
//  Business Logic
//

impl From<&NautilusAccount> for TokenStream {
    fn from(ast: &NautilusAccount) -> Self {
        let name = &ast.ident;
        let bname = format!("{}NautilusAccount", name);
        let bident = syn::Ident::new(&bname, name.span());
        let expanded = quote! {
            struct #bident {
            }
            impl #name {
                fn builder() -> #bident {
                    #bident {
                    }
                }
            }
        };
        expanded.into()
    }
}