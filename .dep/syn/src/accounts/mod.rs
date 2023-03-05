//
//
// ----------------------------------------------------------------
//                 Nautilus account token generation
// ----------------------------------------------------------------
//
//     TokenStream -> DataStruct
//         -> NautilusAccountStruct -> * -> TokenStream
//                                     * New tokens created here
//
//
mod borsh;
mod crud;
mod data;
mod error;
mod parser;
mod spawn;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NautilusAccountStruct {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: syn::Data,
}

impl syn::parse::Parse for NautilusAccountStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse::<syn::Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![struct]) {
            let struct_token = input.parse::<syn::Token![struct]>()?;
            let ident = input.parse::<syn::Ident>()?;
            let generics = input.parse::<syn::Generics>()?;
            let (where_clause, fields, semi) = data_struct(input)?;
            Ok(syn::DeriveInput {
                attrs,
                vis,
                ident,
                generics: syn::Generics {
                    where_clause,
                    ..generics
                },
                data: syn::Data::Struct(syn::DataStruct {
                    struct_token,
                    fields,
                    semi_token: semi,
                }),
            }
            .into())
        } else {
            Err(syn::Error::new(input.span(), error::EnforceStructsError()))
        }
    }
}

pub fn data_struct(
    input: syn::parse::ParseStream,
) -> syn::Result<(
    Option<syn::WhereClause>,
    syn::Fields,
    Option<syn::Token![;]>,
)> {
    let mut lookahead = input.lookahead1();
    let mut where_clause = None;
    if lookahead.peek(syn::Token![where]) {
        where_clause = Some(input.parse()?);
        lookahead = input.lookahead1();
    }

    if where_clause.is_none() && lookahead.peek(syn::token::Paren) {
        let fields = input.parse()?;

        lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![where]) {
            where_clause = Some(input.parse()?);
            lookahead = input.lookahead1();
        }

        if lookahead.peek(syn::Token![;]) {
            let semi = input.parse()?;
            Ok((where_clause, syn::Fields::Unnamed(fields), Some(semi)))
        } else {
            Err(lookahead.error())
        }
    } else if lookahead.peek(syn::token::Brace) {
        let fields = input.parse()?;
        Ok((where_clause, syn::Fields::Named(fields), None))
    } else if lookahead.peek(syn::Token![;]) {
        let semi = input.parse()?;
        Ok((where_clause, syn::Fields::Unit, Some(semi)))
    } else {
        Err(lookahead.error())
    }
}

impl From<syn::DeriveInput> for NautilusAccountStruct {
    fn from(value: syn::DeriveInput) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            ident: value.ident,
            generics: value.generics,
            data: value.data,
        }
    }
}

impl quote::ToTokens for NautilusAccountStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend::<proc_macro2::TokenStream>(self.into());
    }
}

//
//
// Code generation
//
//
impl From<&NautilusAccountStruct> for proc_macro2::TokenStream {
    fn from(ast: &NautilusAccountStruct) -> Self {
        spawn::SpawnNautilusAccount::from_ast(ast)
            .expect("Error parsing annotated tokens.")
            .generate()
            .expect("Error generating Nautilus tokens.")
    }
}
